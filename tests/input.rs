extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

#[test]
fn test_arrow_left()
{
    assert!(
        Control::new(
            [b'\x1B', b'[', b'D',
            b'\x00',  b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00',
            ], 3
        ).is_key().unwrap().is_left()
    );
}
