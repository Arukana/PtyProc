pub mod mouse;
pub mod key;

use ::libc;

pub use super::In;
use self::mouse::Mouse;
use self::key::Key;

#[derive(Clone, Copy, Debug)]
pub enum Operate {
    /// The mouse operate.
    Mouse(Mouse),
    /// The key operate.
    Key(Key),
}

impl Operate {
    /// The constructor method `new` returns evaluated Operate.
    pub fn new(buf: In, len: libc::size_t) -> Self {
        if let Ok(mouse) = Mouse::new(&buf[..len]) {
            Operate::Mouse(mouse)
        } else {
            Operate::Key(Key::from((buf, len)))
        }
    }

    /// The accessor method `is_mouse` returns a Option for the Mouse Operate.
    pub fn is_mouse(&self) -> Option<Mouse> {
        match *self {
            Operate::Mouse(mouse) => Some(mouse),
            Operate::Key(_) => None,
        }
    }

    /// The accessor method `is_key` returns a Option for the Key Operate.
    pub fn is_key(&self) -> Option<Key> {
        match *self {
            Operate::Key(key) => Some(key),
            _ => None,
        }
    }

    pub fn as_input(&self) -> (In, libc::size_t) {
        match *self {
            Operate::Key(key) => key.as_input(),
            Operate::Mouse(mouse) => mouse.as_input(),
        }
    }
}
