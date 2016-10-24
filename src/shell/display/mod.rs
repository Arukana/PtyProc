mod err;
mod winsz;
pub mod cursor;
pub mod control;

use std::ops::{BitAnd, Add, Sub, Mul, Not};
use std::io::{self, Write};
use std::fmt;
use std::str;
use std::mem;

use ::libc;

pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};
use self::cursor::Cursor;
use self::control::Control;

pub type In = [libc::c_uchar; 16];

#[derive(Debug, Clone)]
pub struct Display {
    save_position: (libc::size_t, libc::size_t),
    save_terminal: Option<Box<Display>>,
    right_border: libc::size_t,
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
            right_border: 80,
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

    /// The method `clear` puges the screen vector.
    pub fn clear(&mut self) -> io::Result<libc::size_t> {
        self.goto(0).is_ok().bitand(
            self.screen.get_mut().iter_mut().all(|mut term: &mut Control|
                                                term.clear().is_ok()
            )
        );
        Ok(0)
    }

    /// The method `resize` updates the size of the output screen.
    pub fn resize(&mut self) -> Result<()> {
    match Winszed::new(0) {
        Err(why) => Err(DisplayError::WinszedFail(why)),
    Ok(size) => Ok(self.size = size),
      }
    }

    /// The method `tricky_resize` updates the size of the output screen.
    pub fn tricky_resize(&mut self, begin: libc::size_t, end: libc::size_t)
    { //println!("Resize::({}, {})", begin, end);
      if begin <= end
      { self.region = (begin - 1, end); }}

    /// The method `goto` moves the cursor position
    pub fn goto(&mut self, index: libc::size_t) -> io::Result<libc::size_t> {
        self.screen.set_position(index);
        Ok(0)
    }

    /// The method `goto_home` moves the cursor to the top left of the output screen.
    pub fn goto_home(&mut self) -> io::Result<libc::size_t>
    { println!("Goto::Home");
      self.goto(0);
      self.oob = (0, 0);
      Ok(0) }

