pub mod operate;

use std::{fmt, str};

use ::libc;
use ::time;
use ::input;

pub use super::In;
use self::operate::Operate;

#[derive(Clone, Copy, Debug)]
pub struct Control {
  /// Buffer.
  buf: In,
  /// Length.
  len: libc::size_t,
  /// Time where the control was pressed.
  time: time::Tm,
  /// Operation.
  operate: Operate,
}

impl Control {
  /// The constructor method `new` returns a Control's event from Device.
  pub fn new(buf: In, len: libc::size_t) -> Self {
    Control {
      buf: buf,
      len: len,
      time: time::now(),
      operate: input::parse_Operate(unsafe {
        str::from_utf8_unchecked(&buf[..len])
      }).unwrap(),
    }
  }

  /// The accessor method `as_slice` returns the Control Event.
  pub fn as_slice(&self) -> &[libc::c_uchar] {
    &self.buf[..self.len]
  }

  /// The accessor method `as_timer` returns the Time.
  pub fn as_operate(&self) -> &Operate {
    &self.operate
  }
  /// The accessor method `as_timer` returns the Time.
  pub fn as_time(&self) -> &time::Tm {
    &self.time
  }

  /// The accessor method `is_char` returns an Option for the Character Key.
  pub fn is_char(&self) -> Option<libc::c_uchar> {
    match self.buf {
      [c, b'\0', ..] => Some(c),
      _ => None,
    }
  }

  /// The accessor method `is_enter` returns an Option for the Enter Key.
  pub fn is_enter(&self) -> Option<()> {
    match self.buf {
      [b'\r', b'\0', ..] |
      [b'\n', b'\0', ..] => Some(()),
      _ => None,
    }
  }
}

impl fmt::Display for Control {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(
      unsafe {
        str::from_utf8_unchecked(self.as_slice())
      }
    )
  }
}
