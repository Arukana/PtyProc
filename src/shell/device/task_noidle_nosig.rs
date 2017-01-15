use super::spawn;
use super::DeviceState;
use super::input::In;
use super::output::Out;

use ::chan;
use ::libc;
use ::pty::prelude as pty;

use std::thread;

pub use super::procs::{Proc, BufProc};

/// The struct `Device` is the input/output terminal interface.

#[derive(Clone)]
pub struct Device {
    task: chan::Receiver<BufProc>,
    input: chan::Receiver<(In, libc::size_t)>,
    output: chan::Receiver<(Out, libc::size_t)>,
}

impl Device {

    /// The constructor method `new` returns a Device interface iterable.
    fn new (
        task: chan::Receiver<BufProc>,
        input: chan::Receiver<(In, libc::size_t)>,
        output: chan::Receiver<(Out, libc::size_t)>,
    ) -> Self {
        Device {
            task: task,
            input: input,
            output: output,
        }
    }

    pub fn from_speudo(master: pty::Master, pid: libc::pid_t) -> Self {
        let (tx_task, rx_task) = chan::sync(0);
        let (tx_out, rx_out) = chan::sync(0);
        let (tx_in, rx_in) = chan::sync(0);

        thread::spawn(move || spawn::input(tx_in));
        thread::spawn(move || spawn::task(tx_task, pid));
        thread::spawn(move || spawn::output(tx_out, master));
        Device::new(rx_task, rx_in, rx_out)
    }
}

impl Iterator for Device {
    type Item = DeviceState;

    fn next(&mut self) -> Option<DeviceState> {
        let ref task: chan::Receiver<BufProc> = self.task;
        let ref input: chan::Receiver<(In, libc::size_t)> = self.input;
        let ref output: chan::Receiver<(Out, libc::size_t)> = self.output;

        chan_select! {
            task.recv() -> val => return val.and_then(|name| Some(DeviceState::Proc(name))),
            input.recv() -> val => return val.and_then(|(buf, len)| Some(DeviceState::from_in(buf, len))),
            output.recv() -> val => return match val {
                Some((buf, len @ 1 ... 4096)) => Some(
                    DeviceState::from_out(buf, len)
                ),
                _ => None,
            },
        }
    }
}
