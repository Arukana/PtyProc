mod err;
mod display;
mod device;
pub mod state;

use std::collections::VecDeque;
use std::{fmt, io, mem};
use std::io::{Read, Write};

use ::libc;
use ::fork::Child;
use ::pty::prelude as pty;

use self::device::{Device, DeviceState};
pub use self::state::ShellState;
pub use self::err::{ShellError, Result};
pub use self::display::Display;

/// The struct `Shell` is the speudo terminal interface.

pub struct Shell {
  pid: libc::pid_t,
  device: Device,
  display: Display,
}

impl Shell {
  pub fn new (
    command: Option<&'static str>,
  ) -> Result<Self> {
    match pty::Fork::from_ptmx() {
      Err(cause) => Err(ShellError::BadFork(cause)),
      Ok(fork) => match fork {
        pty::Fork::Child(ref slave) => {
          slave.exec(command.unwrap_or("bash"));
        },
        pty::Fork::Parent(pid, master) => {
          unsafe {
            mem::forget(fork);
          }
          Ok(Shell {
            pid: pid,
            device: Device::from_speudo(master),
            display: Display::default(),
          })
        },
      },
    }
  }
}




impl Iterator for Shell {
  type Item = ShellState;

  fn next(&mut self) -> Option<ShellState> {
    None
  }
}
/*
    match self.device.next() {
      None => None,
      Some((rx_in, rx_out)) => {
        if let Some((rx_out_buf, rx_out_len)) = rx_out {
          self.log.extend(Vec::from(&rx_out_buf[..rx_out_len]));
          io::stdout().write(&rx_out_buf[..rx_out_len]).unwrap();
          io::stdout().flush().unwrap();
        };
        match rx_in {
          None => Some(State::default()),
          Some(10) | Some(13) => {
            let ll = self.line.clone();

            self.line.clear();
            Some(State(Some(ll), Some(10)))
          },
          Some(key) => {
            self.line.push(key);
            Some(State(None, Some(key)))
          },
        }
      },
    }
  }
}

impl io::Write for Shell {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.device.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.device.flush()
  }
}*/
