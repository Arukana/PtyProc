extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

#[test]
fn test_mouse_new() {
    assert_eq!(Mouse::default().as_input(), (In::from(&[b'\x1B', b'[', b'<', 48, b'm', 48][..]), 6));
}
