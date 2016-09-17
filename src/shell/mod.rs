mod err;
mod display;
mod state;
pub mod device;

use std::os::unix::io::AsRawFd;
use std::io::{self, Write};
use std::ops::BitOr;
use std::mem;

use ::libc;
use ::fork::Child;
use ::pty::prelude as pty;

use self::device::Device;
pub use self::state::ShellState;
pub use self::err::{ShellError, Result};
pub use self::display::Display;

/// The struct `Shell` is the speudo terminal interface.

pub struct Shell {
  pid: libc::pid_t,
  speudo: pty::Master,
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
          if ::terminal::setup_terminal(master.as_raw_fd()).is_err() {
            Err(ShellError::BadIoctl)
          }
          else {
            Ok(Shell {
              pid: pid,
              speudo: master,
              device: Device::from_speudo(master),
              state: ShellState::new(master.as_raw_fd()),
            })
          }
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
    self.speudo.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.speudo.flush()
  }
}

impl Drop for Shell {
  fn drop(&mut self) {
    unsafe {
      ::terminal::restore_termios();
      if io::stdout().write(
        "\x1b[?1015l\x1b[?1002l\x1b[?1000l".as_bytes()
      ).is_err().bitor(
        libc::close(self.speudo.as_raw_fd()).eq(&-1)
      ) {
        unimplemented!()
      }
    }
  }
}

impl Iterator for Shell {
  type Item = ShellState;

  fn next(&mut self) -> Option<ShellState> {
    match self.device.next() {
      None => None,
      Some(event) => {
        if let Some(state) = self.state.with_device(event, self.speudo.as_raw_fd()).ok() {
          Some(state)
        }
        else {
          None
        }
      },
    }
  }
}
