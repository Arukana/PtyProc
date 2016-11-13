mod err;
mod winsz;

use std::io::{self, Write};
use std::fmt;
use std::str;

use ::libc;
use ::vt100;

pub use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};

pub struct Display {
    screen: vt100::Screen,
    size: Winszed,
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
        let (x, y) = size.get_row_by_col();
        Display {
            screen: vt100::Screen::new(x, y),
            size: size,
        }
    }

    pub fn resize(&mut self) -> Result<()> {
        self.size = Winszed::new(0).unwrap();
        let (x, y) = self.size.get_row_by_col();
        self.screen.set_window_size(x, y);
        Ok(())
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.size.get_row_by_col();
        write!(f, "{}", self.screen.window_contents_formatted(0, 0, x, y))
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.size.get_row_by_col();
        write!(f, "{}", self.screen.window_contents(0, 0, x, y))
    }
}

impl Write for Display {
    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.
    fn write(&mut self, buf: &[libc::c_uchar]) -> io::Result<usize> {
        Ok(self.screen.process(buf) as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

