mod err;
mod winsz;
pub mod cursor;
pub mod control;

use std::ops::{BitOr, BitAnd, Add, Sub, Mul, Not};
use std::io::{self, Write};
use std::fmt;
use std::str;
use std::mem;

use ::libc;

pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};
use self::cursor::Cursor;
use self::control::Control;

pub type In = [libc::c_uchar; 4];

#[derive(Debug, Clone)]
pub struct Display {
    save_position: (libc::size_t, libc::size_t),
    save_terminal: Option<Box<Display>>,
    ss_mod: bool,
    newline: Vec<(libc::size_t, libc::size_t)>,
    ///Scroll_region set with \x1B[y1;y2r => region(y1, y2)
    region: (libc::size_t, libc::size_t),
    collection: Vec<libc::size_t>,
    oob: (libc::size_t, libc::size_t),
    line_wrap: bool,
    size: Winszed,
    screen: Cursor<Vec<Control>>,
    bell: libc::size_t,
}

impl Display {
    /// The constructor method `default` returns the `Display`'s interface
    /// from shell.
    pub fn new(fd: libc::c_int) -> Result<Display> {
        match Winszed::new(fd) {
          Err(why) => Err(DisplayError::WinszedFail(why)),
          Ok(wsz) => Ok(Display::from_winszed(wsz)),
        }
    }

    /// The constructor method `default` returns the `Display`'s interface
    /// from shell.
    pub fn from_winszed(size: Winszed) -> Display {
        Display {
            save_position: (0, 0),
            save_terminal: None,
            ss_mod: false,
            newline:
            { let mut end_row_newlines: Vec<(libc::size_t, libc::size_t)> = Vec::with_capacity(size.get_row());
              {0..size.ws_row}.all(|i|
              { end_row_newlines.push((size.get_col() - 1, i as usize));
                true });
              end_row_newlines },
            region: (0, size.get_row()),
            collection: Vec::new(),
            oob: (0, 0),
            line_wrap: true,
            size: size,
            bell: 0,
            screen: Cursor::new(
              (0..size.row_by_col()).map(|_: usize|
                                            Control::new(&[b' '][..])
                                        ).collect::<Vec<Control>>()
            ),
        }
    }

    /// The accessor `ss` returns the value of 'ss_mod'.
    pub fn ss(&self) -> bool
    { self.ss_mod }

    /// The accessor `get_cursor_coords` returns the value of 'oob', that is the coordinates of the cursor.
    pub fn get_cursor_coords(&self) -> (libc::size_t, libc::size_t)
    { self.oob }

    /// The accessor `newlines` returns the value of 'newline', that contains all newlines that are now
    /// displayed on the screen.
    pub fn newlines(&self) -> &Vec<(libc::size_t, libc::size_t)>
    { &self.newline }

    /// Converts a Vector of Control into a byte vector.
    pub fn into_bytes(&self) -> Vec<libc::c_uchar> {
        let mut screen: Vec<libc::c_uchar> = Vec::with_capacity(self.len());

        self.screen.get_ref().iter().all(|control: &Control| {
            let buf: &[u8] = control.get_ref();
            screen.extend_from_slice(buf);
            true
        });
        screen
    }

    /// The method `len` returns number of characters.
    pub fn len(&self) -> libc::size_t {
        mem::size_of::<In>().mul(&self.size.row_by_col())
    }

    /// The method `clear` purges the screen vector.
    pub fn clear(&mut self) -> io::Result<libc::size_t> {
        self.screen.get_mut().iter_mut().all(|mut term: &mut Control|
                                             term.clear().is_ok()
        );
        self.newline.clear();
        {0..self.size.ws_row}.all(|i|
        { self.newline.push((self.size.get_col() - 1, i as usize));
          true });
        Ok(0)
    }

