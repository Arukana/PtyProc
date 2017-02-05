#[cfg(feature = "task")]
use super::task::{Proc, BufProc};
pub use super::{In, Out, Sig};

use std::io::{self, Read};

use ::chan;
use ::libc;
use ::pty::prelude as pty;

#[cfg(feature = "task")]
pub fn task(tx_task: chan::Sender<BufProc>, pid: libc::pid_t) {
    let mut task = Proc::new(pid).unwrap();
    loop {
        let next = task.next();
        if let Some(name) = next {
            tx_task.send(name);
        }
    }
}

pub fn input(tx_in: chan::Sender<(In, libc::size_t)>) {
    let mut bytes: In = In::default();
    while let Some(read) = io::stdin().read(&mut bytes).ok() {
        tx_in.send((bytes, read));
    }
}

pub fn output(tx_out: chan::Sender<(Out, libc::size_t)>, mut master: pty::Master) {
    let mut bytes: Out = Out::default();
    while let Some(read) = master.read(&mut bytes).ok() {
        tx_out.send((bytes, read));
    }
}
