#[cfg(feature = "task")]
pub mod task;
pub mod control;
mod state;
mod input;
mod output;
mod spawn;

use ::chan;
use ::libc;
use ::pty::prelude as pty;

use std::thread;

pub use self::state::DeviceState;

pub use self::input::In;
pub use self::output::Out;

#[cfg(feature = "task")]
pub use self::task::BufProc;

pub type Sig = libc::c_int;

/// The struct `Device` is the input/output terminal interface.

#[derive(Clone)]
pub struct Device {
    input: chan::Receiver<(In, libc::size_t)>,
    output: chan::Receiver<(Out, libc::size_t)>,
    #[cfg(feature = "task")] task: chan::Receiver<BufProc>,
}

impl Device {

    /// The constructor method `from_speudo` returns a Device interface iterable for a Master.
    #[allow(unused_variables)]
    pub fn from_speudo(master: pty::Master, pid: libc::pid_t) -> Self {
        let (tx_out, rx_out) = chan::sync(0);
        let (tx_in, rx_in) = chan::sync(0);

        thread::spawn(move || spawn::input(tx_in));
        thread::spawn(move || spawn::output(tx_out, master));
        match () {
            #[cfg(not(feature = "task"))]
            () => {
                Device {
                    input: rx_in,
                    output: rx_out,
                }
            },
            #[cfg(feature = "task")]
            () => {
                let (tx_task, rx_task) = chan::sync(0);

                thread::spawn(move || spawn::task(tx_task, pid));
                Device {
                    input: rx_in,
                    output: rx_out,
                    task: rx_task,
                }
            },
        }
    }
}

impl Iterator for Device {
    type Item = DeviceState;

    fn next(&mut self) -> Option<DeviceState> {
        let ref input: chan::Receiver<(In, libc::size_t)> = self.input;
        let ref output: chan::Receiver<(Out, libc::size_t)> = self.output;

        match () {
            #[cfg(all(not(feature = "task"), not(feature = "idle")))]
            () => chan_select! {
                input.recv() -> val => return val.and_then(|(buf, len)| Some(DeviceState::from_in(buf, len))),
                output.recv() -> val => return match val {
                    Some((buf, len @ 1 ... 4096)) => Some(
                        DeviceState::from_out(buf, len)
                    ),
                    _ => None,
                },
            },
            #[cfg(all(not(feature = "task"), feature = "idle"))]
            () => chan_select! {
                default => return Some(DeviceState::from_idle()),
                input.recv() -> val => return val.and_then(|(buf, len)| Some(DeviceState::from_in(buf, len))),
                output.recv() -> val => return match val {
                    Some((buf, len @ 1 ... 4096)) => Some(
                        DeviceState::from_out(buf, len)
                    ),
                    _ => None,
                },
            },
            #[cfg(all(feature = "task", not(feature = "idle")))]
            () => {
                let ref task: chan::Receiver<BufProc> = self.task;
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
            },
            #[cfg(all(feature = "task", feature = "idle"))]
            () => {
                let ref task: chan::Receiver<BufProc> = self.task;
                chan_select! {
                    default => return Some(DeviceState::from_idle()),
                    task.recv() -> val => return val.and_then(|name| Some(DeviceState::Proc(name))),
                    input.recv() -> val => return val.and_then(|(buf, len)| Some(DeviceState::from_in(buf, len))),
                    output.recv() -> val => return match val {
                        Some((buf, len @ 1 ... 4096)) => Some(
                            DeviceState::from_out(buf, len)
                        ),
                        _ => None,
                    },
                }
            },
        }
    }
}