    /// The method `resize` updates the size of the output screen.
    pub fn resize(&mut self) -> Result<()> {
    match Winszed::new(0) {
      Err(why) => Err(DisplayError::WinszedFail(why)),
      Ok(size) => 
        { if self.size.ws_row < size.ws_row
          { {self.size.ws_row..size.ws_row}.all(|i|
            { self.newline.push((self.size.get_col() - 1, i as usize));
              true });
            match self.size.get_col().checked_mul((size.ws_row - self.size.ws_row) as usize)
            { Some(get) =>
                { let mut vide = {0..get}.map(|_: usize| Control::new(&[b' '][..])).collect::<Vec<Control>>();
                  self.screen.get_mut().append(&mut vide); },
              None => {}, }}
          else if self.size.ws_row > size.ws_row
          { {size.ws_row..self.size.ws_row}.all(|i|
            { match self.newline.iter().position(|&a| a.1.eq(&(i as usize)))
              { Some(n) => { self.newline.remove(n); },
                None => {}, }
              true });
            let srow = size.ws_row as usize;
            if self.region.1 > srow
            { self.region.1 = srow; }
            if self.region.0 >= srow
            { match srow.checked_sub(1)
              { Some(get) => { self.region.0 = get; },
                None => { self.region.0 = 0; }, }}
            match self.size.get_col().checked_mul((self.size.ws_row - size.ws_row) as usize)
            { Some(get) =>
                { if self.oob.1.ge(&(size.ws_row as usize))
                  { let x = self.size.get_col() - 1;
                    self.goto_coord(x, (size.ws_row as usize) - 1); }
                  match self.size.row_by_col().checked_sub(get)
                  { Some(start) => { self.screen.get_mut().drain(start..); },
                    None => {}, }},
              None => {}, }}

          if self.size.ws_col < size.ws_col
          { let col = self.size.ws_col;
            let row = self.size.ws_row;
            let pos = self.screen.position();
            let y = self.oob.1 as u16;
            match row.checked_sub(y)
            { Some(h) =>
                { match size.ws_col.checked_sub(col)
                  { Some(k) =>
                      { match pos.checked_add(((h as usize).mul(k as usize)))
                        { Some(g) => { self.goto(g); }
                          None => {}, }},
                    None => {}, }},
              None => {}, }
          /*  {0..row}.all(|i|
            { match self.newline.iter().position(|&a| a.1.eq(&(i as usize)))
              { Some(n) => { self.newline[n].0 = (size.ws_col as usize) - 1; },
                None => {}, }
              true });*/
            let coucou = self.screen.get_mut();
            {0..row}.all(|i|
            { {0..size.ws_col-col}.all(|_|
              { (*coucou).insert(((row - i) * col) as usize, Control::new(&[b' '][..]));
                true }) }); }
          else if self.size.ws_col > size.ws_col
          { let col = self.size.ws_col;
            let row = self.size.ws_row;
            let pos = self.screen.position();
            let y = self.oob.1 as u16;
            match row.checked_sub(y)
            { Some(h) =>
                { match col.checked_sub(size.ws_col)
                  { Some(k) =>
                      { match pos.checked_sub(((h as usize).mul(k as usize)))
                        { Some(g) => { self.goto(g); }
                          None => {}, }},
                    None => {}, }},
              None => {}, }
           /* {0..row}.all(|i|
            { match self.newline.iter().position(|&a| a.1.eq(&(i as usize)))
              { Some(n) => { self.newline[n].0 = (size.ws_col as usize) - 1; },
                None => {}, }
              true });*/
            let coucou = self.screen.get_mut();
            {0..row}.all(|i|
            { {0..col-size.ws_col}.all(|k|
              { (*coucou).remove((((row - i) * col) - (k + 1)) as usize);
                true }) }); }
          Ok(self.size = size) },
      }
    }

    /// The method `tricky_resize` updates the size of the output screen.
    pub fn tricky_resize(&mut self, begin: libc::size_t, end: libc::size_t)
    { if begin > 0 && begin <= end
      { self.region = (begin - 1, end); }}

