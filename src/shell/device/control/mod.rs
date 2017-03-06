pub mod operate;

use std::{fmt, str};

use ::libc;
use ::time;

pub use super::In;
pub use self::operate::key::Key;
pub use self::operate::mouse::Mouse;
use self::operate::Operate;

#[derive(Clone, Copy)]
pub struct Control {
    /// Buffer, Length.
    input: (In, libc::size_t),
    /// Time where the control was pressed.
    time: time::Tm,
    /// Operation.
    operate: Operate,
}

impl Control {
    /// The constructor method `new` returns a Control's event from Device.
    pub fn new(buf: In, len: libc::size_t) -> Self {
        Control {
            input: (buf, len),
            time: time::now(),
            operate: Operate::new(buf, len),
        }
    }

    pub fn ss_mod(&mut self, ss: libc::c_uchar) {
        let (ref mut buf, ref mut len) = self.input;
        buf[0] = b'\x1B';
        buf[1] = b'O';
        buf[2] = ss;
        *len = 3;
    }

    /// The accessor method `as_slice` returns the Control Event.
    pub fn as_slice(&self) -> &[libc::c_uchar] {
        let (ref buf, len) = self.input;

        &buf[..len]
    }

    /// The accessor method `as_time` returns the Time.
    pub fn as_time(&self) -> time::Tm {
        self.time
    }

    /// The accessor method `is_enter` returns an Option for the Enter Key.
    pub fn is_enter(&self) -> Option<()> {
        match self.operate.is_key() {
            Some(key) if key.is_enter() => Some(()),
            _ => None,
        }
    }

    /// The accessor method `is_key` returns an Option for
    /// the Key interface.
    pub fn is_key(&self) -> Option<Key> {
        self.operate.is_key()
    }

    /// The accessor method `is_mouse` returns an Option for
    /// the Mouse interface.
    pub fn is_mouse(&self) -> Option<(Mouse, bool, libc::c_ushort, libc::c_ushort)> {
        self.operate.is_mouse()
    }
}

impl fmt::Debug for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Control {{ operate: {:?} }}", self.operate)
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            f.write_str(str::from_utf8_unchecked(self.as_slice()))
        }
    }
}

impl PartialEq for Control {
    fn eq(&self, rhs: &Control) -> bool {
        self.input.eq(&rhs.input)
    }
}

impl From<Operate> for Control {
    fn from(operate: Operate) -> Control {
        Control {
            input: operate.as_input(),
            time: time::now(),
            operate: operate,
        }
    }
}
