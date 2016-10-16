mod err;
mod winsz;
pub mod cursor;
pub mod control;

use std::ops::{BitAnd, Add};
use std::io::{self, Write};
use std::fmt;
use std::str;

use ::libc;

pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};
use self::cursor::Cursor;
use self::control::Control;

pub type In = [libc::c_uchar; 16];

#[derive(Debug, Clone)]
pub struct Display {
    save_position: libc::size_t,
    line_wrap: bool,
    size: Winszed,
    screen: Cursor<Vec<Control>>,
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
            line_wrap: true,
            size: size,
            screen: Cursor::new(
              (0..size.row_by_col()).map(|_: usize|
                                            Control::new(&[b' '][..])
                                        ).collect::<Vec<Control>>()
            ),
        }
    }

    pub fn get_ref(&self) -> Vec<libc::c_uchar> {
        let mut screen: Vec<libc::c_uchar> = Vec::with_capacity(self.len());

        self.screen.get_ref().iter().all(|control: &Control| {
            let buf: &[u8] = control.get_ref();
            screen.extend_from_slice(buf);
            true
        });
        screen
    }

    /// The accessor method `get_wrap` returns a mutable reference on
    /// the variable 'line_wrap'
    pub fn get_wrap(&mut self) -> &mut bool
    { &mut self.line_wrap }

    /// The accessor method `get_save` returns a mutable reference on
    /// the variable 'save_position'
    pub fn get_save(&mut self) -> &mut libc::size_t
    { &mut self.save_position }

    /// The accessor method `get_position` returns the position
    /// of the cursor into the screen vector
    pub fn get_position(&self) -> libc::size_t {
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
    pub fn goto(&mut self, index: libc::size_t) -> io::Result<libc::size_t> {
        self.screen.set_position(index);
        Ok(0)
    }

    /// The method `clear` puges the screen vector.
    pub fn clear(&mut self) -> io::Result<libc::size_t> {
        self.goto(0).is_ok().bitand(
          self.screen.get_mut().iter_mut().all(|mut term: &mut Control| {
            term.clear();
            true
          })
        );
        Ok(0)
    }
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
        let mut screen: String = String::with_capacity(self.len());

        screen.extend(
            self.screen.get_ref().into_iter().map(|control| unsafe {
               let ref character: &[u8] = control.get_ref();
               str::from_utf8_unchecked(character)
            }).collect::<Vec<&str>>()
        );
        write!(f, "{}", screen)
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
              { //println!("Cursor::EraseRightLine");
                { let col = self.size.get_col();
                  let pos = self.get_position();
                  let mut get = col;
                  if pos >= col
                  { get = pos;
                    while (get + 1) % col != 0
                    { get += 1; }; }
                  self.goto((get - 1));
                  let coucou = self.screen.get_mut();
                  {pos..get}.all(|i|
                  { (*coucou)[i].clear();
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'K', ref next..] =>
              { //println!("Cursor::EraseLeftLine");
                { let col = self.size.get_col();
                  let pos = self.get_position();
                  let mut get = 0;
                  if pos >= col
                  { get = pos;
                    while get % col != 0
                    { get -= 1; }; }
                  let coucou = self.screen.get_mut();
                  {get..(pos + 1)}.all(|i|
                  { (*coucou)[i].clear();
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'K', ref next..] =>
              { //println!("Cursor::EraseLine");
                { let col = self.size.get_col();
                  let mut pos = self.get_position();
                  let mut get = 0;
                  while pos % col != 0
                  { pos -= 1; };
                  while (get + pos + 1) % col != 0
                  { get += 1; };
                  self.goto(get + pos);
                  let coucou = self.screen.get_mut();
                  {pos..(get + pos + 1)}.all(|i|
                  { (*coucou)[i].clear();
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'J', ref next..] =>
              { //println!("Cursor::EraseDown");
                { let pos = self.get_position();
                  let len = { (*self.screen.get_ref()).len() };
                  let coucou = self.screen.get_mut();
                  {pos..len}.all(|i|
                  { (*coucou)[i].clear();
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'J', ref next..] =>
              { //println!("Cursor::EraseUp");
                { let pos = self.get_position();
                  let coucou = self.screen.get_mut();
                  {0..(pos + 1)}.all(|i|
                  { (*coucou)[i].clear();
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'J', ref next..] =>
              { self.clear();
                self.write(next) },

            //------------ INSERT -----------------
            &[b'\x1B', b'[', b'L', ref next..] =>
              { println!("InsertEmptyLine");
                { let col = self.size.get_col();
                  let pos = self.get_position();
                  let coucou = self.screen.get_mut();
                  {0..col}.all(|_|
                  { (*coucou).pop();
                    (*coucou).insert(pos, Control::new(&[b' '][..]));
                    true }); }
                self.write(next) },

            //------------- GOTO ------------------
            &[b'\x1B', b'[', b';', b'H', ref next..] |
            &[b'\x1B', b'[', b';', b'f', ref next..] | 
            &[b'\x1B', b'[', b'H', ref next..] |
            &[b'\x1B', b'[', b'f', ref next..] /*|
            &[b'\x1B', b'[', b'?', b'6', b'l', ref next..]*/ =>
              { //println!("Goto::Home");
                self.goto(0);
                self.write(next) },
            &[b'\x1B', b'[', b'A', ref next..] |
            &[b'\x1B', b'O', b'A', ref next..] =>
              { //println!("Goto::Up(1)");
                let col = self.size.get_col();
                let pos = self.get_position();
                let len = { (*self.screen.get_ref()).len() };
                if pos - col >= 0
                { self.goto((pos - col)); }
                else
                { panic!("Cursor::CursorUp(1) moved the cursor out of screen limits"); }
                self.write(next) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
              { //println!("Goto::Down(1)");
                let col = self.size.get_col();
                let pos = self.get_position();
                let len = { (*self.screen.get_ref()).len() };
                if (pos + col) < len
                { self.goto((pos + col)); }
                else
                { panic!("Cursor::CursorDown(1) moved the cursor out of screen limits"); }
                self.write(next) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
              { //println!("Goto::Right(1)");
                let col = self.size.get_col();
                let pos = self.get_position();
                let wrap = { *(self.get_wrap()) };
                if pos + 1 % col != 0 || pos < col - 1 || wrap
                { self.goto((pos + 1)); }
                else
                { panic!("Cursor::CursorRight(1) moved the cursor out of screen limits"); }
                self.write(next) },
            &[b'\x1B', b'[', b'D', ref next..] |
            &[b'\x1B', b'O', b'D', ref next..] |
            &[b'\x08', ref next..] =>
              { //println!("Goto::Left(1)");
                let col = self.size.get_col();
                let pos = self.get_position();
                let wrap = { *(self.get_wrap()) };
                if pos > 0 && (pos % col != 0 || wrap)
                { self.goto((pos - 1)); }
                else
                { panic!("Cursor::CursorLeft(1) moved the cursor out of screen limits"); }
                self.write(next) },

            //--------- POSITION SAVE ----------
            &[b'\x1B', b'[', b's', ref next..] |
            &[b'\x1B', b'7', ref next..] =>
              { //println!("Cursor::SaveCursor");
                { let pos = self.get_position();
                  let save = self.get_save();
                  *save = pos; }
                self.write(next) },
            &[b'\x1B', b'[', b'u', ref next..] |
            &[b'\x1B', b'8', ref next..] =>
              { //println!("Cursor::RestoreCursor");
                { let pos =
                  { let restore = self.get_save();
                    *restore };
                  self.goto(pos); }
                self.write(next) },

            //------------- SCROLL ---------------
            &[b'\x1B', b'D', ref next..] =>
              { //println!("Cursor::ScrollUp");
                { let col = self.size.get_col();
                  let coucou = self.screen.get_mut();
                  {0..col}.all(|i|
                  { (*coucou).pop();
                    (*coucou).insert(i, Control::new(&[b' '][..]));
                    true }); }
                self.write(next) },
            &[b'\x1B', b'M', ref next..] =>
              { //println!("Cursor::ScrollDown");
                { let col = self.size.get_col();
                  let coucou = self.screen.get_mut();
                  {0..col}.all(|_|
                  { (*coucou).remove(0);
                    (*coucou).push(Control::new(&[b' '][..]));
                    true }); }
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
                  { //println!("Cursor::CursorUp({});", number);
                    let col: usize = self.size.get_col();
                    let pos: usize = self.get_position();
                    self.goto((pos - (number * col)));
                    self.write(next) },
                Some((number, &[b'B', ref next..])) =>
                  { //println!("Cursor::CursorDown({});", number);
                    let col: usize = self.size.get_col();
                    let pos: usize = self.get_position();
                    self.goto((pos + (number * col)));
                    self.write(next) },
                Some((number, &[b'C', ref next..])) =>
                  { //println!("Cursor::CursorRight({});", number);
                    let pos: usize = self.get_position();
                    self.goto((pos + number));
                    self.write(next) },
                Some((number, &[b'D', ref next..])) =>
                  { //println!("Cursor::CursorLeft({});", number);
                    let pos: usize = self.get_position();
                    self.goto((pos - number));
                    self.write(next) },

                Some((number, &[b'm', ref next..])) =>
                  { //println!("Cursor::Attribute({});", number);
                    self.write(next) },

                Some((x, &[b';', ref next..])) => {
                  match parse_number!(next) {
                    Some((c, &[b'H', ref next..])) |
                    Some((c, &[b'f', ref next..])) => {
                      //println!("Cursor::CursorGoto({}, {})", x, c);
                      let row: usize = self.size.get_row();
                      let col: usize = self.size.get_col();
                      if x <= row && c <= col
                      { self.goto(((c - 1) + ((x - 1) * col))); }
                      else
                      { panic!("Goto::CursorGoto({}, {}) moved the cursor out of screen limits", x, c); }
                      self.write(next)
                    },

                    Some((y, &[b';', b'0', b'c', ref next..])) |
                    Some((y, &[b';', b'c', ref next..])) => {
                      //println!("Cursor::TermVersionOut({}, {})", x, y);
                      self.write(next)
                    },

                    Some((y, &[b'r', ref next..])) =>
                    { //println!("Resize::({}, {})", x, y);
                      self.write(next) },

                    _ => { println!("HHHHHH  {:?}  HHHHHH", buf);
                      self.screen.write(&[b'\x1B', b'[', b';']).and_then(|f|
                      self.write(next).and_then(|n| Ok(f.add(&n)))
                    )},
                  }
                },
                _ => self.screen.write(&[b'\x1B', b'[']).and_then(|f|
                  self.write(next).and_then(|n| Ok(f.add(&n)))
                ),
              }
            },
            &[b'\x07', ref next..] =>
            { self.write(next) },
            &[b'\x0D', ref next..] =>
            { self.write(next) },
            &[u1 @ b'\xF0' ... b'\xF4', u2 @ b'\x8F' ... b'\x90', u3 @ b'\x80' ... b'\xBF', u4 @ b'\x80' ... b'\xBF', ref next..] => {
              self.screen.write(&[u1, u2, u3, u4]).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) ))
            },
            &[u1 @ b'\xE0' ... b'\xF0', u2 @ b'\x90' ... b'\xA0', u3 @ b'\x80' ... b'\xBF', ref next..] => {
              self.screen.write(&[u1, u2, u3]).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) ))
            },
            &[u1 @ b'\xC2' ... b'\xDF', u2 @ b'\x80' ... b'\xBF', ref next..] => {
              self.screen.write(&[u1, u2]).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) ))
            },
            &[u1, ref next..] => {
              self.screen.write(&[u1]).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) ))
            },
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
