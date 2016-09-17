mod err;
mod winsz;

use std::collections::VecDeque;
use std::{io, fmt};

use ::libc;
use self::winsz::Winszed;
pub use self::err::{DisplayError, Result};

#[derive(Debug, Clone)]
pub struct Display {
    size: Winszed,
    screen: VecDeque<u8>,
}

impl Display {
    /// The constructor method `default` returns the `Display`'s interface
    /// from shell.
    pub fn new(fd: libc::c_int) -> Result<Display> {
        match Winszed::new(fd) {
          Err(why) => Err(DisplayError::WinszedFail(why)),
          Ok(size) => Ok(Display {
            size: size,
            screen: VecDeque::with_capacity(
              size.get_row().checked_mul(
                size.get_col()
              ).unwrap_or_default()
            ),
          }),
        }
    }

    pub fn to_matrix(&self) -> Vec<Vec<u8>> {
        let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(self.screen.len());
        let mut flag: bool = true;
        let mut tmp = self.screen.iter();

        {0..self.size.get_row()}.all(|y: usize| {
            let mut coucou: Vec<u8> = Vec::with_capacity(self.size.get_col());
            {0..self.size.get_col()}.all(|x: usize| {
                if flag {
                    if let Some(k) = tmp.next() {
                        coucou.insert(x, *k);
                        if *k == 10 {
                            flag = false;
                        }
                    }
                } else {
                    coucou.insert(x, 0);
                }
                true
            });
            matrix.insert(y, coucou);
            true
        });
        matrix
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
        match self.screen.iter().next() {
            Some(&term) => Some(term),
            None => None,
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            String::from_utf8_lossy(self.screen.as_slices().0),
        )
    }
}

impl io::Write for Display {
    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.iter()
            .map(|&term| {
                self.screen.push_back(term);
            })
            .count())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for Display {
    /// The method `read` from trait `io::Read` sets the screen to
    /// the argument buffer.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(buf.iter_mut()
            .zip(self.screen.iter())
            .map(|(mut dest, src)| {
                *dest = *src;
            })
            .count())
    }
}
