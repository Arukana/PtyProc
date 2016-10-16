mod err;
mod winsz;

use std::ops::{BitAnd, Add};
use std::{io, fmt};
use std::io::Write;

use ::libc;
pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};

#[derive(Debug, Clone)]
pub struct Display {
    oob: (i64, i64),
    save_position: u64,
    line_wrap: bool,
    size: Winszed,
    screen: io::Cursor<Vec<u8>>,
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
            oob: (0, 0),
            save_position: 0,
            line_wrap: true,
            size: size,
            screen: io::Cursor::new(
              (0..size.row_by_col()).map(|_: usize| b' ')
                                    .collect::<Vec<u8>>()
            ),
        }
    }

    /// The accessor method `get_ref` returns a reference on screen vector.
    pub fn get_ref(&self) -> &Vec<u8> {
      self.screen.get_ref()
    }

    /// The accessor method `get_wrap` returns a mutable reference on
    /// the variable 'line_wrap'
    pub fn get_wrap(&mut self) -> &mut bool
    { &mut self.line_wrap }

    /// The accessor method `out_of_bounds` returns a mutable reference on
    /// the tuple 'oob' and should be called every times the cursor moves
    pub fn out_of_bounds(&mut self) -> &mut (i64, i64)
    { &mut self.oob }

    /// The accessor method `is_oob` returns an option if 
    /// the tuple 'oob' points out of the output screen
    pub fn is_oob(&self) -> Option<()>
    { if self.oob.0 >= 0 && self.oob.0 < self.size.get_col() as i64 &&
         self.oob.1 >= 0 && self.oob.1 < self.size.get_row() as i64
      { None }
      else
      { Some(()) }}

    /// The accessor method `is_border` returns a boolean if 
    /// the tuple 'oob' points to the last left bottom character
    pub fn is_border(&self) -> bool
    { if self.oob.0 == self.size.get_col() as i64 && self.oob.1 == self.size.get_row() as i64 - 1
      { true }
      else
      { false }}

    /// The method `bell_or_goto` displays a bell or go to new coordinates
    pub fn bell_or_goto(&mut self)
    { if self.is_oob().is_some()
      { self.write(&[b'\x07']); }
      else
      { let check = { *(self.out_of_bounds()) };
        let col = self.size.get_col(); 
        self.goto((check.0 + (check.1 * (col as i64))) as u64); }}

    /// The accessor method `get_save` returns a mutable reference on
    /// the variable 'save_position'
    pub fn get_save(&mut self) -> &mut u64
    { &mut self.save_position }

    /// The accessor method `get_mut` returns a reference on screen vector.
    pub fn get_mut(&mut self) -> &mut Vec<u8>
    { self.screen.get_mut() }

    /// The accessor method `get_position` returns the position
    /// of the cursor into the screen vector
    pub fn get_position(&self) -> u64 {
      self.screen.position()
    }

    /// The method `resize` updates the size of the output screen.
    pub fn resize(&mut self) -> Result<()> {
      match Winszed::new(0) {
        Err(why) => Err(DisplayError::WinszedFail(why)),
        Ok(size) => Ok(self.size = size),
      }
    }

    /// The method `tricky_resize` updates the size of the output screen.
    pub fn tricky_resize(&mut self) -> Result<()> {
      match Winszed::new(0) {
        Err(why) => Err(DisplayError::WinszedFail(why)),
        Ok(size) => Ok(self.size = size),
      }
    }

    /// The method `goto` moves the cursor position
    pub fn goto(&mut self, index: libc::c_ulong) -> io::Result<usize> {
        self.screen.set_position(index);
        Ok(0)
    }

    /// The method `clear` puges the screen vector.
    pub fn clear(&mut self) -> io::Result<usize> {
        self.goto(0).is_ok().bitand(
          self.screen.get_mut().iter_mut().all(|mut term: &mut u8| {
            *term = b' ';
            true
          })
        );
        Ok(0)
    }

    /// The method `goto_home` moves the cursor to the top left of the output screen.
    pub fn goto_home(&mut self)
    { //println!("Goto::Home");
      { self.goto(0); }
      let oob: &mut (i64, i64) = self.out_of_bounds();
      (*oob).0 = 0;
      (*oob).1 = 0; }

    /// The method `goto_up` moves the cursor up.
    pub fn goto_up(&mut self)
    { //println!("Goto::Up(1)");
      let col = self.size.get_col();
      let pos = self.get_position();
      let wrap = { *(self.get_wrap()) };
      let len = { (*self.get_ref()).len() };
      if !self.is_oob().is_some() && pos - col as u64 >= 0 && wrap
      { { self.goto((pos - col as u64) as u64); }
        let oob: &mut (i64, i64) = self.out_of_bounds();
        (*oob).1 -= 1; }
      else
      { { let oob: &mut (i64, i64) = self.out_of_bounds();
          (*oob).1 -= 1; }
        self.bell_or_goto(); }}

    /// The method `goto_down` moves the cursor down.
    pub fn goto_down(&mut self)
    { //println!("Goto::Down(1)");
      let col = self.size.get_col();
      let pos = self.get_position();
      let wrap = { *(self.get_wrap()) };
      let len = { (*self.get_ref()).len() };
      if !self.is_oob().is_some() && (pos + col as u64) < len as u64 && wrap
      { { self.goto((pos + col as u64) as u64); }
        let oob: &mut (i64, i64) = self.out_of_bounds();
        (*oob).1 += 1; }
      else
      { { let oob: &mut (i64, i64) = self.out_of_bounds();
          (*oob).1 += 1; }
        self.bell_or_goto(); }}

    /// The method `goto_right` moves the cursor to its right.
    /// If 'line_wrap' is true and the cursor is on the right border,
    /// it moves the cursor to the next line's left border
    pub fn goto_right(&mut self)
    { //println!("Goto::Right(1)");
      let col = self.size.get_col();
      let pos = self.get_position();
      let wrap = { *(self.get_wrap()) };
      if !self.is_oob().is_some() && (pos + 1 % col as u64 != 0 || pos < col as u64 - 1 || wrap)
      { { self.goto((pos + 1) as u64); }
        let oob: &mut (i64, i64) = self.out_of_bounds();
        if (*oob).0 == (col as i64) - 1
        { (*oob).0 = 0;
          (*oob).1 += 1; }}
      else
      { { let oob: &mut (i64, i64) = self.out_of_bounds();
          (*oob).0 += 1; }
        self.bell_or_goto(); }}

    /// The method `goto_left` moves the cursor to its left.
    /// If 'line_wrap' is true and the cursor is on the left border,
    /// it moves the cursor to the previous line's right border
    pub fn goto_left(&mut self)
    { //println!("Goto::Left(1)");
      let col = self.size.get_col();
      let pos = self.get_position();
      let wrap = { *(self.get_wrap()) };
      if !self.is_oob().is_some() && pos > 0 && (pos % col as u64 != 0 || wrap)
      { { self.goto((pos - 1) as u64); }
        let oob: &mut (i64, i64) = self.out_of_bounds();
        if (*oob).0 == 0
        { (*oob).0 = (col as i64) - 1;
          (*oob).1 -= 1; }}
      else
      { { let oob: &mut (i64, i64) = self.out_of_bounds();
          (*oob).0 -= 1; }
        self.bell_or_goto(); }}

    /// The method `goto_coord` moves the cursor to the given coordinates
    pub fn goto_coord(&mut self, x: u16, y: u16)
    { //println!("Cursor::CursorGoto({}, {})", x, y);
      let row = self.size.get_row();
      let col = self.size.get_col();
      { let oob: &mut (i64, i64) = self.out_of_bounds();
        (*oob).0 = x as i64 - 1;
        (*oob).1 = y as i64 - 1; }
      if !self.is_oob().is_some()
      { self.goto(((x - 1) + ((y - 1) * col as u16)) as u64); }
      else
      { self.bell_or_goto(); }}

    /// The method `scroll_up` insert an empty line on top of the screen
    /// (the cursor doesn't move)
    pub fn scroll_up(&mut self)
    { //println!("Cursor::ScrollUp");
      let col = self.size.get_col();
      let coucou = self.get_mut();
      {0..col}.all(|_|
      { (*coucou).pop();
        (*coucou).insert(0, b' ');
        true }); }

    /// The method `scroll_down` append an empty line on bottom of the screen
    /// (the cursor doesn't move)
    pub fn scroll_down(&mut self)
    { //println!("Cursor::ScrollDown");
      let col = self.size.get_col();
      let coucou = self.get_mut();
      {0..col}.all(|_|
      { (*coucou).remove(0);
        (*coucou).push(b' ');
        true }); }

    /// The method `save_position` save a position in the variable 'save_position' to get
    /// restored with self.restore_position() described right after
    /// If save_position() is called many times, only the newest safe will be kept.
    pub fn save_position(&mut self)
    { //println!("Cursor::SaveCursor");
      if !self.is_oob().is_some()
      { let pos = self.get_position();
        let save = self.get_save();
        *save = pos; }}

    /// The method `restore_position` move the cursor to coordinates safe 
    /// with self.save_position() described right before.
    /// If no coordinates were safe, cursor moves to the top left of the output screen
    pub fn restore_position(&mut self)
    { //println!("Cursor::RestoreCursor");
      let pos =
      { let restore = self.get_save();
        *restore };
      { self.goto(pos); }
      let len = { (*self.get_ref()).len() };
      let oob: &mut (i64, i64) = self.out_of_bounds();
      (*oob).0 = (pos % len as u64) as i64 - 1;
      (*oob).1 = (pos / len as u64) as i64; }

    /// The method `insert_empty_line` insert an empty line on the right of the cursor
    /// (the cursor doesn't move)
    pub fn insert_empty_line(&mut self)
    { //println!("InsertEmptyLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let pos = self.get_position();
        let coucou = self.get_mut();
        {0..col}.all(|_|
        { (*coucou).pop();
          (*coucou).insert(pos as usize, b' ');
          true }); }}

    /// The method `erase_right_line` erase the current line from the cursor
    /// to the right border column
    /// (char under the cursor included)
    pub fn erase_right_line(&mut self)
    { //println!("Cursor::EraseRightLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let pos = self.get_position();
        let mut get = col;
        if pos >= col as u64
        { get = pos as usize;
          while (get + 1) % col != 0
          { get += 1; }; }
        self.goto((get - 1) as u64);
        let coucou = self.get_mut();
        {pos as usize..get}.all(|i|
        { (*coucou)[i] = b' ';
          true }); }}

    /// The method `erase_left_line` erase the current line from the left border column
    /// to the cursor
    /// (char under the cursor included)
    pub fn erase_left_line(&mut self)
    { //println!("Cursor::EraseLeftLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let pos = self.get_position();
        let mut get = 0;
        if pos >= col as u64
        { get = pos as usize;
          while get % col != 0
          { get -= 1; }; }
        let coucou = self.get_mut();
        {get..(pos + 1) as usize}.all(|i|
        { (*coucou)[i] = b' ';
          true }); }}

    /// The method `erase_line` erase the entire current line
    pub fn erase_line(&mut self)
    { //println!("Cursor::EraseLine");
      if !self.is_oob().is_some()
      { let col = self.size.get_col();
        let mut pos = self.get_position();
        let mut get = 0;
        while pos as usize % col != 0
        { pos -= 1; };
        while (get + pos + 1) % col as u64 != 0
        { get += 1; };
        self.goto(get + pos);
        let coucou = self.get_mut();
        {pos as usize..(get + pos + 1) as usize}.all(|i|
        { (*coucou)[i] = b' ';
          true }); }}

    /// The method `erase_down` erase all lines from the current line up to the top of the
    /// screen, and erase the current line from the left border column to the cursor.
    /// (char under the cursor included)
    pub fn erase_up(&mut self)
    { //println!("Cursor::EraseUp");
      if !self.is_oob().is_some()
      { let pos = self.get_position();
        let coucou = self.get_mut();
        {0..(pos + 1) as usize}.all(|i|
        { (*coucou)[i] = b' ';
          true }); }}

    /// The method `erase_down` erase all lines from the current line down to the bottom
    /// of the screen and erase the current line from the cursor to the right border column
    /// (char under the cursor included)
    pub fn erase_down(&mut self)
    { //println!("Cursor::EraseDown");
      if !self.is_oob().is_some()
      { let pos = self.get_position();
        let len = { (*self.get_ref()).len() };
        let coucou = self.get_mut();
        {pos as usize..len}.all(|i|
        { (*coucou)[i] = b' ';
          true }); }}

    pub fn print_char(&mut self, first: u8, next: &[u8]) -> io::Result<usize>
    { //println!("FIRST::{}", first);
      if !self.is_oob().is_some() || self.is_border()
      { { let wrap = { *(self.get_wrap()) };
        let row = self.size.get_row();
        let col = self.size.get_col();
        let oob: &mut (i64, i64) = { self.out_of_bounds() };
        if (*oob).0 < col as i64 - 1 || ((*oob).0 == col as i64 - 1 && (*oob).1 == row as i64 - 1)
        { (*oob).0 += 1; }
        else if wrap && (*oob).1 < row as i64 - 1
        { (*oob).0 = 0;
          (*oob).1 += 1; }}
        self.screen.write(&[first]).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) )) }
      else
      { self.write(next) }}
}

