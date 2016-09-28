use ::libc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Enter
    Enter,
    /// Backspace
    Backspace,
    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,
    /// Delete key
    Delete,
    /// Insert key
    Insert,
    /// Function keys between 1-12.
    F(libc::c_uchar),
    /// Simple character
    Char(libc::c_uchar),
    /// Character used with Alt
    Alt(libc::c_uchar),
    /// Character used with Ctrl
    /// Note that certain keys may not be modifiable with `ctrl`, 
    /// due to limitations of terminals.
    Ctrl(libc::c_uchar),
    /// Esc key
    Esc,
}
