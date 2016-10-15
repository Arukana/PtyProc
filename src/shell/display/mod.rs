mod err;
mod winsz;

use std::ops::{BitAnd, Add};
use std::{io, fmt};

use ::libc;
pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};

#[derive(Debug, Clone)]
pub struct Display {
    save_position: u64,
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
            save_position: 0,
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

    /// The accessor method `get_ref` returns a reference on screen vector.
    pub fn get_mut(&mut self) -> &mut Vec<u8>
    { self.screen.get_mut() }

    /// The accessor method `get_position` returns the position
    /// of the cursor into the screen vector
    pub fn get_position(&self) -> u64 {
      self.screen.position()
    }

    /// The method `resize` updates the size of the Display interface.
    pub fn resize(&mut self) -> Result<()> {
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
                self.write(next) },
            &[b'\x1B', b'[', b'7', b'l', ref next..] =>
              { //println!("Cursor::LineWrap(false)");
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
                  if pos >= col as u64
                  { get = pos as usize;
                    while (get + 1) % col != 0
                    { get += 1; }; }
                  self.goto((get - 1) as u64);
                  let coucou = self.get_mut();
                  {pos as usize..get}.all(|i|
                  { (*coucou)[i] = b' ';
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'K', ref next..] =>
              { //println!("Cursor::EraseLeftLine");
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
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'K', ref next..] =>
              { //println!("Cursor::EraseLine");
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
                    true }); }
                self.write(next) },
            &[b'\x1B', b'[', b'J', ref next..] =>
              { //println!("Cursor::EraseDown");
                self.write(next) },
            &[b'\x1B', b'[', b'1', b'J', ref next..] =>
              { //println!("Cursor::EraseUp");
                self.write(next) },
            &[b'\x1B', b'[', b'2', b'J', ref next..] =>
              { self.clear();
                self.write(next) },

            //------------ INSERT -----------------
            &[b'\x1B', b'[', b'L', ref next..] =>
              { println!("InsertEmptyLine");
                { let col = self.size.get_col();
                  let pos = self.get_position();
                  let coucou = self.get_mut();
                  {0..col}.all(|_|
                  { (*coucou).pop();
                    (*coucou).insert(pos as usize, b' ');
                    true }); }
                self.write(next) },

            //------------- GOTO ------------------
            &[b'\x1B', b'[', b';', b'H', ref next..] |
            &[b'\x1B', b'[', b';', b'f', ref next..] | 
            &[b'\x1B', b'[', b'H', ref next..] |
            &[b'\x1B', b'[', b'f', ref next..] |
            &[b'\x1B', b'[', b'?', b'6', b'l', ref next..] =>
              { //println!("Goto::Home");
                self.goto(0);
                self.write(next) },
            &[b'\x1B', b'[', b'A', ref next..] |
            &[b'\x1B', b'O', b'A', ref next..] =>
              { //println!("Goto::Up(1)");
                let col = self.size.get_col();
                let pos = self.get_position();
                self.goto((pos - col as u64) as u64);
                self.write(next) },
            &[b'\x1B', b'[', b'B', ref next..] |
            &[b'\x1B', b'O', b'B', ref next..] =>
              { //println!("Goto::Down(1)");
                let col = self.size.get_col();
                let pos = self.get_position();
                self.goto((pos + col as u64) as u64);
                self.write(next) },
            &[b'\x1B', b'[', b'C', ref next..] |
            &[b'\x1B', b'O', b'C', ref next..] =>
              { //println!("Goto::Right(1)");
                let pos = self.get_position();
                self.goto((pos + 1) as u64);
                self.write(next) },
            &[b'\x1B', b'[', b'D', ref next..] |
            &[b'\x1B', b'O', b'D', ref next..] |
            &[b'\x08', ref next..] =>
              { //println!("Goto::Left(1)");
                let pos = self.get_position();
                self.goto((pos - 1) as u64);
                self.write(next) },

            //--------- POSITION SAVE ----------
            &[b'\x1B', b'[', b's', ref next..] |
            &[b'\x1B', b'7', ref next..] =>
              { //println!("Cursor::SaveCursor");
                self.write(next) },
            &[b'\x1B', b'[', b'u', ref next..] |
            &[b'\x1B', b'8', ref next..] =>
              { //println!("Cursor::RestoreCursor");
                self.write(next) },

            //------------- SCROLL ---------------
            &[b'\x1B', b'D', ref next..] =>
              { //println!("Cursor::ScrollUp");
                self.write(next) },
            &[b'\x1B', b'M', ref next..] =>
              { //println!("Cursor::ScrollDown");
                self.write(next) },

            //------------ CL ATTR -------------
            &[b'\x1B', b'[', b'0', b'm', ref next..] |
            &[b'\x1B', b'[', b'm', ref next..] =>
              { //println!("Cursor::ClearAttribute");
                self.write(next) },

            &[b'\x1B', b'[', ref next..] =>
            { match parse_number!(next)
              { //------------- n GOTO ------------------
                Some((Some(&b'A'), number, ref next)) =>
                  { //println!("Cursor::CursorUp({});", number);
                    let col = self.size.get_col();
                    let pos = self.get_position();
                    self.goto((pos - (number * col as u16) as u64) as u64);
                    self.write(next) },
                Some((Some(&b'B'), number, ref next)) =>
                  { //println!("Cursor::CursorDown({});", number);
                    let col = self.size.get_col();
                    let pos = self.get_position();
                    self.goto((pos + (number * col as u16) as u64) as u64);
                    self.write(next) },
                Some((Some(&b'C'), number, ref next)) =>
                  { //println!("Cursor::CursorRight({});", number);
                    let pos = self.get_position();
                    self.goto((pos + number as u64) as u64);
                    self.write(next) },
                Some((Some(&b'D'), number, ref next)) =>
                  { //println!("Cursor::CursorLeft({});", number);
                    let pos = self.get_position();
                    self.goto((pos + number as u64) as u64);
                    self.write(next) },

                    Some((Some(&b'm'), number, ref next)) =>
                      { //println!("Cursor::Attribute({});", number);
                        self.write(next) },
                    Some((Some(&b';'), x, ref next)) => {
                        match parse_number!(next) {
                            Some((Some(&b'H'), y, ref next)) |
                            Some((Some(&b'f'), y, ref next)) => {
                                println!("Cursor::CursorGoto({}, {})", x, y);
                                let row = self.size.get_row();
                                let col = self.size.get_col();
                                if x <= row as u16 && y <= col as u16
                                { self.goto(((y - 1) + ((x - 1) * (col as u16 - 1))) as u64); }
                                else
                                { panic!("Coordinates out of terminal limits : ({}, {})", y, x); }
                                self.write(next)
                            },
                            Some((Some(&b';'), y, &[b'0', b'c', ref next..])) |
                            Some((Some(&b';'), y, &[b'c', ref next..])) => {
                                //println!("Cursor::TermVersionOut({}, {})", x, y);
                                self.write(next)
                            },

                            Some((Some(&b'r'), y, ref next)) =>
                            { println!("Resize::({}, {})", x, y);
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
            &[first, ref next..] => 
              { self.screen.write(&[first]).and_then(|f| self.write(next).and_then(|n| Ok(f.add(&n)) )) },
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
