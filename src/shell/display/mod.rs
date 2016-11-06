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

pub type In = [libc::c_uchar; 4];

#[derive(Debug, Clone)]
pub struct Display {
    save_position: libc::size_t,
    save_terminal: Option<Box<Display>>,
    ///Scroll_region set with \x1B[y1;y2r => region(y1, y2)
    region: (libc::size_t, libc::size_t),
    collection: Vec<libc::size_t>,
    oob: (libc::ssize_t, libc::ssize_t),
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
            save_position: 0,
            save_terminal: None,
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

    /// The accessor method `is_oob` returns an option if
    /// the tuple 'oob' points out of the output screen
    pub fn is_oob(&self) -> Option<(libc::ssize_t, libc::ssize_t)> {
        let (x, y): (libc::ssize_t, libc::ssize_t) = self.oob;

        if x.is_negative().not().bitand(&x.gt(&self.size.get_icol()))
                                .bitand(
                y.is_negative().not().bitand(&y.gt(&self.size.get_irow()))
       ) {
            Some((x, y))
        } else {
            None
        }
    }

    /// The accessor method `is_border` returns a boolean if
    /// the tuple 'oob' points to the last left bottom character
    pub fn is_border(&self) -> bool
    { if self.oob.0 != self.size.get_icol() || self.oob.1 != self.size.get_irow() - 1
      { false }
      else
      { true }}


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

    /// The method `bell_or_goto` displays a bell or go to new coordinates
    pub fn bell_or_goto(&mut self) -> io::Result<libc::size_t>
    { if self.is_oob().is_some()
      { self.write(&[b'\x07']) }
      else
      { let check = self.oob;
        let col = self.size.get_icol();
        self.goto((check.0 + (check.1 * col)) as libc::size_t) }}

    /// The method `goto` moves the cursor position
    pub fn goto(&mut self, index: libc::size_t) -> io::Result<libc::size_t> {
        self.screen.set_position(index);
        Ok(0)
    }

    /// The method `goto_home` moves the cursor to the top left of the output screen.
    pub fn goto_home(&mut self) -> io::Result<libc::size_t>
    { //println!("Goto::Home");
      { self.goto(0); }
      self.oob = (0, 0);
      Ok(0) }

    /// The method `goto_up` moves the cursor up.
    pub fn goto_up(&mut self) -> io::Result<libc::size_t>
    { 
      let col = self.size.get_icol();
      let pos = self.screen.position() as libc::ssize_t;
      if !self.is_oob().is_some() && pos - col >= 0 && self.line_wrap
      { self.goto(pos.sub(&col) as libc::size_t);
        self.oob.1 -= 1;
        Ok(0) }
      else
      { self.oob.1 -= 1;
        self.bell_or_goto() }}

    /// The method `goto_down` moves the cursor down.
    pub fn goto_down(&mut self) -> io::Result<libc::size_t>
    { 
      let col = self.size.get_col();
      let pos = self.screen.position();
      let len = { (*self.into_bytes()).len() };
      if !self.is_oob().is_some() && (pos + col) < len && self.line_wrap
      { self.goto(pos + col);
        self.oob.1 += 1;
        Ok(0) }
      else
      { self.oob.1 += 1;
        self.bell_or_goto() }}

    /// The method `goto_right` moves the cursor to its right.
    /// If 'line_wrap' is true and the cursor is on the right border,
    /// it moves the cursor to the next line's left border
    pub fn goto_right(&mut self) -> io::Result<libc::size_t>
    { 
      let col = self.size.get_col();
      let pos = self.screen.position();
      if !self.is_oob().is_some() && (pos + 1 % col != 0 || pos < col - 1 || self.line_wrap)
      { { self.goto(pos + 1); }
        if self.oob.0 < (col as libc::ssize_t) - 1
        { self.oob.0 += 1; }
        else
        { self.oob.0 = 0;
          self.oob.1 += 1; } Ok(0)}
      else
        { {
          self.oob.0 += 1; }
        self.bell_or_goto() }}

    /// The method `goto_left` moves the cursor to its left.
    /// If 'line_wrap' is true and the cursor is on the left border,
    /// it moves the cursor to the previous line's right border
    pub fn goto_left(&mut self) -> io::Result<libc::size_t>
    { 
      let col = self.size.get_col();
      let pos = self.screen.position();
      if !self.is_oob().is_some() && pos > 0 && (pos % col != 0 || self.line_wrap)
      { { self.goto(pos - 1); }
        if self.oob.0 > 0
        { self.oob.0 -= 1; }
        else
        { self.oob.0 = (col as libc::ssize_t) - 1;
          self.oob.1 -= 1; } Ok(0)}
      else
        { self.oob.0 -= 1;
      		self.bell_or_goto() }}

