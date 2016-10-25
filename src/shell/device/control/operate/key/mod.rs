use std::char;
use std::mem;

use ::libc;

pub use super::In;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Unicode Character.
    Utf8(char),
    /// Enter.
    Enter,
    ///Tabulation.
    Tab,
    /// Backspace.
    Backspace,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys between 1-19.
    F(libc::c_uchar),
    /// Character used with Alt.
    Alt(libc::c_uchar),
    /// Character used with Ctrl
    /// Note that certain keys may not be modifiable with `ctrl`,
    /// due to limitations of terminals.
    Ctrl(libc::c_uchar),
    /// Esc key.
    Esc,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Shift Left arrow.
    ShiftLeft,
    /// Shift Right arrow.
    ShiftRight,
    /// Shift Up arrow.
    ShiftUp,
    /// Shift Down arrow.
    ShiftDown,
    /// Alt Left arrow.
    AltLeft,
    /// Alt Right arrow.
    AltRight,
    /// Alt Up arrow.
    AltUp,
    /// Alt Down arrow.
    AltDown,
    /// Control Left arrow.
    CtrlLeft,
    /// Control Right arrow.
    CtrlRight,
    /// Control Up arrow.
    CtrlUp,
    /// Control Down arrow.
    CtrlDown,
    /// Alt Shift Left arrow.
    AltShiftLeft,
    /// Alt Shift Right arrow.
    AltShiftRight,
    /// Alt Shift Up arrow.
    AltShiftUp,
    /// Alt Shift Down arrow.
    AltShiftDown,
    /// Control Shift Left arrow.
    CtrlShiftLeft,
    /// Control Shift Right arrow.
    CtrlShiftRight,
    /// Control Shift Up arrow.
    CtrlShiftUp,
    /// Control Shift Down arrow.
    CtrlShiftDown,
}