    /// The method `goto` moves the cursor position
    pub fn goto(&mut self, index: libc::size_t) -> io::Result<libc::size_t> {
        self.screen.set_position(index);
        Ok(0)
    }

    /// The method `goto_home` moves the cursor to the top left of the output screen.
    pub fn goto_home(&mut self) -> io::Result<libc::size_t>
    { self.goto(0);
      self.oob = (0, 0);
      Ok(0) }

    /// The method `goto_up` moves the cursor up.
    pub fn goto_up(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let col = self.size.get_col();
      let pos = self.screen.position();
      if self.oob.1 >= mv
      { self.goto(pos.sub(&((col.mul(&mv)))));
        self.oob.1 = self.oob.1.sub(&mv); }
      else
      { self.oob.1 = 0;
        let x = self.oob.0;
        self.goto_coord(x, 0); }
      Ok(0) }

    /// The method `goto_down` moves the cursor down.
    pub fn goto_down(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let row = self.size.get_row();
      let col = self.size.get_col();
      let pos = self.screen.position();
      if self.oob.1 + mv <= row - 1
      { self.goto(pos.add(&(col.mul(&mv))));
        self.oob.1 = self.oob.1.add(&mv); }
      else
      { self.oob.1 = row - 1;
        let x = self.oob.0;
        self.goto_coord(x, row - 1); }
      Ok(0) }

    /// The method `goto_right` moves the cursor to its right.
    pub fn goto_right(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let col = self.size.get_col();
      let pos = self.screen.position();
      if self.oob.0 + mv <= col - 1
      { self.goto(pos.add(&mv));
        self.oob.0 = self.oob.0.add(&mv); }
      else
      { self.goto_end_row(); }
      Ok(0) }

