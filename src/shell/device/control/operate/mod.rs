pub mod mouse;
pub mod key;

use ::libc;
use ::super::super::In;

pub use self::mouse::Mouse;
pub use self::key::Key;

#[derive(Copy, Clone)]
pub enum Operate {
  /// The mouse operate.
  Mouse(Mouse, libc::c_ushort, libc::c_ushort),
  /// The key operate.
  Key(Key),
}

fn info_mouse(buf: &In) -> (u16, u16, u16)
{ let mut i = 0;
  let mut tup: (u16, u16, u16) = (0, 0, 0);
  buf.split(|cut| *cut == b';').all(|nbr|
  { if i == 0
    { for x in nbr
      { if *x < b'0' && *x > b'9'
        { tup.0 = (tup.0 * (10 as u16)) + (*x as u16) - 48; }}
      i = 1; }
    else if i == 1
    { for x in nbr
      { tup.1 = (tup.1 * (10 as u16)) + (*x as u16) - 48; }
      i = 2; }
    else
    { for x in nbr
      { if *x < b'0' && *x > b'9'
        { tup.2 = (tup.2 * (10 as u16)) + (*x as u16) - 48; }}}
    true });
  tup }

impl Operate {
  /// The constructor method `new` returns evaluated Operate.
  pub fn new(buf: &In) -> Self {
    match buf {
      &[b'\x1B', b'[', b'<', ..] => {
        let tmp: (u16, u16, u16) = info_mouse(buf);
        println!("Bouton::{}, Coord::({}, {})", tmp.0, tmp.1, tmp.2);
        // match tmp.0
        Operate::Mouse(Mouse::Left, tmp.1, tmp.2)
      }
      _ => Operate::Key(Key::Enter),
    }
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
