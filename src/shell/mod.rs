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
  child_fd: libc::c_int,
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
    let mut pipefd: Vec<i32> = Vec::with_capacity(2);
    let mut coucou = pipefd.as_ptr() as *mut i32;
    libc::pipe(coucou);
    pipefd = Vec::from_raw_parts(coucou, 2, pipefd.capacity());

    match pty::Fork::from_ptmx() {
      Err(cause) => Err(ShellError::ForkFail(cause)),
      Ok(fork) => match fork {
        pty::Fork::Child(ref slave) =>
         {
            // Child window init
            libc::ioctl(0, libc::TIOCSWINSZ, &winsz);
            
            // Use pipe
            libc::close(pipefd[0]);

            // Max path length = 1024
            let mut the: Vec<u8> = Vec::with_capacity(1024);
            let mut bonjour = the.as_ptr() as *mut libc::c_void;
            
            // Get info about /dev/tty of the child
            libc::fcntl(libc::STDOUT_FILENO, 50, bonjour);

            // Transfer it to master
            libc::write(pipefd[1], bonjour, 1024);
            libc::close(pipefd[1]);

            // Enter the shell exec
            //slave.exec(command.unwrap_or("/Users/jpepin/work42/minishell/21sh")) },
            slave.exec(command.unwrap_or("/bin/bash")) },

        pty::Fork::Parent(pid, master) => {
        mem::forget(fork);
            
            // Use pipe
            libc::close(pipefd[1]);
            let mut get: Vec<u8> = Vec::with_capacity(1024);
            let mut buf = get.as_ptr() as *mut libc::c_void;

            // Receive the /dev/tty of the child
            libc::read(pipefd[0], buf, 1024);
            libc::close(pipefd[0]);
            get = Vec::from_raw_parts(buf as *mut u8, 1024, get.capacity());
            let mut buf = get.as_ptr() as *const i8;
            
            // DEBUG : Ici 'get' contient le /dev/tty du child
            // print!("MASTER::");
            // for i in get
            // { print!("{}", i as char); }
            // println!("");
            // DEBUG

            // Collect the file desciptor of the child
            let child_fd = libc::open(buf, libc::O_RDWR);
            println!("FD::{}", child_fd);

            // Cas d'erreur si le fd est inférieur ou égal à 2
            // child_fd.gt(&2).except("Can't get file desciptor of the child");

          Ok(Shell {
            child_fd: child_fd,
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
              unsafe
              { // Manually set child size
                let winsz: Winszed = Winszed::default();
                libc::ioctl(0, libc::TIOCGWINSZ, &winsz);
                libc::ioctl(self.child_fd, libc::TIOCSWINSZ, &winsz);

                libc::kill(self.pid, libc::SIGWINCH); }
          }
          Some(state)
      },
    }
  }
}