    pub fn goto_left(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { let col = self.size.get_col();
      let pos = self.screen.position();
      if self.oob.0 >= mv
      { self.goto(pos.sub(&mv));
        self.oob.0 = self.oob.0.sub(&mv); }
      else
      { self.goto_begin_row(); }
      Ok(0) }

    /// The method `goto_begin_row` moves the cursor to the beginning of the row
    pub fn goto_begin_row(&mut self)
    { let y = self.oob.1;
      self.goto_coord(0, y); }

    /// The method `goto_end_row` moves the cursor to the end of the row
    pub fn goto_end_row(&mut self)
    { let x = self.size.get_col() - 1;
      let y = self.oob.1;
      self.goto_coord(x, y); }

    /// The method `goto_coord` moves the cursor to the given coordinates
    pub fn goto_coord(&mut self, x: libc::size_t, y: libc::size_t)
    { let col = self.size.get_col();
      let row = self.size.get_row();
      let c;
      let r;
      if x < col
      { self.oob.0 = x;
        c = x; }
      else
      { self.oob.0 = col - 1;
        c = col - 1; }
      if y < row
      { self.oob.1 = y;
        r = y; }
      else
      { self.oob.1 = row - 1;
        r = row - 1; }
      self.goto(c + (r * col)); }

    /// The method `scroll_down` append an empty line on bottom of the screen
    /// (the cursor doesn't move)
    pub fn scroll_down(&mut self, base: libc::size_t)
    { let col = self.size.get_col();
      let resize = self.region;
      if !self.newline.is_empty()
      { match self.newline.iter().position(|&i| i.1.eq(&(resize.1 - 1)).bitand(i.1.eq(&(self.size.get_row() - 1)).not()))
        { Some(n) => { self.newline.remove(n); },
          None => {}, }
        self.newline.iter_mut().all(|mut a|
        { if a.1.ge(&resize.0).bitand(a.1.lt(&(resize.1 - 1)))
          { a.1 += 1; }
          true }); }
      self.newline.push((self.size.get_col() - 1, resize.0));
      self.newline.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
      self.newline.dedup();
      let coucou = self.screen.get_mut();
      {0..col}.all(|_|
      { (*coucou).insert(base * col, Control::new(&[b' '][..]));
        (*coucou).remove(resize.1 * col);
        true }); }

    /// The method `scroll_up` insert an empty line on top of the screen
    /// (the cursor doesn't move)
    pub fn scroll_up(&mut self, base: libc::size_t)
    { let col = self.size.get_col();
      let resize = self.region;
      if !self.newline.is_empty()
      { match self.newline.iter().position(|&i| i.1.eq(&base))
        { Some(n) => { self.newline.remove(n); },
          None => {}, }
        self.newline.iter_mut().all(|mut a|
        { if a.1.gt(&base).bitand(a.1.lt(&resize.1))
          { a.1 -= 1; }
          true }); }
      self.newline.push((self.size.get_col() - 1, resize.1 - 1));
      self.newline.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
      self.newline.dedup();
      let coucou = self.screen.get_mut();
      {0..col}.all(|_|
      { (*coucou).insert(resize.1 * col, Control::new(&[b' '][..]));
        (*coucou).remove(base * col);
        true }); }

    /// The method `save_position` save a position in the variable 'save_position' to get
    /// restored with self.restore_position() described right after.
    /// If save_position() is called many times, only the newest safe will be kept.
    pub fn save_position(&mut self)
    { self.save_position = (self.oob.0, self.oob.1); }

    /// The method `restore_position` move the cursor to coordinates safe
    /// with self.save_position() described right before.
    /// If no coordinates were safe, cursor moves to the top left of the output screen
    pub fn restore_position(&mut self)
    { let (x, y) = self.save_position;
      self.goto_coord(x, y); }

    /// The method `insert_empty_line` insert an empty line on the right of the cursor
    /// (the cursor doesn't move)
    pub fn insert_empty_line(&mut self, mv: libc::size_t)
    { let mut col = self.size.get_col();
      let region = self.region;
      let pos = self.screen.position();
      let oob = self.oob;
      let coucou = self.screen.get_mut();
      {0..(col * mv)}.all(|_|
      { (*coucou).insert(pos, Control::new(&[b' '][..]));
        { (*coucou).remove(region.1 * col); }
        true }); }

    /// The method `erase_right_line` erase the current line from the cursor
    /// to the next '\n' encountered
    /// (char under the cursor included)
    pub fn erase_right_line(&mut self, pos: libc::size_t)
    { let col = self.size.get_col();
      if !self.newline.is_empty()
      { match self.newline.iter().position(|&a| a.1.ge(&(pos/col)))
        { Some(n) => 
            { self.screen.get_mut().into_iter().skip(pos).take(self.newline[n].0 + (self.newline[n].1 * col) + 1 - pos).all(|mut term: &mut Control| { term.clear().is_ok() }); },
          None => 
            { self.erase_down(); }, }}
      else
      { self.erase_down(); }}

    /// The method `erase_left_line` erase the current line from the previous '\n'
    /// to the cursor
    /// (char under the cursor included)
    pub fn erase_left_line(&mut self, pos: libc::size_t)
    { let col = self.size.get_col();
      if !self.newline.is_empty()
      { self.newline.reverse();
        match self.newline.iter().position(|&a| a.1.lt(&(pos/col)))
        { Some(n) => 
            { self.screen.get_mut().into_iter().skip(self.newline[n].0 + (self.newline[n].1 * col)).take(pos - (self.newline[n].0 + (self.newline[n].1 * col)) + 1).all(|mut term: &mut Control| { term.clear().is_ok() }); },
          None => 
            { self.erase_up(); }, }
        self.newline.reverse(); }
      else
      { self.erase_up(); }}

    /// The method `erase_line` erase the entire current line
    pub fn erase_line(&mut self, mv: libc::size_t)
    { let col = self.size.get_col();
      let mut x = self.oob.0;
      let mut y = self.oob.1;
      {0..mv}.all(|_|
      { match self.newline.iter().position(|&a| a.1.ge(&y))
        { Some(n) =>
            { self.erase_left_line(x + (y * col));
              self.erase_right_line(x + (y * col));
              x = self.newline[n].0;
              y = self.newline[n].1 + 1;
              true },
          None => { self.erase_down() ; false }, }}); }

    /// The method `erase_up` erase all lines from the current line up to
    /// the top of the screen, and erase the current line from the left border
    /// column to the cursor.
    /// (char under the cursor included)
    pub fn erase_up(&mut self)
    { let pos = self.screen.position();
      self.screen.get_mut().into_iter().take(pos + 1).all(|mut term: &mut Control|
      { term.clear().is_ok() }); }

    /// The method `erase_down` erase all lines from the current line down to
    /// the bottom of the screen and erase the current line from the cursor to
    /// the right border column
    /// (char under the cursor included)
    pub fn erase_down(&mut self)
    { let pos = self.screen.position();
      let len = self.size.row_by_col();
      self.screen.get_mut().into_iter().skip(pos).take(len - pos + 1).all(|mut term: &mut Control|
      { term.clear().is_ok() }); }

    /// The method `print_enter` reproduce the behavior of a '\n'
    pub fn print_enter(&mut self)
    { let pos = self.screen.position();
      if self.oob.1.lt(&(self.region.1.sub(&1)))
      { self.goto_down(1); }
      else if self.oob.1.eq(&(self.region.1.sub(&1)))
      { let x = self.region.0;
        self.scroll_up(x); }}

    /// The method `print_char` print an unicode character (1 to 4 chars range)
    pub fn print_char(&mut self, first: &[u8], next: &[u8]) -> io::Result<usize>
    { let wrap = self.line_wrap;
      let row = self.size.get_row();
      let col = self.size.get_col();
      if self.oob.0 < col - 1
      { self.oob.0 += 1; }
      else if self.oob.1.lt(&self.region.1.sub(&1)).bitand(self.ss_mod.not())
      { if self.newline.is_empty().not().bitand(self.ss_mod.not())
        { match self.newline.iter().position(|&x| x.1.eq(&self.oob.1))
          { Some(n) => { self.newline.remove(n); },
            None => {}, }; }
        self.oob.1 += 1;
        self.oob.0 = 0; }
      else if self.oob.1.eq(&(self.region.1.sub(&1)))
      { let x = self.region.0;
        self.scroll_up(x);
        self.goto_begin_row();
        { let pos = self.screen.position();
          self.goto(pos - 1); }}
      else
      { let pos = self.screen.position();
        self.goto(pos - 1); }
      self.screen.write(first).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) )) }

    pub fn catch_numbers<'a>(&self, mut acc: Vec<libc::size_t>, buf: &'a [u8]) -> (Vec<libc::size_t>, &'a [u8])
    { match parse_number!(buf)
      { Some((number, &[b';', ref next..])) =>
          { acc.push(number);
            self.catch_numbers(acc, next) },
        Some((number, &[ref next..])) =>
          { acc.push(number);
            (acc, next) },
        _ =>
          { (acc, buf) }, }}

    /// The method `next_tab` return the size of the current printed tabulation
    pub fn next_tab(&self) -> libc::size_t
    { 8 - (self.oob.0 % 8) }

    /// The method `save_terminal` saves the terminal Display configuration.
    pub fn save_terminal(&mut self)
    { self.save_terminal = Some(Box::new(self.clone())); }

    /// The method `restore_terminal` restore the terminal Display configuration
    /// kept in the 'save_terminal' variable.
    pub fn restore_terminal(&mut self)
    { if let Some(save) = self.save_terminal.take()
      { *self = *save; }}

    /// The method `erase_chars` erases couple of chars in the current line from the cursor.
    pub fn erase_chars(&mut self, mv: libc::size_t)
    { let pos = self.screen.position();
      let border = match self.newline.iter().position(|&x| x.1.ge(&self.oob.1))
      { Some(n) => self.newline[n].0 + (self.newline[n].1 * self.size.get_col()) + 1,
        None => self.size.row_by_col() - 1, };
      let coucou = self.screen.get_mut();
      {0..mv}.all(|i|
      { (*coucou).insert(border, Control::new(&[b' '][..]));
        (*coucou).remove(pos);
        true }); }
}

