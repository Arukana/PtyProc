extern crate pty_proc;
extern crate libc;

#[cfg(feature = "task")]
use pty_proc::prelude::*;

#[cfg(feature = "task")]
use std::ffi::CStr;
#[cfg(feature = "task")]
use self::std::env;
#[cfg(feature = "task")]
use self::std::io::Write;
#[cfg(feature = "task")]
use self::std::{thread, time};

#[cfg(feature = "task")]
const SIZE: Winszed = Winszed {
    ws_row: 8,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0,
};

#[test]
#[cfg(feature = "task")]
fn test_proc_new() {
    assert!(
        Proc::new(unsafe {
            libc::getpid()
        }).is_ok()
    );
}

#[test]
#[ignore]
#[cfg(feature = "task")]
fn test_proc_next()
{ env::set_var("HOME", "/tmp");
  { let mut shell: Shell = Shell::new(
        None,
        None,
        Some("/bin/bash"),
        None,
    ).unwrap();

    shell.set_window_size_with(&SIZE);

    assert!(shell.write(b"/bin/bash\n").is_ok());
    {0..100000000}.all(|_|
    { shell.next().unwrap().is_task().is_none() });

    println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");

    assert!(shell.write(b"/bin/bash\n").is_ok());
    {0..100000000}.all(|_|
    { shell.next().unwrap().is_task().is_none() });

    println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");

    assert!(shell.write(b"/bin/bash\n").is_ok());
    {0..100000000}.all(|_|
    { shell.next().unwrap().is_task().is_none() });

    println!("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ");

    assert!(shell.write(b"exit\n").is_ok());
    {0..100000000}.all(|_|
    { shell.next().unwrap().is_task().is_none() });

    println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");

    assert!(shell.write(b"exit\n").is_ok());
    {0..100000000}.all(|_|
    { shell.next().unwrap().is_task().is_none() });

    println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");

    assert!(shell.write(b"exit\n").is_ok());
    {0..100000000}.all(|_|
    { shell.next().unwrap().is_task().is_none() });

    println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");

    assert!(shell.write(b"exit\n").is_ok()); }}

// cargo test --no-fail-fast --features task -- --nocapture
