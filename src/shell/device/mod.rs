#[cfg(feature = "task")]
pub mod task;
pub mod control;

mod state;
mod spawn;

use ::chan;
use ::libc;
use ::pty::prelude as pty;
use std::thread;

pub use self::state::DeviceState;

pub type In = [libc::c_uchar; 12];
pub type Out = [libc::c_uchar; 4096];
pub type Sig = libc::c_int;

/// The struct `Device` is the input/output terminal interface.

#[derive(Clone)]
pub struct Device {
    #[cfg(feature = "task")] task: chan::Receiver<String>,
    input: chan::Receiver<(In, libc::size_t)>,
    output: chan::Receiver<(Out, libc::size_t)>,
    signal: chan::Receiver<libc::c_int>,
}

#[cfg(feature = "task")]
impl Device {

    /// The constructor method `new` returns a Device interface iterable.
    fn new (
        task: chan::Receiver<String>,
        input: chan::Receiver<(In, libc::size_t)>,
        output: chan::Receiver<(Out, libc::size_t)>,
        signal: chan::Receiver<libc::c_int>,
    ) -> Self {
        Device {
            task: task,
            input: input,
            output: output,
            signal: signal,
        }
    }

    pub fn from_speudo(master: pty::Master, pid: libc::pid_t) -> Self {
        let (tx_task, rx_task) = chan::sync(0);
        let (tx_out, rx_out) = chan::sync(0);
        let (tx_in, rx_in) = chan::sync(0);
        let (tx_sig, rx_sig) = chan::sync(0);

        thread::spawn(move || spawn::input(tx_in));
        thread::spawn(move || spawn::task(tx_task, pid));
        thread::spawn(move || spawn::output(tx_out, master));
        thread::spawn(move || spawn::signal(tx_sig, master));
        Device::new(rx_task, rx_in, rx_out, rx_sig)
    }
}

#[cfg(not(feature = "task"))]
impl Device {

    /// The constructor method `new` returns a Device interface iterable.
    fn new (
        input: chan::Receiver<(In, libc::size_t)>,
        output: chan::Receiver<(Out, libc::size_t)>,
        signal: chan::Receiver<libc::c_int>,
    ) -> Self {
        Device {
            input: input,
            output: output,
            signal: signal,
        }
    }

    pub fn from_speudo(mut master: pty::Master, _: libc::pid_t) -> Self {
        let (tx_out, rx_out) = chan::sync(0);
        let (tx_in, rx_in) = chan::sync(0);
        let (tx_sig, rx_sig) = chan::sync(0);

        thread::spawn(move || spawn::input(tx_in));
        thread::spawn(move || spawn::output(tx_out, master));
        thread::spawn(move || spawn::signal(tx_sig, master));
        Device::new(rx_in, rx_out, rx_sig)
    }
}

impl Iterator for Device {
    type Item = DeviceState;

    #[cfg(feature = "task")]
    fn next(&mut self) -> Option<DeviceState> {
        let ref task: chan::Receiver<String> = self.task;
        let ref signal: chan::Receiver<Sig> = self.signal;
        let ref input: chan::Receiver<(In, libc::size_t)> = self.input;
        let ref output: chan::Receiver<(Out, libc::size_t)> = self.output;

        chan_select! {
            default => return Some(DeviceState::from_idle()),
            task.recv() -> val => return val.and_then(|name| Some(DeviceState::Proc(name))),
            signal.recv() -> val => return val.and_then(|sig| Some(DeviceState::from_sig(sig))),
            input.recv() -> val => return val.and_then(|(buf, len)| Some(DeviceState::from_in(buf, len))),
            output.recv() -> val => return match val {
                Some((buf, len @ 1 ... 4096)) => Some(
                    DeviceState::from_out(buf, len)
                ),
                _ => None,
            },
        }
    }

    #[cfg(not(feature = "task"))]
    fn next(&mut self) -> Option<DeviceState> {
        let ref signal: chan::Receiver<Sig> = self.signal;
        let ref input: chan::Receiver<(In, libc::size_t)> = self.input;
        let ref output: chan::Receiver<(Out, libc::size_t)> = self.output;

        chan_select! {
            default => return Some(DeviceState::from_idle()),
            signal.recv() -> val => return val.and_then(|sig| Some(DeviceState::from_sig(sig))),
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
