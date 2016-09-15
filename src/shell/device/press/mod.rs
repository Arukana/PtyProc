use ::libc;
use ::std::{fmt, str};

use super::In;

#[derive(Copy, Clone)]
pub struct Press {
  /// Buffer.
  buf: In,
  /// Length.
  len: libc::size_t,
}

impl Press {
  /// The constructor method `new` returns a Pressed's event from Device.
  pub fn new(buf: In, len: libc::size_t) -> Self {
    Press {
      buf: buf,
      len: len,
    }
  }

  /// The accessor method `as_slice` returns the pressed Event.
  pub fn as_slice(&self) -> &[libc::c_uchar] {
    &self.buf[..self.len]
  }

  /// The accessor method `is_char` returns a Option for the Character Key.
  pub fn is_char(&self) -> Option<libc::c_uchar> {
    match self.buf {
      [c, b'\0', ..] => Some(c),
      _ => None,
    }
  }

  /// The accessor method `is_enter` returns a Option for the Enter Key.
  pub fn is_enter(&self) -> Option<()> {
    match self.buf {
      [b'\r', b'\0', ..] | [b'\n', b'\0', ..] => Some(()),
      _ => None,
    }
  }

 // [b'[', b'<', n, ..] => {
}

impl fmt::Display for Press {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(
      unsafe {
        str::from_utf8_unchecked(self.as_slice())
      }
    )
  }
}
