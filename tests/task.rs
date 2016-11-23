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
#[cfg(feature = "task")]
fn test_proc_next() {
    env::set_var("HOME", "/tmp");
    {/*
        let mut shell: Shell = Shell::new(None, None, Some("/bin/bash")).unwrap();
        let pid: libc::pid_t = shell.get_pid();
        let process: Proc = Proc::new(pid).unwrap();

        assert!(shell.write(b"/bin/bash\n").is_ok());
        thread::sleep(time::Duration::from_millis(200));
        assert!(process.take(50).find(|&(_, ref event)|
            CStr::from_bytes_with_nul(&event[..5]).eq(
                &CStr::from_bytes_with_nul(b"bash\0")
            )
        ).is_some());*/
    }
    {
        let mut shell: Shell = Shell::new(
            None,
            None,
            Some("/bin/bash"),
        ).unwrap();

        shell.set_window_size_with(&SIZE);
        assert!(shell.write(b"/bin/bash\n").is_ok());
        thread::sleep(time::Duration::from_millis(200));
        assert!(shell.take(50).find(|event|
            event.is_task().and_then(|&(_, ref task)| Some(
                CStr::from_bytes_with_nul(&task[..5]).eq(
                    &CStr::from_bytes_with_nul(b"bash\0")
                )
            )).unwrap_or_default()
        ).is_some());
    }
}