impl IntoIterator for Display {
    type Item = Control;
    type IntoIter = ::std::vec::IntoIter<Control>;

    fn into_iter(self) -> Self::IntoIter {
        self.screen.into_iter()
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "{}", String::from_utf8_unchecked(self.into_bytes()))
        }
    }
}

impl Write for Display {
    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match buf {
            &[] => Ok(0),

            //---------- TERMINAL SAVE -----------
            &[b'\x1B', b'[', b'?', b'1', b'0', b'4', b'9', b'h', ref next..] =>
              { self.save_terminal();
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'4', b'9', b'l', ref next..] =>
              { self.restore_terminal();
                self.write(next) },

            //------------ SETTINGS -------------
            &[b'\x1B', b'c', ref next..] =>
              { self.write(next) },
            &[b'\x1B', b'[', b'>', b'0', b'c', ref next..] |
            &[b'\x1B', b'[', b'>', b'c', ref next..] =>
              { self.write(next) },
            &[b'\x1B', b'[', b'?', b'7', b'h', ref next..] |
            &[b'\x1B', b'[', b'2', b'0', b'h', ref next..] =>
              { self.line_wrap = true;
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'l', ref next..] |
            &[b'\x1B', b'[', b'2', b'0', b'l', ref next..] =>
              { self.line_wrap = false;
                self.write(next) },
            &[b'\x1B', b'[', b'r', ref next..] =>
              { self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'h', ref next..] =>
              { self.ss_mod = true;
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'l', ref next..] =>
              { self.ss_mod = false;
                self.write(next) },

            //------------ ERASE -----------------
            &[b'\x1B', b'[', b'K', ref next..] |
            &[b'\x1B', b'[', b'0', b'K', ref next..] =>
              { let pos = self.screen.position();
                self.erase_right_line(pos);
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'K', ref next..] =>
              { let pos = self.screen.position();
                self.erase_left_line(pos);
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'K', ref next..] =>
              { self.erase_line(1);
                self.write(next) },
            &[b'\x1B', b'[', b'J', ref next..] |
            &[b'\x1B', b'[', b'0', b'J', ref next..] =>
              { self.erase_down();
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'J', ref next..] =>
              { self.erase_up();
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'J', ref next..] => self.clear().and(self.write(next)),
            &[b'\x1B', b'[', b'P', ref next..] =>
              { self.erase_chars(1);
                self.write(next) },

            //------------ INSERT -----------------
            &[b'\x1B', b'[', b'L', ref next..] =>
              { if self.oob.1.ge(&self.region.0).bitand(self.oob.1.lt(&self.region.1))
                { self.insert_empty_line(1); }
                self.write(next) },

            //------------- GOTO ------------------
            &[b'\x1B', b'[', b';', b'H', ref next..] |
            &[b'\x1B', b'[', b';', b'f', ref next..] |
            &[b'\x1B', b'[', b'd', ref next..] |
            &[b'\x1B', b'[', b'H', ref next..] |
            &[b'\x1B', b'[', b'f', ref next..] =>
              { self.goto_home();
                self.write(next) },
            &[b'\x1B', b'[', b'A', ref next..] |
            &[b'\x1B', b'[', b'm', b'A', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'A', ref next..] =>
              { self.goto_up(1);
                self.write(next) },
            &[b'A', b'\x08'] =>
              { self.goto_up(1);
                Ok(0) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'\x1B', b'[', b'm', b'B', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
              { self.goto_down(1);
                self.write(next) },
            &[b'B', b'\x08'] =>
              { self.goto_down(1);
                Ok(0) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'\x1B', b'[', b'm', b'C', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
              { self.goto_right(1);
                self.write(next) },
            &[b'C', b'\x08'] =>
              { self.goto_right(1);
                Ok(0) },
            &[b'\x1B', b'[', b'D', ref next..] |
            &[b'\x1B', b'[', b'm', b'D', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'D', ref next..] |
            &[b'\x08', ref next..] =>
              { self.goto_left(1);
                self.write(next) },
            &[b'D', b'\x08'] =>
              { self.goto_left(1);
                Ok(0) },

            //--------- POSITION SAVE ----------
            &[b'\x1B', b'[', b's', ref next..] |
            &[b'\x1B', b'7', ref next..] =>
              { self.save_position();
                self.write(next) },
            &[b'\x1B', b'[', b'u', ref next..] |
            &[b'\x1B', b'8', ref next..] =>
              { self.restore_position();
                self.write(next) },

            //------------- SCROLL ---------------
            &[b'\x1B', b'M', ref next..] =>
              { if self.oob.1.ge(&self.region.0).bitand(self.oob.1.lt(&self.region.1))
                { let x = self.oob.1;
                  if !self.ss_mod
                  { self.scroll_up(x); }
                  else
                  { self.scroll_down(x); }}
                self.write(next) },
            &[b'\x1B', b'S', ref next..] =>
              { if self.oob.1.ge(&self.region.0).bitand(self.oob.1.lt(&self.region.1))
                { let x = self.region.0;
                  self.scroll_up(x); }
                self.write(next) },
           &[b'\x1B', b'L', ref next..] =>
             { if self.oob.1.ge(&self.region.0).bitand(self.oob.1.lt(&self.region.1))
               { let x = self.oob.1;
                 self.scroll_down(x); }
                self.write(next) },
           &[b'\x1B', b'T', ref next..] =>
             { if self.oob.1.ge(&self.region.0).bitand(self.oob.1.lt(&self.region.1))
               { let x = self.region.0;
                 self.scroll_down(x); }
                self.write(next) },

            //------------ CL ATTR -------------
            &[b'\x1B', b'[', b'0', b'm', ref next..] |
            &[b'\x1B', b'[', b'm', ref next..] =>
              { self.collection.clear();
                self.write(next) },

            &[b'\x1B', b'[', b'?', ref next..] |
            &[b'\x1B', b'[', b'>', ref next..] |
            &[b'\x1B', b'[', ref next..] |
            &[b'\x1B', b'(', ref next..] |
            &[b'\x1B', b'?', ref next..] |
            &[b'\x1B', ref next..] =>
            { let (mut bonjour, coucou) =
              { self.catch_numbers(Vec::new(), next) };
              match coucou
              { //------------- n GOTO ------------------
                &[b'A', ref next..] =>
                  { if bonjour.len() == 1
                    { self.goto_up(bonjour[0]); }
                    self.write(next) },
                &[b'B', ref next..] =>
                  { if bonjour.len() == 1
                    { self.goto_down(bonjour[0]); }
                    self.write(next) },
                &[b'C', ref next..] =>
                  { if bonjour.len() == 1
                    { self.goto_right(bonjour[0]); }
                    self.write(next) },
                &[b'D', ref next..] =>
                  { if bonjour.len() == 1
                    { self.goto_left(bonjour[0]); }
                    self.write(next) },
                &[b'G', ref next..] =>
                  { if bonjour.len() == 1 && self.size.get_col() >= bonjour[0]
                    { let y = self.oob.1;
                      self.goto_coord(bonjour[0] - 1, y); }
                    self.write(next) },
                &[b'd', ref next..] =>
                  { if bonjour.len() == 1 && self.size.get_row() >= bonjour[0]
                    { let x = self.oob.0;
                      self.goto_coord(x, bonjour[0] - 1); }
                    self.write(next) },
                &[b'H', ref next..] |
                &[b'f', ref next..] =>
                  { if bonjour.len() == 2 && bonjour[0] > 0 && bonjour[1] > 0
                    { self.goto_coord(bonjour[1] - 1, bonjour[0] - 1); }
                    self.write(next) },

                //-------------- ERASE ----------------
                &[b'P', ref next..] =>
                  { if bonjour.len() == 1
                    { self.erase_chars(bonjour[0]); }
                    self.write(next) },

                //-------------- SCROLL ----------------
                &[b'M', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.oob.1.ge(&self.region.0)).bitand(self.oob.1.lt(&self.region.1))
                    { let x = self.oob.1;
                      {0..bonjour[0]}.all(|_|
                      { if !self.ss_mod
                        { self.scroll_up(x); }
                        else
                        { self.scroll_down(x); }
                        true }); }
                    self.write(next) },
                &[b'S', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.oob.1.ge(&self.region.0)).bitand(self.oob.1.lt(&self.region.1))
                    { let x = self.region.0;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_up(x);
                        true }); }
                    self.write(next) },
                &[b'L', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.oob.1.ge(&self.region.0)).bitand(self.oob.1.lt(&self.region.1))
                    { let x = self.oob.1;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_down(x);
                        true }); }
                    self.write(next) },
                &[b'T', ref next..] =>
                  { if bonjour.len().eq(&1).bitand(self.oob.1.ge(&self.region.0)).bitand(self.oob.1.lt(&self.region.1))
                    { let x = self.region.0;
                      {0..bonjour[0]}.all(|_|
                      { self.scroll_down(x);
                        true }); }
                    self.write(next) },

                //------------- ATTRIBUTS ---------------
                &[b'm', ref next..] =>
                  { self.collection.append(&mut bonjour);
                    self.write(next) },

                //----------- TRICKY RESIZE -------------
                &[b'r', ref next..] =>
                  { if bonjour.len() == 2
                    { self.tricky_resize(bonjour[0], bonjour[1]); }
                    self.write(next) },

                //----------- TERM VERSION --------------
                &[b'c', ref next..] =>
                  { self.write(next) },
                &[b';', b'c', ref next..] =>
                  { self.write(next) },

                &[_, ref next..] |
                &[ref next..] =>
                  { self.write(next) }, }},

            &[b'\x07', ref next..] =>
              { self.bell += 1;
                self.write(next) },
            &[b'\x0A', b'\x0D', ref next..] |
            &[b'\x0A', ref next..] |
            &[b'\x0D', b'\x0A', ref next..] =>
              { self.print_enter();
                self.goto_begin_row();
                self.write(next) },
            &[b'\x0D', ref next..] =>
              { self.goto_begin_row();
                self.write(next) },

            &[b'\x09', ref next..] =>
            { let resize = self.region;
              let col = self.size.get_col();
              let tab_width = self.next_tab();
              let pos = self.screen.position();
              { let coucou = self.screen.get_mut();
                {0..tab_width}.all(|_|
                { (*coucou).insert(pos, Control::new(&[b' '][..]));
                  (*coucou).remove(resize.1 * col);
                  true }); }
              self.goto_right(tab_width);
              self.write(next) },

            &[u1 @ b'\xF0' ... b'\xF4', u2 @ b'\x8F' ... b'\x90', u3 @ b'\x80' ... b'\xBF', u4 @ b'\x80' ... b'\xBF', ref next..] =>
            { self.print_char(&[u1, u2, u3, u4], next) },
            &[u1 @ b'\xE0' ... b'\xF0', u2 @ b'\x90' ... b'\xA0', u3 @ b'\x80' ... b'\xBF', ref next..] =>
            { self.print_char(&[u1, u2, u3], next) },
            &[u1 @ b'\xC2' ... b'\xDF', u2 @ b'\x80' ... b'\xBF', ref next..] =>
            { self.print_char(&[u1, u2], next) },
            &[u1, ref next..] =>
            { self.print_char(&[u1], next) },
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
