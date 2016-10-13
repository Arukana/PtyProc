mod err;
mod winsz;

use std::ops::Add;
use std::{io, fmt};

use ::libc;
pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};

#[derive(Debug, Clone)]
pub struct Display {
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
            size: size,
            screen: io::Cursor::new (
              (0..size.row_by_col()).map(|_: usize| b'\x20')
                                    .collect::<Vec<u8>>()
            ),
        }
    }

    /// The accessor method `get_ref` returns a reference on screen vector.
    pub fn get_ref(&self) -> &Vec<u8> {
        self.screen.get_ref()
    }

    /// The method `resize` updates the size of the Display interface.
    pub fn resize(&mut self) -> Result<()> {
      match Winszed::new(libc::STDIN_FILENO) {
        Err(why) => Err(DisplayError::WinszedFail(why)),
        Ok(size) => Ok(self.size = size),
      }
    }

    /// The method `goto` moves the cursor position.
    pub fn goto(&mut self, index: libc::c_ulong) -> io::Result<usize> {
        self.screen.set_position(index);
        Ok(0)
    }
}

impl ExactSizeIterator for Display {
    fn len(&self) -> usize {
        self.size.get_col().checked_mul(
          self.size.get_row()
        ).unwrap_or_default()
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
            &[b'\x1B', b'[', b'>', b'0', b'c', ref next..] => {
                println!("Cursor::TermVersionIn");
                self.write(next)
            },
            &[b'\x1B', b'[', b'7', b'h', ref next..] => {
                println!("Cursor::LineWrap(true)");
                self.write(next)
            },
            &[b'\x1B', b'[', b'7', b'l', ref next..] => {
                println!("Cursor::LineWrap(false)");
                self.write(next)
            },
            &[b'\x1B', b'[', b';', b'H', ref next..] |
            &[b'\x1B', b'[', b';', b'f', ref next..] | 
            &[b'\x1B', b'[', b'H', ref next..] |
            &[b'\x1B', b'[', b'f', ref next..] => {
                self.goto(0).and(self.write(next))
            },
            &[b'\x1B', b'[', b'1', b'K', ref next..] => {
                println!("Cursor::EraseLeftLine");
                self.write(next)
            },
            &[b'\x1B', b'[', b'2', b'K', ref next..] => {
                println!("Cursor::EraseLine");
                self.write(next)
            },
            &[b'\x1B', b'[', b'1', b'J', ref next..] => {
                println!("Cursor::EraseUp");
                self.write(next)
            },
            &[b'\x1B', b'[', b'2', b'J', ref next..] => {
                println!("Cursor::Clear");
                self.write(next)
            },
            &[b'\x1B', b'[', b'0', b'm', ref next..] => {
                println!("Cursor::ClearAttribute");
                self.write(next)
            },
            &[b'\x1B', b'[', b'r', ref next..] => {
                println!("Cursor::ScrollEnable");
                self.write(next)
            },
            &[b'\x1B', b'[', b'A', ref next..] => {
                println!("Cursor::CursorUp(1)");
                self.write(next)
            },
            &[b'\x1B', b'[', b'B', ref next..] => {
                println!("Cursor::CursorDown(1)");
                self.write(next)
            },
            &[b'\x1B', b'[', b'C', ref next..] => {
                println!("Cursor::CursorRight(1)");
                self.write(next)
            },
            &[b'\x1B', b'[', b'D', ref next..] => {
                println!("Cursor::CursorLeft(1)");
                self.write(next)
            },
            &[b'\x1B', b'[', b's', ref next..] => {
                println!("Cursor::SaveCursor");
                self.write(next)
            },
            &[b'\x1B', b'[', b'u', ref next..] => {
                println!("Cursor::RestoreCursor");
                self.write(next)
            },
            &[b'\x1B', b'[', b'K', ref next..] => {
                println!("Cursor::EraseRightLine");
                self.write(next)
            },
            &[b'\x1B', b'[', b'J', ref next..] => {
                println!("Cursor::EraseDown");
                self.write(next)
            },
            &[b'\x1B', b'[', b'm', ref next..] => {
                println!("Cursor::ClearAttribute");
                self.write(next)
            },
            &[b'\x1B', b'[', b'>', ref next..] |
            &[b'\x1B', b'>', ref next..] |
            &[b'\x1B', b'[', ref next..] => {
                if let Some((buf, (x, y))) = c_xy!(next) {
                    match buf {
                        &[b';', b'0', b'c', ref next..] |
                        &[b';', b'c', ref next..] => {
                            println!("Cursor::TermVersionOut({}, {})", x, y);
                            self.write(next)
                        },
                        &[b'H', ref next..] |
                        &[b'f', ref next..] => {
                            println!("Cursor::CursorGoto({}, {})", x, y);
                            self.write(next)
                        },
                        _ => Ok(0),
                    }
                } else if let Some((buf, i)) = c_i!(next) {
                    match buf {
                        &[b'A', ref next..] => {
                            println!("Cursor::CursorUp({});", i);
                            self.write(next)
                        },
                        &[b'B', ref next..] => {
                            println!("Cursor::CursorDown({});", i);
                            self.write(next)
                        },
                        &[b'C', ref next..] => {
                            println!("Cursor::CursorRight({});", i);
                            self.write(next)
                        },
                        &[b'D', ref next..] => {
                            println!("Cursor::CursorLeft({});", i);
                            self.write(next)
                        },
                        &[b'm', ref next..] => {
                            println!("Cursor::Attribute({});", i);
                            self.write(next)
                        },
                        _ => Ok(0),
                    }
                } else {
                    Ok(0)
                }
            },
            &[b'\x1B', b'7', ref next..] => {
                println!("Cursor::SaveCursor");
                self.write(next)
            },
            &[b'\x1B', b'c', ref next..] => {
                println!("Cursor::TermReset");
                self.write(next)
            },
            &[b'\x1B', b'8', ref next..] => {
                println!("Cursor::RestoreCursor");
                self.write(next)
            },
            &[b'\x1B', b'D', ref next..] => {
                println!("Cursor::ScrollUp");
                self.write(next)
            },
            &[b'\x1B', b'M', ref next..] => {
                println!("Cursor::ScrollDown");
                self.write(next)
            },
            &[first, ref next..] => self.screen.write(&[first])
                                               .and_then(|f|
                self.write(next).and_then(|n|
                    Ok(f.add(&n))
                )
            ),
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