impl Key {
  /// The constructor method `new` returns a parsed Key.
  pub fn new(buf: &In, _: libc::size_t) -> Self {
    match buf {
      &[b'\x1B', b'\0', ..] => Key::Esc,
      &[b'\t', b'\0', ..] => Key::Tab,
      &[b'\x7F', b'\0', ..] => Key::Backspace, 
      &[b'\x1B', b'O', b'P', b'\0', ..] => Key::F(1),
      &[b'\x1B', b'O', b'Q', b'\0', ..] => Key::F(2),
      &[b'\x1B', b'O', b'R', b'\0', ..] => Key::F(3),
      &[b'\x1B', b'O', b'S', b'\0', ..] => Key::F(4),
      &[b'\x1B', b'[', b'A', b'\0', ..] => Key::Up,
      &[b'\x1B', b'[', b'B', b'\0', ..] => Key::Down,
      &[b'\x1B', b'[', b'C', b'\0', ..] => Key::Right,
      &[b'\x1B', b'[', b'D', b'\0', ..] => Key::Left,
      &[b'\x1B', b'[', b'F', b'\0', ..] => Key::End,
      &[b'\x1B', b'[', b'3', b'~', b'\0', ..] => Key::Delete,
      &[b'\x1B', b'[', b'5', b'~', b'\0', ..] => Key::PageUp,
      &[b'\x1B', b'[', b'6', b'~', b'\0', ..] => Key::PageDown,
      &[b'\x1B', b'[', b'1', b'5', b'~', b'\0', ..] => Key::F(5),
      &[b'\x1B', b'[', b'1', b'7', b'~', b'\0', ..] => Key::F(6),
      &[b'\x1B', b'[', b'1', b'8', b'~', b'\0', ..] => Key::F(7),
      &[b'\x1B', b'[', b'1', b'9', b'~', b'\0', ..] => Key::F(8),
      &[b'\x1B', b'[', b'2', b'0', b'~', b'\0', ..] => Key::F(9),
      &[b'\x1B', b'[', b'2', b'1', b'~', b'\0', ..] => Key::F(10),
      &[b'\x1B', b'[', b'2', b'3', b'~', b'\0', ..] => Key::F(11),
      &[b'\x1B', b'[', b'2', b'4', b'~', b'\0', ..] => Key::F(12),
      &[b'\x1B', b'[', b'1', b';', b'2', b'P', b'\0', ..] => Key::F(13),
      &[b'\x1B', b'[', b'1', b';', b'2', b'Q', b'\0', ..] => Key::F(14),
      &[b'\x1B', b'[', b'1', b';', b'2', b'R', b'\0', ..] => Key::F(15),
      &[b'\x1B', b'[', b'1', b';', b'2', b'S', b'\0', ..] => Key::F(16),
      &[b'\n', b'\0', ..] => Key::Enter,
      &[b'\r', b'\0', ..] => Key::Enter,
      &[b'\n', b'\r', b'\0', ..] => Key::Enter,
      &[b'\x1B', b'[', b'1', b';', b'2', b'A', b'\0', ..] => Key::ShiftUp,
      &[b'\x1B', b'[', b'1', b';', b'2', b'B', b'\0', ..] => Key::ShiftDown,
      &[b'\x1B', b'[', b'1', b';', b'2', b'C', b'\0', ..] => Key::ShiftRight,
      &[b'\x1B', b'[', b'1', b';', b'2', b'D', b'\0', ..] => Key::ShiftLeft,
      &[b'\x1B', b'[', b'1', b';', b'9', b'A', b'\0', ..] => Key::AltUp,
      &[b'\x1B', b'[', b'1', b';', b'9', b'B', b'\0', ..] => Key::AltDown,
      &[b'\x1B', b'[', b'1', b';', b'9', b'C', b'\0', ..] => Key::AltRight,
      &[b'\x1B', b'[', b'1', b';', b'9', b'D', b'\0', ..] => Key::AltLeft,
      &[b'\x1B', b'[', b'1', b';', b'5', b'A', b'\0', ..] => Key::CtrlUp,
      &[b'\x1B', b'[', b'1', b';', b'5', b'B', b'\0', ..] => Key::CtrlDown,
      &[b'\x1B', b'[', b'1', b';', b'5', b'C', b'\0', ..] => Key::CtrlRight,
      &[b'\x1B', b'[', b'1', b';', b'5', b'D', b'\0', ..] => Key::CtrlLeft,
      &[b'\x1B', b'[', b'1', b';', b'6', b'A', b'\0', ..] => Key::CtrlShiftUp,
      &[b'\x1B', b'[', b'1', b';', b'6', b'B', b'\0', ..] => Key::CtrlShiftDown,
      &[b'\x1B', b'[', b'1', b';', b'6', b'C', b'\0', ..] => Key::CtrlShiftRight,
      &[b'\x1B', b'[', b'1', b';', b'6', b'D', b'\0', ..] => Key::CtrlShiftLeft,
      &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'A', b'\0', ..] => Key::AltShiftUp,
      &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'B', b'\0', ..] => Key::AltShiftDown,
      &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'C', b'\0', ..] => Key::AltShiftRight,
      &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'D', b'\0', ..] => Key::AltShiftLeft,
      &[u1 @ b'\xF0' ... b'\xF4',
        u2 @ b'\x8F' ... b'\x90', 
        u3 @ b'\x80' ... b'\xBF',
        u4 @ b'\x80' ... b'\xBF', ..] => Key::from_utf8([u1, u2, u3, u4]),
      &[u1 @ b'\xE0' ... b'\xF0', u2 @ b'\x90' ... b'\xA0', 
        u3 @ b'\x80' ... b'\xBF', ..] => Key::from_utf8([u1, u2, u3, b'\x00']),
      &[u1 @ b'\xC2' ... b'\xDF',
        u2 @ b'\x80' ... b'\xBF', ..] => Key::from_utf8([u1, u2, b'\x00', b'\x00']),
      &[u1, ..] => Key::from_utf8([u1, b'\x00', b'\x00', b'\x00']),
    }
  }

  /// The constructor method `from_utf8` returns a UTF-8 parsed Key.
  pub fn from_utf8(buf: [libc::c_uchar; 4]) -> Self {
    unsafe {
      let i: libc::c_uint = mem::transmute::<[libc::c_uchar; 4], libc::c_uint>(buf);
      let c: char = char::from_u32_unchecked(i);

      Key::Utf8(c)
    }
  }

  /// The accessor method `is_enter` returns an Option for the Enter Key.
  pub fn is_enter(&self) -> Option<()> {
    if self.eq(&Key::Enter) {
      Some(())
    } else {
      None
    }
  }
}
