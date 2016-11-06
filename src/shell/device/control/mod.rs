pub mod operate;

use std::{fmt, str};

use ::libc;
use ::time;

pub use super::In;
pub use self::operate::key::Key;
pub use self::operate::mouse::Mouse;
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
      operate: Operate::new(&buf, len),
    }
  }

/*  pub fn clone_from_buf(&mut self, buf: In) {
    self.buf.write(buf);
  } */

  pub fn ss_mod(&mut self, ss: libc::c_uchar)
  { self.buf = [b'\x1B', b'O', ss, 0, 0, 0, 0, 0, 0, 0, 0, 0]; } 

  /// The accessor method `as_slice` returns the Control Event.
  pub fn as_slice(&self) -> &[libc::c_uchar] {
    &self.buf[..self.len]
  }

  /// The accessor method `as_time` returns the Time.
  pub fn as_time(&self) -> time::Tm {
    self.time
  }

  pub fn is_unicode(&self) -> Option<&[libc::c_uchar]> {
    if self.operate.is_key().is_some() {
      Some(self.as_slice())
    } else {
      None
    }
  }

  /// The accessor method `is_enter` returns an Option for the Enter Key.
  pub fn is_enter(&self) -> Option<()> {
    match self.operate.is_key() {
      Some(key) => key.is_enter(),
      None => None,
    }
  }

  /// The accessor method `is_key` returns an Option for
  /// the Key interface.
  pub fn is_key(&self) -> Option<Key> {
    self.operate.is_key()
  }

  /// The accessor method `is_mouse` returns an Option for
  /// the Mouse interface.
  pub fn is_mouse(&self) -> Option<(Mouse, libc::c_ushort, libc::c_ushort)> {
    self.operate.is_mouse()
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

impl PartialEq for Control {
    fn eq(&self, rhs: &Control) -> bool {
        self.buf.eq(&rhs.buf)
    }
}
