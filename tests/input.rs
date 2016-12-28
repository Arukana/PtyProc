extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

#[test]
fn test_arrow_left() {
    let mut buf: In = In::default();

    buf[0] = b'\x1B';
    buf[1] = b'[';
    buf[2] = b'D';
    assert!(
        Control::new(buf, 3).is_key().unwrap().is_left()
    );
}
