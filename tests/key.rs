extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;
use std::io::Read;

#[test]
fn test_key_new() {
    assert_eq!(Key::from(233u32), Key::Char(233));
}

#[test]
fn test_key_read() {
    let mut buf: [u8; 3] = [0u8; 3];
    let mut k: Key = Key::from(233u32);

    assert_eq!(k.read(&mut buf[..]).ok(), Some(3));
    assert_eq!(buf, [233u8, 0u8, 0u8]);

    let mut k: Key = Key::from(362u32);

    assert_eq!(k.read(&mut buf[..]).ok(), Some(3));
    assert_eq!(buf, [106u8, 1u8, 0u8]);
}
