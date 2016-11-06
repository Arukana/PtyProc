#[cfg(feature = "task")]
use super::task::Proc;
pub use super::{In, Out, Sig};

use std::io::{self, Read};
#[cfg(feature = "task")]
use std::{thread, time};

use ::chan;
use ::libc;
use ::pty::prelude as pty;

#[cfg(feature = "task")]
pub fn task(tx_task: chan::Sender<String>, pid: libc::pid_t) {
    let mut task = Proc::new(pid).unwrap();
    loop {
        thread::sleep(time::Duration::from_millis(200));
        let next = task.next();
        if let Some(name) = next {
            tx_task.send(name);
        }
    }
}

pub fn input(tx_in: chan::Sender<(In, libc::size_t)>) {
    let mut bytes: In = [0u8; 12];
    while let Some(read) = io::stdin().read(&mut bytes).ok() {
        tx_in.send((bytes, read));
    }
}

pub fn output(tx_out: chan::Sender<(Out, libc::size_t)>, mut master: pty::Master) {
    let mut bytes: Out = [0u8; 4096];
    while let Some(read) = master.read(&mut bytes).ok() {
        tx_out.send((bytes, read));
    }
}

pub fn signal(tx_sig: chan::Sender<libc::c_int>) {
    use std::sync::atomic::{AtomicI32, ATOMIC_I32_INIT, Ordering};
    use std::time::Duration;
    use std::ops::Div;
    use std::thread;

    static GOT_SIGNAL: AtomicI32 = ATOMIC_I32_INIT;

    unsafe fn event(sig: Sig) {
        GOT_SIGNAL.store(sig, Ordering::Release);
    }
    let duration: Duration = Duration::from_secs(1).div(2);
    unsafe {
        libc::signal(libc::SIGWINCH, event as libc::sighandler_t);
        loop {
            match GOT_SIGNAL.load(Ordering::Relaxed) {
                0 => thread::sleep(duration),
                sig => {
                    GOT_SIGNAL.store(0, Ordering::Release);
                    tx_sig.send(sig);
                },
            }
        }
    }
}
