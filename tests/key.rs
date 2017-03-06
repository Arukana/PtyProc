extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

#[test]
fn test_key_new() {
    assert_eq!(Key::from(233u32), Key::Char(233));
}
