extern crate pty_proc;
extern crate libc;

use pty_proc::prelude::*;

use std::io::Write;
use std::env;

use self::std::{thread, time};

#[test]
#[ignore]
fn test_overflow() {
    let mut shell: Shell = Shell::new(
        None,
        None,
        Some("/bin/bash"),
        None,
    ).unwrap();

    env::set_var("HOME", "/tmp");
    assert!(<Shell as Write>::write(&mut shell, b"/bin/bash\n").is_ok());
    thread::sleep(time::Duration::from_millis(200));
    assert!(<Shell as Write>::write(&mut shell, b"env > /tmp/hello\n").is_ok());
    thread::sleep(time::Duration::from_millis(200));
}
