extern crate pty_proc;
extern crate libc;

#[cfg(feature = "task")]
use pty_proc::prelude::*;

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

        assert!(shell.write(b"/bin/sh\n").is_ok());
        thread::sleep(time::Duration::from_millis(200));

        let sh: Option<String> = process.next();
        assert!(shell.write(b"exit\n").is_ok());
        assert!(shell.write(b"exit\n").is_ok());
        assert_eq!(sh, Some("sh".to_string()));
    }
    {
        let mut shell: Shell = Shell::new(
            None,
            None,
            Some("/bin/bash"),
        ).unwrap();

        thread::sleep(time::Duration::from_millis(200));
        //assert!(shell.write(b"exit\n").is_ok());
        assert!(shell.take(200).find(|event| {
            event.is_task() == Some(&"bash".to_string())
        }).is_some());
    }
}
