extern crate pty_proc;
extern crate libc;

#[cfg(feature = "task")]
use pty_proc::prelude::*;


#[cfg(feature = "task")]
use self::std::env;
#[cfg(feature = "task")]
use std::ops::Not;
#[cfg(feature = "task")]
use self::std::io::Write;
#[cfg(feature = "task")]
use self::std::{thread, time};

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
    {
        let mut shell: Shell = Shell::new(None, None, Some("/bin/bash")).unwrap();
        let pid: libc::pid_t = *shell.get_pid();
        let mut process: Proc = Proc::new(pid).unwrap();

        env::set_var("HOME", "/tmp");
        assert_eq!(process.next(), Some("bash".to_string()));
        assert!(shell.write(b"/bin/sh\n").is_ok());
        thread::sleep(time::Duration::from_millis(2000));
        assert_eq!(process.next(), Some("sh".to_string()));

        assert!(shell.write(b"/bin/bash\n").is_ok());
        thread::sleep(time::Duration::from_millis(2000));
        assert_eq!(process.next(), Some("bash".to_string()));

        thread::sleep(time::Duration::from_millis(2000));
        assert!(shell.write(b"exit\n").is_ok());
        thread::sleep(time::Duration::from_millis(2000));
        process.next();
        assert_eq!(process.next(), Some("sh".to_string()));
    }
    {
        let mut shell: Shell = Shell::new(
            None,
            None,
            Some("/bin/sh"),
        ).unwrap();

        env::set_var("HOME", "/tmp");
        assert!(shell.write(b"/bin/sh\n").is_ok());
        assert!(shell.write(b"/bin/bash\n").is_ok());
        thread::sleep(time::Duration::from_millis(200));
        assert!(shell.take(50).find(|event| {
             event.is_task().eq(&Some(&"bash".to_string())).not()
        }).is_some());
    }
}
