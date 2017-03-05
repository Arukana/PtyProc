#[cfg(feature = "task")]
pub mod task;
pub mod control;
pub mod state;
mod input;
mod output;
mod spawn;

use ::libc;
use ::pty::prelude as pty;

use std::time;
use std::thread;
use std::sync::mpsc;

pub use self::state::DeviceState;

pub use self::input::In;
pub use self::output::Out;

#[cfg(feature = "task")] 
const TM_TASK: u64 = 100;
const TM_DEVICE: u64 = 10;

#[cfg(feature = "task")]
pub use self::task::BufProc;

pub type Sig = libc::c_int;

/// The struct `Device` is the input/output terminal interface.

#[derive(Debug)]
pub struct Device {
    delay: time::Duration,
    input: mpsc::Receiver<(In, libc::size_t)>,
    output: mpsc::Receiver<(Out, libc::size_t)>,
    #[cfg(feature = "task")] task: mpsc::Receiver<BufProc>,
}

impl Device {

    /// The constructor method `from_speudo` returns a Device interface iterable for a Master.
    #[allow(unused_variables)]
    pub fn from_speudo(master: pty::Master, pid: libc::pid_t) -> Self {
        let (tx_out, rx_out) = mpsc::sync_channel(0);
        let (tx_in, rx_in) = mpsc::sync_channel(0);

        thread::spawn(move || spawn::input(tx_in));
        thread::spawn(move || spawn::output(tx_out, master));
        match () {
            #[cfg(not(feature = "task"))]
            () => {
                Device {
                    delay: time::Duration::from_millis(TM_DEVICE),
                    input: rx_in,
                    output: rx_out,
                }
            },
            #[cfg(feature = "task")]
            () => {
                let (tx_task, rx_task) = mpsc::sync_channel(0);

                thread::spawn(move || spawn::task(time::Duration::from_millis(TM_TASK), tx_task, pid));
                Device {
                    delay: time::Duration::from_millis(TM_DEVICE),
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
        loop {
            return match self.output.try_recv() {
                Ok((_, 0)) => None,
                Ok((buf, len)) => Some(DeviceState::from_out(buf, len)),
                #[cfg(not(feature = "task"))]
                Err(_) => {
                    match self.input.recv_timeout(self.delay) {
                        Ok((buf, len)) => Some(DeviceState::from_in(buf, len)),
                        #[cfg(feature = "idle")]
                        _ => Some(DeviceState::from_idle()),
                        #[cfg(not(feature = "idle"))]
                        _ => continue ,
                    }
                },
                #[cfg(feature = "task")]
                Err(_) => {
                    match self.input.try_recv() {
                        Ok((buf, len)) => Some(DeviceState::from_in(buf, len)),
                        _ => {
                            match self.task.recv_timeout(self.delay) {
                                Ok(name) => Some(DeviceState::Proc(name)),
                                #[cfg(feature = "idle")]
                                _ => Some(DeviceState::from_idle()),
                                #[cfg(not(feature = "idle"))]
                                _ => continue ,
                            }
                        },
                    }
                },
            }
        }
    }
}
