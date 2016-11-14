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
pub use self::display::Display;

use self::display::winsz::Winszed;

/// The struct `Shell` is the speudo terminal interface.

pub struct Shell {
  child_fd: libc::c_int,
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
    //        println!("FD::{}", child_fd);

            // Cas d'erreur si le fd est inférieur ou égal à 2
            // child_fd.gt(&2).except("Can't get file desciptor of the child");

          Ok(Shell {
            child_fd: child_fd,
            pid: pid,
            config: Termios::default(),
            speudo: master,
            device: Device::from_speudo(master),
            state: ShellState::new(repeat, interval),
            screen: Display::new(libc::STDOUT_FILENO).unwrap(),
          })
        },
      },
    }
  }}

  /// The accessor method `get_pid` returns the pid from the master.
  pub fn get_pid(&self) -> &libc::pid_t {
    &self.pid
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
          self.state.clone_from(&mut self.screen, event);
          if self.state.is_signal_resized().is_some() {
              unsafe
              { // Manually set child size
                let winsz: Winszed = Winszed::default();
                libc::ioctl(0, libc::TIOCGWINSZ, &winsz);
                libc::ioctl(self.child_fd, libc::TIOCSWINSZ, &winsz);

                libc::kill(self.pid, libc::SIGWINCH); }
          }
          Some(self.state)
      },
    }
  }
}