    /// The method `goto_begin_line` moves the cursor to the beginning of the line
    pub fn goto_begin_line(&mut self)
    { if !self.is_oob().is_some() || self.is_border()
      { let x = self.oob.0;
        {0..x}.all(|_|
        { self.goto_left();
          true }); }}

    /// The method `goto_coord` moves the cursor to the given coordinates
    pub fn goto_coord(&mut self, x: libc::size_t, y: libc::size_t)
    { //println!("Cursor::CursorGoto({}, {})", x, y);
      let col = self.size.get_col();
        self.oob = (x as libc::ssize_t - 1, y as libc::ssize_t - 1);
      if !self.is_oob().is_some()
      { self.goto((x - 1) + ((y - 1) * col)); }
      else
      { self.bell_or_goto(); }}

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
      let coucou = self.screen.get_mut();
      {0..col}.all(|_|
      {
        (*coucou).insert(resize.1 * col, Control::new(&[b' '][..]));
        (*coucou).remove(resize.0 * col);
        true }); }

    /// The method `save_position` save a position in the variable 'save_position' to get
    /// restored with self.restore_position() described right after
    /// If save_position() is called many times, only the newest safe will be kept.
    pub fn save_position(&mut self)
    { //println!("Cursor::SaveCursor");
      if !self.is_oob().is_some()
      { let pos = self.screen.position();
        self.save_position = pos; }}

    /// The method `restore_position` move the cursor to coordinates safe
    /// with self.save_position() described right before.
    /// If no coordinates were safe, cursor moves to the top left of the output screen
    pub fn restore_position(&mut self)
    { //println!("Cursor::RestoreCursor");
      let pos = self.save_position;
      self.goto(pos);
      let len = (*self.into_bytes()).len();
      self.oob = ((pos % len ) as libc::ssize_t - 1, (pos / len ) as libc::ssize_t); }

    /// The method `insert_empty_line` insert an empty line on the right of the cursor
    /// (the cursor doesn't move)
    pub fn insert_empty_line(&mut self)
    { //println!("InsertEmptyLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let resize = self.region;
        let pos = self.screen.position();
        let coucou = self.screen.get_mut();
        {0..col}.all(|_|
        { (*coucou).insert(pos, Control::new(&[b' '][..]));
          (*coucou).remove(resize.1 * col);
            true });
    }}

    /// The method `erase_right_line` erase the current line from the cursor
    /// to the right border column
    /// (char under the cursor included)
    pub fn erase_right_line(&mut self)
    { //println!("Cursor::EraseRightLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let pos = self.screen.position();
        let mut get = col;
        if pos >= col
        { get = pos;
          while (get + 1) % col != 0
          { get += 1; }; }
        //self.goto((get - 1) as libc::size_t);
        self.screen.get_mut().into_iter().skip(pos).take(get - pos).all(|mut term: &mut Control|
            term.clear().is_ok()
        );
      }}

    /// The method `erase_left_line` erase the current line from the left border column
    /// to the cursor
    /// (char under the cursor included)
    pub fn erase_left_line(&mut self)
    { //println!("Cursor::EraseLeftLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let pos = self.screen.position();
        let mut get = 0;
        if pos >= col
        { get = pos;
          while get % col != 0
          { get -= 1; }; }
        self.screen.get_mut().into_iter().skip(get).take(get - pos+1).all(|mut term: &mut Control|
            term.clear().is_ok()
        );
    }}

    /// The method `erase_line` erase the entire current line
    pub fn erase_line(&mut self)
    { //println!("Cursor::EraseLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let mut pos = self.screen.position();
        let mut get = 0;
        while pos % col != 0
        { pos -= 1; };
        while (get + pos + 1) % col != 0
        { get += 1; };
        self.goto(pos);
        self.screen.get_mut().into_iter().skip(pos).take(get + pos+1).all(|mut term: &mut Control|
            term.clear().is_ok()
        );
    }}

    /// The method `erase_up` erase all lines from the current line up to
    /// the top of the screen, and erase the current line from the left border
    /// column to the cursor.
    /// (char under the cursor included)
    pub fn erase_up(&mut self)
    { //println!("Cursor::EraseUp");
      if !self.is_oob().is_some()
        { let pos = self.screen.position();
          self.screen.get_mut().into_iter().take(pos+1).all(|mut term: &mut Control|
                term.clear().is_ok()
          );
        }}

    /// The method `erase_down` erase all lines from the current line down to
    /// the bottom of the screen and erase the current line from the cursor to
    /// the right border column
    /// (char under the cursor included)
    pub fn erase_down(&mut self)
    { //println!("Cursor::EraseDown");
      if !self.is_oob().is_some()
      { let pos = self.screen.position();
        let len = self.len();
        self.screen.get_mut().into_iter().skip(pos).take(len + pos).all(|mut term: &mut Control|
            term.clear().is_ok()
        );
      }}

    /// The method `print_enter` reproduce the behavior of a '\n'
    pub fn print_enter(&mut self)
    { if !self.is_oob().is_some()
      { let check = self.oob.1;
        let row = self.size.get_irow();
        if check < row - 1
        { self.goto_down(); }
        else
        { self.scroll_down(); }}
      // !! A VERIFIER !! (Je suppose qu'un \n sur la dernière ligne scroll l'écran)
      else if self.is_border()
      { self.scroll_down(); }}

    /// The method `print_char` print an unicode character (1 to 4 chars range)
    pub fn print_char(&mut self, first: &[u8], next: &[u8]) -> io::Result<usize>
    {
        let wrap = self.line_wrap;
      if !self.is_oob().is_some() || self.is_border()
        {
            {
        let row = self.size.get_irow();
        let col = self.size.get_icol();
        //println!("print_char {}", first.len() );
        if self.oob.0 < col - 1 || (self.oob.0 == col - 1 && self.oob.1 == row - 1)
        { self.oob.0 += 1; }
        else if wrap && self.oob.1 < row - 1
        { self.oob.0 = 0;
          self.oob.1 += 1; }}
        self.screen.write(first).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) )) }
      else
      { self.write(next) }}

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
    pub fn next_tab(&self) -> u8
    { let pos = self.screen.position();
      let mut get: u8 = 1;
      while (pos + get as usize) % 8 != 0
      { get += 1; };
      get }

    /// The method `save_terminal` saves the terminal Display configuration.
    pub fn save_terminal(&mut self) {
        self.save_terminal = Some(Box::new(self.clone()));
    }

    /// The method `restore_terminal` restore the terminal Display configuration
    /// kept in the 'save_terminal' variable.
    pub fn restore_terminal(&mut self)
    {
      if let Some(save) = self.save_terminal.take() {
        *self = *save;
      }} 
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
            { self.goto_up();
            self.write(next) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'B', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'B', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
            { self.goto_down();
            self.write(next) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'C', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'C', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
            { self.goto_right();
            self.write(next) },
            &[b'\x1B', b'[', b'D', ref next..] |
            &[b'D', b'\x08', ref next..] |
            &[b'\x1B', b'[', b'm', b'D', b'\x08', ref next..] |
            &[b'\x1B', b'O', b'D', ref next..] |
            &[b'\x08', ref next..] =>
            { self.goto_left();
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
                    { {0..bonjour[0]}.all(|_|
                      { self.goto_up();
                        true }); }
                    self.write(next) },
                &[b'B', ref next..] =>
                  { if bonjour.len() == 1
                    { {0..bonjour[0]}.all(|_|
                      { self.goto_down();
                        true }); }
                    self.write(next) },
                &[b'C', ref next..] =>
                  { if bonjour.len() == 1
                    { {0..bonjour[0]}.all(|_|
                      { self.goto_right();
                        true }); }
                    self.write(next) },
                &[b'D', ref next..] =>
                  { if bonjour.len() == 1
                    { {0..bonjour[0]}.all(|_|
                      { self.goto_left();
                        true }); }
                    self.write(next) },

                //------------- INSERT ---------------
                &[b'L', ref next..] =>
                  { if bonjour.len() == 1
                    { {0..bonjour[0]}.all(|_|
                      { self.insert_empty_line();
                        true }); }
                    self.write(next) },
                &[b'P', ref next..] =>
                  { if bonjour.len() == 1
                    { let pos = self.screen.position();
                      let border = pos % self.size.get_col();
                      let coucou = self.screen.get_mut();
                      println!("ERASE::{}, {}, {}", pos, border, bonjour[0]);
                      {0..bonjour[0]}.all(|i|
                      { (*coucou).insert(pos + border, Control::new(&[b' '][..]));
                        (*coucou).remove(pos);
                        true }); }
                    self.write(next) },

                //------------- ATTRIBUTS ---------------
                &[b'm', ref next..] =>
                  { self.collection.append(&mut bonjour);
                    self.write(next) },

                //--------------- GOTO ------------------
                &[b'H', ref next..] |
                &[b'f', ref next..] =>
                  { if bonjour.len() == 2
                    { self.goto_coord(bonjour[1], bonjour[0]); }
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
            &[b'\x0D', ref next..] =>
              { self.goto_begin_line();
                self.write(next) },
            &[b'\x0A', ref next..] =>
              { self.print_enter();
                self.write(next) },
            &[b'\x09', ref next..] =>
            { if !self.is_oob().is_some()
              { let resize = self.region;
                let col = self.size.get_col();
                let tab_width = self.next_tab();
                let pos = self.screen.position();
                { let coucou = self.screen.get_mut();
                  {0..tab_width}.all(|_|
                  { (*coucou).insert(pos, Control::new(&[b' '][..]));
                    (*coucou).remove(resize.1 * col);
                    true }); }
                {0..tab_width}.all(|_|
                { self.goto_right();
                  true }); }
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