    /// The method `goto_up` moves the cursor up.
    pub fn goto_up(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { println!("Goto::Up({})", mv);
      let col = self.size.get_col();
      let pos = self.screen.position();
      println!("   POS::{}, OOB::{:?}", pos, self.oob);
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
    { println!("Goto::Down({})", mv);
      let row = self.size.get_row();
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
    { println!("Goto::Right({})", mv);
      let col = self.size.get_col();
      let pos = self.screen.position();
      if self.oob.0 + mv <= col - 1
      { self.goto(pos.add(&mv));
        self.oob.0 = self.oob.0.add(&mv); }
      else
      { self.goto_end_line(); }
      Ok(0) }

    /// The method `goto_left` moves the cursor to its left.
    pub fn goto_left(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
    { print!("Goto::Left({})", mv);
      let col = self.size.get_col();
      let pos = self.screen.position();
      println!("   POS::{}, OOB::{:?}, COL::{}", pos, self.oob, col);
      if self.oob.0 >= mv
      { self.goto(pos.sub(&mv));
      println!("LEFT POS::{} or {}", pos.sub(&mv), self.screen.position());
        self.oob.0 = self.oob.0.sub(&mv); }
      else
      { self.goto_begin_line(); }
      Ok(0) }

    /// The method `goto_begin_line` moves the cursor to the beginning of the line
    pub fn goto_begin_line(&mut self)
    { let x = self.oob.0;
      self.goto_left(x); }

    /// The method `goto_end_line` moves the cursor to the end of the line
    pub fn goto_end_line(&mut self)
    { let x = self.size.get_col() - self.oob.0 - 1;
      self.goto_right(x); }

    /// The method `goto_coord` moves the cursor to the given coordinates
    pub fn goto_coord(&mut self, x: libc::size_t, y: libc::size_t)
    { //println!("Cursor::CursorGoto({}, {})", x, y);
      let col = self.size.get_col();
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

    /// The method `scroll_up` insert an empty line on top of the screen
    /// (the cursor doesn't move)
    pub fn scroll_up(&mut self)
    { //println!("Cursor::ScrollUp");
      let col = self.size.get_col();
      let resize = self.region;
      let coucou = self.screen.get_mut();
      {0..col}.all(|_|
      { (*coucou).insert(resize.0 * col, Control::new(&[b' '][..]));
        (*coucou).remove(resize.1 * col);
        true }); }

    /// The method `scroll_down` append an empty line on bottom of the screen
    /// (the cursor doesn't move)
    pub fn scroll_down(&mut self)
    { //println!("Cursor::ScrollDown");
      let col = self.size.get_col();
      let resize = self.region;
      println!("INSERT::{}, POS::{}, OOB::{:?}", col, self.screen.position(), self.oob);
     { let coucou = self.screen.get_mut();
      {0..col}.all(|_|
      { 
      println!("INSERT::{}, REMOVE::{}", resize.1 * col, resize.0 * col);

        (*coucou).insert(resize.1 * col, Control::new(&[b' '][..]));
        (*coucou).remove(resize.0 * col);
        true }); }
      println!("POS::{}, OOB::{:?}", self.screen.position(), self.oob);
      }

    /// The method `save_position` save a position in the variable 'save_position' to get
    /// restored with self.restore_position() described right after.
    /// If save_position() is called many times, only the newest safe will be kept.
    pub fn save_position(&mut self)
    { //println!("Cursor::SaveCursor");
      self.save_position = (self.oob.0, self.oob.1); }

    /// The method `restore_position` move the cursor to coordinates safe
    /// with self.save_position() described right before.
    /// If no coordinates were safe, cursor moves to the top left of the output screen
    pub fn restore_position(&mut self)
    { //println!("Cursor::RestoreCursor");
      let (x, y) = self.save_position;
      self.goto_coord(x, y); }

    /// The method `insert_empty_line` insert an empty line on the right of the cursor
    /// (the cursor doesn't move)
    pub fn insert_empty_line(&mut self)
    { //println!("InsertEmptyLine");
      let mut col = self.size.get_col();
      let resize = self.region;
      let pos = self.screen.position();
      if pos + col > self.len()
      { col = self.len() - pos; }
      let coucou = self.screen.get_mut();
      {0..col}.all(|_|
      { (*coucou).insert(pos, Control::new(&[b' '][..]));
        (*coucou).remove(resize.1 * col);
        true }); }

/*
      ************************* FLAG *************************** 
                !!! OBTENIR 'get' AVEC UNE CLOSURE !!!
      self.screen.get_mut().iter().skip(pos+(col-(pos%col)-1)).map(|&h| h.fold(col-1, |get, &x|
      { if !x.is_space().is_some() // Il faut faire un x.skip(get)
        { get + col }              // sinon on s'arrete au premier espace
        else
        { get }} ));
*/
    /// The method `erase_right_line` erase the current line from the cursor
    /// to the next '\n' encountered
    /// (char under the cursor included)
    pub fn erase_right_line(&mut self)
    { //println!("Cursor::EraseRightLine");
      let col = self.size.get_col();
      let pos = self.screen.position();
        println!("ERASE_RIGHT::{:?}", self.oob);
      if (pos + 1) % col != 0
      { let mut get = col - 1;
        println!("1");
        while !self.screen.get_ref()[pos+(get-(pos%col))].is_space().is_some()
        { get += col; }
        println!("2");
        self.screen.get_mut().into_iter().skip(pos).take(get + 1).all(|mut term: &mut Control|
        { term.clear().is_ok() }); }
      else
      { 
        println!("3");
        self.screen.get_mut()[pos].clear(); }}

    /// The method `erase_left_line` erase the current line from the previous '\n'
    /// to the cursor
    /// (char under the cursor included)
    pub fn erase_left_line(&mut self)
    { //println!("Cursor::EraseLeftLine");
      let col = self.size.get_col();
      let pos = self.screen.position();
      if pos % col != 0
      { let mut get = pos - (pos%col);
        while get >= col && !self.screen.get_ref()[get-1].is_space().is_some()
        { get -= col; }
        if get == col - 1 && !self.screen.get_ref()[pos-((pos%col)+get)].is_space().is_some()
        { get = 0; }
        self.screen.get_mut().into_iter().skip(get).take(pos+1).all(|mut term: &mut Control|
        { term.clear().is_ok() }); }
      else
      { self.screen.get_mut()[pos].clear(); }}

    /// The method `erase_line` erase the entire current line
    pub fn erase_line(&mut self)
    { //println!("Cursor::EraseLine");
      self.erase_left_line();
      self.erase_right_line(); }

    /// The method `erase_up` erase all lines from the current line up to
    /// the top of the screen, and erase the current line from the left border
    /// column to the cursor.
    /// (char under the cursor included)
    pub fn erase_up(&mut self)
    { //println!("Cursor::EraseUp");
      let pos = self.screen.position();
      self.screen.get_mut().into_iter().take(pos + 1).all(|mut term: &mut Control|
      { term.clear().is_ok() }); }

    /// The method `erase_down` erase all lines from the current line down to
    /// the bottom of the screen and erase the current line from the cursor to
    /// the right border column
    /// (char under the cursor included)
    pub fn erase_down(&mut self)
    { //println!("Cursor::EraseDown");
      let pos = self.screen.position();
      let len = self.len();
      self.screen.get_mut().into_iter().skip(pos).take(len - pos + 1).all(|mut term: &mut Control|
      { term.clear().is_ok() }); }

    /// The method `print_enter` reproduce the behavior of a '\n'
    pub fn print_enter(&mut self)
    { println!("PRINT_ENTER::{:?}", self.region);
      if self.oob.1 < self.region.1 - 1
      { self.goto_down(1); }
      else
      { self.scroll_down(); }}

    /// The method `print_char` print an unicode character (1 to 4 chars range)
    pub fn print_char(&mut self, first: &[u8], next: &[u8]) -> io::Result<usize>
    { let wrap = self.line_wrap;
      let row = self.size.get_row();
      let col = self.size.get_col();
        print!("FIRST::{:?} | ", first);
        for i in first
        { print!("{} ", *i as char); }
        println!("\nOOB::({}, {})", self.oob.0, self.oob.1);
      if self.oob.0 < col - 1
      { self.oob.0 += 1; }
      else if self.oob.1 < row - 1
      { self.oob.1 += 1;
        self.oob.0 = 0;
        if !next.is_empty() && next[0] == b' '
        { match next
          { &[] => {},
            &[_, ref tmp..] => return self.screen.write(first).and_then(|f| self.write(tmp).and_then(|n| Ok(f.add(&n)) )) }}}
      else
      { self.scroll_down();
        self.goto_begin_line();
        { let pos = self.screen.position();
          self.goto(pos - 1); }}
        println!("SELF OOB::({}, {})", self.oob.0, self.oob.1);
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

/*
      ************************* FLAG *************************** 
        !!! REMPLACER '8' PAR LA LONGUEUR D'UNE TABULATION !!!
*/
    /// The method `next_tab` return the size of the current printed tabulation
    pub fn next_tab(&self) -> libc::size_t
    { let pos = self.screen.position();
      let mut get: libc::size_t = 1;
      println!("TAB::{}", pos);
      while (pos + get) % 8 != 0
      { get += 1; };
      get }

    /// The method `save_terminal` saves the terminal Display configuration.
    pub fn save_terminal(&mut self)
    { self.save_terminal = Some(Box::new(self.clone())); }

    /// The method `restore_terminal` restore the terminal Display configuration
    /// kept in the 'save_terminal' variable.
    pub fn restore_terminal(&mut self)
    { if let Some(save) = self.save_terminal.take()
      { *self = *save; }}


    /// The method `erase_chars` erases couple of chars on the right of the cursor.
    pub fn erase_chars(&mut self, mv: libc::size_t)
    { let pos = self.screen.position();
      let border = self.size.get_col() - (pos % self.size.get_col());
      let coucou = self.screen.get_mut();
      println!("HEY::{} | {}", pos, border);
      {0..mv}.all(|i|
      { (*coucou).insert(pos + border, Control::new(&[b' '][..]));
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
              { //println!("Save Terminal State");
                self.save_terminal();
                self.write(next) },
            &[b'\x1B', b'[', b'?', b'1', b'0', b'4', b'9', b'l', ref next..] =>
              { //println!("Restore Terminal State");
                self.restore_terminal();
                self.write(next) },

            //------------ SETTINGS -------------
            &[b'\x1B', b'c', ref next..] =>
              { //println!("Cursor::TermReset");
                self.write(next) },
            &[b'\x1B', b'[', b'>', b'0', b'c', ref next..] |
            &[b'\x1B', b'[', b'>', b'c', ref next..] =>
              { //println!("Cursor::TermVersionIn");
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'h', ref next..] |
            &[b'\x1B', b'[', b'2', b'0', b'h', ref next..] =>
            { //println!("Cursor::LineWrap(true)");
                self.line_wrap = true;
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'l', ref next..] |
            &[b'\x1B', b'[', b'2', b'0', b'l', ref next..] =>
              { //println!("Cursor::LineWrap(false)");
                self.line_wrap = false;
                self.write(next) },
            &[b'\x1B', b'[', b'r', ref next..] =>
              { //println!("Cursor::ScrollEnable");
                self.write(next) },

            //------------ ERASE -----------------
            &[b'\x1B', b'[', b'K', ref next..] |
            &[b'\x1B', b'[', b'0', b'K', ref next..] =>
            { self.erase_right_line();
            self.write(next) },
            &[b'\x1B', b'[', b'1', b'K', ref next..] =>
            { self.erase_left_line();
            self.write(next) },
            &[b'\x1B', b'[', b'2', b'K', ref next..] =>
            { self.erase_line();
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
            { self.insert_empty_line();
            self.write(next) },

            //------------- GOTO ------------------
            &[b'\x1B', b'[', b';', b'H', ref next..] |
            &[b'\x1B', b'[', b';', b'f', ref next..] |
            &[b'\x1B', b'[', b'H', ref next..] |
            &[b'\x1B', b'[', b'f', ref next..] =>
            { self.goto_home();
            self.write(next) },
            &[b'\x1B', b'[', b'A', ref next..] |
            &[b'A', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'A', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'A', ref next..] =>
            { self.goto_up(1);
            self.write(next) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'B', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'B', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
            { self.goto_down(1);
            self.write(next) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'C', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'C', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
            { self.goto_right(1);
            self.write(next) },
            &[b'\x1B', b'[', b'D', ref next..] |
            &[b'D', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'D', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'D', ref next..] |
            &[b'\x08', ref next..] =>
            { self.goto_left(1);
            self.write(next) },

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
            &[b'\x1B', b'D', ref next..] =>
            { self.scroll_up();
            self.write(next) },
            &[b'\x1B', b'M', ref next..] =>
            { self.scroll_down();
            self.write(next) },

            //------------ CL ATTR -------------
            &[b'\x1B', b'[', b'0', b'm', ref next..] |
            &[b'\x1B', b'[', b'm', ref next..] =>
              { //println!("Cursor::ClearAttribute");
                self.collection.clear();
                self.write(next) },

            &[b'\x1B', b'[', b'?', ref next..] |
            &[b'\x1B', b'[', b'>', ref next..] |
            &[b'\x1B', b'[', ref next..] |
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

                //------------- INSERT ---------------
                &[b'L', ref next..] =>
                  { if bonjour.len() == 1
                    { {0..bonjour[0]}.all(|_|
                      { self.insert_empty_line();
                        true }); }
                    self.write(next) },

                //-------------- ERASE ----------------
                &[b'P', ref next..] =>
                  { if bonjour.len() == 1
                    { self.erase_chars(bonjour[0]); }
                    self.write(next) },

                //------------- ATTRIBUTS ---------------
                &[b'm', ref next..] =>
                  { self.collection.append(&mut bonjour);
                    self.write(next) },

                //--------------- GOTO ------------------
                &[b'H', ref next..] |
                &[b'f', ref next..] =>
                  { if bonjour.len() == 2 && bonjour[0] > 0 && bonjour[1] > 0
                    { self.goto_coord(bonjour[1] - 1, bonjour[0] - 1); }
                    self.write(next) },

                //----------- TRICKY RESIZE -------------
                &[b'r', ref next..] =>
                  { if bonjour.len() == 2
                    { self.tricky_resize(bonjour[0], bonjour[1]); }
                    self.write(next) },

                //----------- TERM VERSION --------------
                &[b'c', ref next..] =>
                  { if bonjour.len() == 2
                    { /*println!("Cursor::TermVersionOut({}, {})", x, y);*/ }
                    self.write(next) },
                &[b';', b'c', ref next..] =>
                  { if bonjour.len() == 3 && bonjour[2] == 0
                    { /*println!("Cursor::TermVersionOut({}, {})", x, y);*/ }
                    self.write(next) },

                &[_, ref next..] |
                &[ref next..] =>
                  { self.write(next) }, }},

            &[b'\x07', ref next..] => //BELL \b
              { self.bell += 1;
                self.write(next) },
            &[b'\x0A', b'\x0D', ref next..] |
            &[b'\x0D', b'\x0A', ref next..] =>
              { self.goto_begin_line();
                self.print_enter();
                self.write(next) },
            &[b'\x0D'] =>
              { self.goto_begin_line();
              //  if self.oob.1 < self.region.1 - 1
              //  { self.print_enter(); }
                Ok(0) },
            &[b'\x0D', ref next..] =>
              { self.goto_begin_line();
                self.write(next) },
            &[b'\x0A', ref next..] =>
              { self.print_enter();
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
