pub enum Key {
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
    F(u8),
    /// Simple character
    Char(char),
    /// Character used with Alt
    Alt(char),
    /// Character used with Ctrl
    /// Note that certain keys may not be modifiable with `ctrl`, 
    /// due to limitations of terminals.
    Ctrl(char),
    /// Esc key
    Esc,
    /// Null byte
    Null,
    #[allow(missing_docs)]
    #[doc(hidden)]
    __IsNotComplete
}
