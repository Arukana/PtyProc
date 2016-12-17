extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

pub const SIZE: Winszed = Winszed {
    ws_row: 8,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0,
};

#[test]
fn test_winszed_new() {
    assert!(Winszed::new(libc::STDIN_FILENO).is_ok());
}

#[test]
fn test_winszed_resize() {
    assert!(Winszed::from_winsized(libc::STDIN_FILENO, &SIZE).is_ok());

    assert_eq!(Winszed::new(libc::STDIN_FILENO).ok(), Some(SIZE));
}
