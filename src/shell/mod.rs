pub mod display;
pub mod device;
pub mod state;
pub mod termios;
mod err;

use std::os::unix::io::AsRawFd;
use std::io::{self, Write};
use std::mem;
use std::fmt;

use ::libc;
use ::fork::Child;
use ::pty::prelude as pty;

use self::device::Device;
use self::termios::Termios;
pub use self::state::ShellState;
pub use self::err::{ShellError, Result};
use self::display::Display;

use self::display::winsz::Winszed;

/// The struct `Shell` is the speudo terminal interface.

pub struct Shell {
    pid: libc::pid_t,
    #[allow(dead_code)]
    config: Termios,
    speudo: pty::Master,
    device: Device,
    state: ShellState,
    screen: Display,
}

impl Shell {

  /// The constructor method `new` returns a shell interface according to
  /// the command's option and a configured mode Line by Line.
  pub fn new (
      repeat: Option<i64>,
      interval: Option<i64>,
      command: Option<&'static str>,
      windows: Option<Winszed>,
  ) -> Result<Self> {
      unsafe {
        let winsz: Winszed = windows.unwrap_or_else(|| Winszed::new(libc::STDIN_FILENO).unwrap());
        match pty::Fork::from_ptmx() {
              Err(cause) => Err(ShellError::ForkFail(cause)),
              Ok(fork) => match fork {
                  pty::Fork::Child(ref slave) => {
                      libc::ioctl(0, libc::TIOCSWINSZ, &winsz);
                      slave.exec(command.unwrap_or("/bin/zsh"))
                  },
                  pty::Fork::Parent(pid, master) => {
                      mem::forget(fork);
                      Ok(Shell {
                          pid: pid,
                          config: Termios::default(),
                          speudo: master,
                          device: Device::from_speudo(master, libc::getpid()),
                          state: ShellState::new(repeat, interval),
                          screen: Display::from_winszed(winsz),
                      })
                  },
              }
          }
      }}

    /// The accessor method `get_pid` returns the pid from the master.
    pub fn get_pid(&self) -> libc::pid_t {
        self.pid
    }

    /// The accessor method `get_screen` returns a reference on the Display interface.
    pub fn get_screen(&self) -> &Display {
        &self.screen
    }

    /// The mutator method `set_window_size` redimentionnes the window
    /// with a argument size.
    pub fn set_window_size_with(&mut self, size: &Winszed) {
        self.screen.set_window_size(size);
        unsafe {
            libc::ioctl(self.speudo.as_raw_fd(), libc::TIOCSWINSZ, size);
            libc::kill(self.pid, libc::SIGWINCH);
        }
    }

    /// The mutator method `set_window_size` redimentionnes the window
    /// with a default size.
    pub fn set_window_size(&mut self) {
        if let Ok(size) = Winszed::new(libc::STDOUT_FILENO) {
            self.set_window_size_with(&size);
        }
    }

    /// The mutator method `write_screen` set a buffer to the display
    /// without needing to print it
    pub fn write_screen(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.screen.write(buf)
    }
}

impl Iterator for Shell {
    type Item = ShellState;

    fn next(&mut self) -> Option<ShellState> {
        match self.device.next() {
            None => None,
            Some(event) => {
                self.state.clone_from(&mut self.screen, event);
                if self.state.is_signal_resized().is_some() {
                    self.set_window_size();
                }
                Some(self.state)
            },
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

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.screen)
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        unsafe {
            assert_ne!(libc::close(self.speudo.as_raw_fd()), -1);
            libc::kill(self.pid, libc::SIGKILL);
        }
    }
}
