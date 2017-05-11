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
use ::child::exec;
use ::pty::prelude as pty;

use self::device::Device;
use self::termios::Termios;
pub use self::state::ShellState;
pub use self::err::ShellError;
use self::display::Display;
pub use self::display::winsz::Winszed;

pub use super::parent::Parent;

/// The struct `Shell` is the speudo terminal interface.

#[derive(Debug)]
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
      command: Option<&str>,
      windows: Option<Winszed>,
  ) -> Result<Self, ShellError> {
      unsafe {
            let winsz: Winszed =
                windows.and_then(|winsz| {
                    let _ = Winszed::from_winsized(libc::STDIN_FILENO, &winsz);
                    Some(winsz)
                })
                .or_else(|| Winszed::new(libc::STDIN_FILENO).ok())
                .unwrap_or_default();
            match pty::Fork::from_ptmx() {
                Err(cause) => Err(ShellError::ForkFail(cause)),
                Ok(fork) => match fork {
                    pty::Fork::Child(_) => {
                        libc::ioctl(libc::STDIN_FILENO, libc::TIOCSWINSZ, &winsz);
                        exec(command.unwrap_or("/bin/bash"))
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
        }
    }
}

impl Parent for Shell {

    /// The accessor method `get_pid` returns the pid from the master.
    fn get_pid(&self) -> libc::pid_t {
        self.pid
    }

    /// The accessor method `get_speudo` returns the master interface.
    fn get_speudo(&self) -> &pty::Master {
        &self.speudo
    }

    /// The accessor method `get_screen` returns a reference on the Display interface.
    fn get_screen(&self) -> &Display {
        &self.screen
    }

    /// The accessor method `get_window_size` returns the window size.
    fn get_window_size(&self) -> &Winszed {
        self.screen.get_window_size()
    }

    /// The mutator method `set_window_size` redimentionnes the window
    /// with a default size.
    fn set_window_size(&mut self) {
        if let Ok(size) = Winszed::new(libc::STDOUT_FILENO) {
            self.set_window_size_with(&size);
        }
    }

    /// The mutator method `set_window_size` redimentionnes the window
    /// with a argument size.
    fn set_window_size_with(&mut self, size: &Winszed) {
        self.screen.set_window_size(size);
        unsafe {
            libc::ioctl(self.speudo.as_raw_fd(), libc::TIOCSWINSZ, size);
            libc::kill(self.pid, libc::SIGWINCH);
        }
    }

    /// The mutator method `write` set a buffer to the display
    /// without needing to print it
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.screen.write(buf)
    }

    /// The mutator method `next` updates the event and returns
    /// the new state.
    fn next(&mut self, event: state::DeviceState) -> ShellState {
        match () {
            #[cfg(feature = "auto-resize")]
            () => {
                self.state.update_from(&mut self.screen, event);
                if let Some(size) = self.state.is_resized() {
                    self.set_window_size_with(&size);
                }
                self.state
            },
            #[cfg(not(feature = "auto-resize"))]
            () => {
                self.state.update_from(&mut self.screen, event);
                self.state
            },
        }
    }
}

impl Iterator for Shell {
    type Item = ShellState;

    fn next(&mut self) -> Option<ShellState> {
        match self.device.next() {
            None => None,
            #[cfg(feature = "auto-resize")]
            Some(event) => {
                self.state.update_from(&mut self.screen, event);
                if let Some(size) = self.state.is_resized() {
                    self.set_window_size_with(&size);
                }
                Some(self.state)
            },
            #[cfg(not(feature = "auto-resize"))]
            Some(event) => {
                self.state.update_from(&mut self.screen, event);
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

impl Default for Shell {
    fn default() -> Shell {
        unsafe {
            let master: pty::Master = mem::zeroed();

            Shell {
                pid: 0,
                config: mem::zeroed(),
                speudo: master,
                device: Device::from_speudo(master, 0),
                state: ShellState::default(),
                screen: Display::default(),
            }
        }
    }
}
