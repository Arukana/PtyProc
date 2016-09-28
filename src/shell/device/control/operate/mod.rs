pub mod mouse;
pub mod key;

use super::In;
use ::libc;

pub use self::mouse::Mouse;
pub use self::key::Key;

#[derive(Clone, Copy, Debug)]
pub enum Operate {
  /// The mouse operate.
  Mouse(Mouse, libc::c_ushort, libc::c_ushort),
  /// The key operate.
  Key(Key),
}

impl Operate {
  /// The constructor method `new` returns evaluated Operate.
  pub fn new(buf: &In) -> Self {
    Operate::Key(Key::Enter)
  }

  /// The accessor method `is_mouse` returns a Option for the Mouse Operate.
  pub fn is_mouse(&self) -> Option<Mouse> {
    match *self {
      Operate::Mouse(mouse, _, _) => Some(mouse),
      Operate::Key(_) => None,
    }
  }

  /// The accessor method `is_key` returns a Option for the Key Operate.
  pub fn is_key(&self) -> Option<Key> {
    match *self {
      Operate::Key(key) => Some(key),
      Operate::Mouse(_, _, _) => None,
    }
  }
}
