pub mod display;
pub mod device;
pub mod state;
pub mod mode;
pub mod termios;
mod err;

use std::os::unix::io::AsRawFd;
use std::io::{self, Write};
use std::mem;

use ::libc;
use ::fork::Child;
use ::pty::prelude as pty;

use self::mode::Mode;
use self::device::Device;
use self::termios::Termios;
use self::state::clone::Clone;
pub use self::state::ShellState;
pub use self::err::{ShellError, Result};
pub use self::display::Display;

/// The struct `Shell` is the speudo terminal interface.

pub struct Shell {
  pid: libc::pid_t,
  mode: Mode,
  #[allow(dead_code)]
  config: Termios,
  speudo: pty::Master,
  device: Device,
  state: ShellState,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Winszed {
  /// Rows, in characters.
  pub ws_row: libc::c_ushort,
  /// Columns, in characters.
  pub ws_col: libc::c_ushort,
  /// Horizontal size, pixels.
  pub ws_xpixel: libc::c_ushort,
  /// Vertical size, pixels.
  pub ws_ypixel: libc::c_ushort,
}

impl Shell {

  /// The constructor method `new` returns a shell interface according to
  /// the command's option and a configured mode Line by Line.
  pub fn new (
      repeat: Option<i64>,
      interval: Option<i64>,
      command: Option<&'static str>,
  ) -> Result<Self> {
    Shell::from_mode(repeat, interval, command, Mode::None)
  }

  /// The constructor method `from_mode` returns a shell interface according to
  /// the command's option and the mode.
    pub fn from_mode (
      repeat: Option<i64>,
      interval: Option<i64>,
      command: Option<&'static str>,
      mode: Mode,
    ) -> Result<Self> {
    unsafe {
    let winsz: Winszed = Winszed::default();
    libc::ioctl(0, libc::TIOCGWINSZ, &winsz);
  
    match pty::Fork::from_ptmx() {
      Err(cause) => Err(ShellError::ForkFail(cause)),
      Ok(fork) => match fork {
        pty::Fork::Child(ref slave) =>
         { libc::ioctl(0, libc::TIOCSWINSZ, &winsz);
           slave.exec(command.unwrap_or("/bin/bash")) },
        pty::Fork::Parent(pid, master) => {
        mem::forget(fork);
          Ok(Shell {
            pid: pid,
            config: Termios::default(),
            mode: mode,
            speudo: master,
            device: Device::from_speudo(master),
            state: ShellState::new(repeat, interval, libc::STDOUT_FILENO),
          })
        },
      },
    }
  }}

  /// The accessor method `get_pid` returns the pid from the master.
  pub fn get_pid(&self) -> &libc::pid_t {
    &self.pid
  }

  /// The method `set_mode` changes the terminal mode.
  pub fn set_mode(&mut self, mode: Mode) {
    self.mode = mode;
  }

    /// The method `mode_pass` sends the input to the speudo terminal
    /// if the mode was defined with a procedure.
    fn mode_pass (
        &mut self,
        state: &ShellState,
    ) {
        if self.mode == Mode::Character {

          if let Some(ref text) = state.is_input_slice() {
             self.write(text).unwrap();
             self.flush().unwrap();
          }
       }
    }
}

impl Write for Shell {
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
      if libc::close(self.speudo.as_raw_fd()).eq(&-1) {
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
          self.state.clone_from(event);
          let state: ShellState = self.state.clone();
          self.mode_pass(&state);
          if state.is_signal_resized().is_some() {
              unsafe {
                  libc::kill(self.pid, libc::SIGWINCH);
              }
          }
          Some(state)
      },
    }
  }
}
