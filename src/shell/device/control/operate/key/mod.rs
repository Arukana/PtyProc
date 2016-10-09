pub mod chars;

use ::libc;

use self::chars::Char;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Unicode Character.
    Utf8(Char),
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