impl ExactSizeIterator for Display {
    fn len(&self) -> usize {
        self.size.row_by_col()
    }
}

impl Iterator for Display {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        None
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            String::from_utf8_lossy(&self.screen.get_ref()),
        )
    }
}

impl io::Write for Display {
    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match buf {
            &[] => Ok(0),

            //------------ SETTINGS -------------
            &[b'\x1B', b'c', ref next..] =>
              { //println!("Cursor::TermReset");
                self.write(next) },
            &[b'\x1B', b'[', b'>', b'0', b'c', ref next..] =>
              { //println!("Cursor::TermVersionIn");
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'h', ref next..] =>
              { //println!("Cursor::LineWrap(true)");
                { let wrap: &mut bool = self.get_wrap();
                  *wrap = true; }
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'l', ref next..] =>
              { //println!("Cursor::LineWrap(false)");
                { let wrap: &mut bool = self.get_wrap();
                  *wrap = false; }
                self.write(next) },
            &[b'\x1B', b'[', b'r', ref next..] =>
              { //println!("Cursor::ScrollEnable");
                self.write(next) },

            //------------ ERASE -----------------
            &[b'\x1B', b'[', b'K', ref next..] =>
              { self.erase_right_line();
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'K', ref next..] =>
              { self.erase_left_line();
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'K', ref next..] =>
              { self.erase_line();
                self.write(next) },
            &[b'\x1B', b'[', b'J', ref next..] =>
              { self.erase_down();
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'J', ref next..] =>
              { self.erase_up();
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'J', ref next..] =>
              { self.clear();
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
            &[b'\x1B', b'O', b'A', ref next..] =>
              { self.goto_up();
                self.write(next) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
              { self.goto_down();
                self.write(next) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
              { self.goto_right();
                self.write(next) },
            &[b'\x1B', b'[', b'D', ref next..] |
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
                self.write(next) },

            &[b'\x1B', b'[', ref next..] =>
            { match parse_number!(next)
              { //------------- n GOTO ------------------
                Some((number, &[b'A', ref next..])) =>
                  { {0..number}.all(|_|
                    { self.goto_up();
                      true });
                    self.write(next) },
                Some((number, &[b'B', ref next..])) =>
                  { {0..number}.all(|_|
                    { self.goto_down();
                      true });
                    self.write(next) },
                Some((number, &[b'C', ref next..])) =>
                  { {0..number}.all(|_|
                    { self.goto_right();
                      true });
                    self.write(next) },
                Some((number, &[b'D', ref next..])) =>
                  { {0..number}.all(|_|
                    { self.goto_left();
                      true });
                    self.write(next) },

                Some((number, &[b'm', ref next..])) =>
                  { //println!("Cursor::Attribute({});", number);
                    self.write(next) },

                Some((x, &[b';', ref next..])) =>
                { match parse_number!(next)
                  { Some((c, &[b'H', ref next..])) |
                    Some((c, &[b'f', ref next..])) =>
                      { self.goto_coord(c, x);
                        self.write(next) },

                    Some((y, &[b';', b'0', b'c', ref next..])) |
                    Some((y, &[b';', b'c', ref next..])) =>
                      { //println!("Cursor::TermVersionOut({}, {})", x, y);
                        self.write(next) },

                    Some((y, &[b'r', ref next..])) =>
                      { //println!("Resize::({}, {})", x, y);
                        self.write(next) },

                    _ => 
                      { self.screen.write(&[b'\x1B', b'[', b';']).and_then(|f|
                        self.write(next).and_then(|n| Ok(f.add(&n)) ))}, }},
                _ => 
                  { self.screen.write(&[b'\x1B', b'[']).and_then(|f|
                    self.write(next).and_then(|n| Ok(f.add(&n))) )}, }},
            &[b'\x07', ref next..] => //BELL \b
              { self.write(next) },
            &[b'\x0D', ref next..] => //BASE LINE \r
              { self.write(next) },
            &[b'\x0A', ref next..] => //DOWN \n
              { self.write(next) },
            &[first, ref next..] => 
              { self.print_char(first, next) },
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for Display {
    /// The method `read` from trait `io::Read` sets the screen to
    /// the argument buffer.
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}
