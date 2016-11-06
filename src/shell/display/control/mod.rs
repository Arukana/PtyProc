pub mod operate;

use std::io::{self, Write};
use std::mem;

use ::libc;

pub use super::In;
use self::operate::Operate;

#[derive(Clone, Copy, Debug, Default)]
pub struct Control {
   /// Term's buffer.
   buf: In,
   /// Term's Length.
   len: libc::size_t,
   /// Operation.
   operate: Operate,
}

impl Control {
    /// The constructor method `new` returns a term character.
    pub fn new(buf: &[libc::c_uchar]) -> Self {
        let mut control: Control = Control::default();

        control.write(buf);
        control
    }

    pub fn is_enter(&self) -> Option<()> {
      if self.buf.first().eq(&Some(&b'\n')) {
        Some(())
      } else {
        None
      }
    }

    pub fn is_space(&self) -> Option<()> {
      if self.buf.first().eq(&Some(&b' ')) {
        Some(())
      } else {
        None
      }
    }

    /// The method `clear` resets the term character.
    pub fn clear(&mut self) -> io::Result<usize> {
        *self = Default::default();
        self.write(&[b' '][..])
    }

    /// The accessor method `get_ref` returns a reference on term character buffer.
    pub fn get_ref(&self) -> &[libc::c_uchar] {
        &self.buf[..self.len]
    }

    /// The accessor method `get_ref` returns a reference on term character buffer.
    pub fn as_char(&self) -> char {
        unsafe {
            mem::transmute::<In, char>(self.buf)
        }
    }
}

impl Write for Control {

    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.len = buf.len();
        (&mut self.buf[..]).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
