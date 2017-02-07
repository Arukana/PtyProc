#[cfg(feature = "task")]
use super::task::{Proc, BufProc};
pub use super::{In, Out, Sig};

use std::io::{self, Read};

#[cfg(feature = "task")]
use std::{time, thread};
use std::sync::mpsc;

use ::libc;
use ::pty::prelude as pty;

#[cfg(feature = "task")]
pub fn task(delay: time::Duration, tx_task: mpsc::SyncSender<BufProc>, pid: libc::pid_t) {
    let mut task = Proc::new(pid).unwrap();
    loop {
        let next = task.next();
        if let Some(name) = next {
            let _ = tx_task.send(name);
        }
        thread::sleep(delay);
    }
}

pub fn input(tx_in: mpsc::SyncSender<(In, libc::size_t)>) {
    let mut bytes: In = In::default();
    while let Some(read) = io::stdin().read(&mut bytes).ok() {
        let _ = tx_in.send((bytes, read));
    }
}

pub fn output(tx_out: mpsc::SyncSender<(Out, libc::size_t)>, mut master: pty::Master) {
    let mut bytes: Out = Out::default();
    while let Some(read) = master.read(&mut bytes).ok() {
        let _ = tx_out.send((bytes, read));
    }
}
