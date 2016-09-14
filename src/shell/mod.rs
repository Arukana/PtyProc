mod err;
mod display;
mod device;
pub mod state;

use std::{io, mem};

use ::libc;
use ::fork::Child;
use ::pty::prelude as pty;

pub use self::device::{Device, DeviceState};
pub use self::state::ShellState;
pub use self::err::{ShellError, Result};
pub use self::display::Display;

/// The struct `Shell` is the speudo terminal interface.

pub struct Shell {
  pid: libc::pid_t,
  device: Device,
  state: ShellState,
}

impl Shell {

  /// The constructor method `new` returns a shell interface according to 
  /// the command's option.
  pub fn new (
    command: Option<&'static str>,
  ) -> Result<Self> {
    match pty::Fork::from_ptmx() {
      Err(cause) => Err(ShellError::BadFork(cause)),
      Ok(fork) => match fork {
        pty::Fork::Child(ref slave) => slave.exec(command.unwrap_or("bash")),
        pty::Fork::Parent(pid, master) => {
          mem::forget(fork);
          Ok(Shell {
            pid: pid,
            device: Device::from_speudo(master),
            state: ShellState::default(),
          })
        },
      },
    }
  }

  /// The accessor method `get_pid` returns the pid from the master.
  pub fn get_pid(&self) -> &libc::pid_t {
    &self.pid
  }
}

impl io::Write for Shell {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.device.write(buf)
  }
  fn flush(&mut self) -> io::Result<()> {
    self.device.flush()
  }
}

impl Iterator for Shell {
  type Item = ShellState;

  fn next(&mut self) -> Option<ShellState> {
    match self.device.next() {
      None => None,
      Some(event) => {
        if self.state.with_device(event).is_err() {
          None
        }
        else {
          Some(self.state.clone())
        }
      },
    }
  }
}
