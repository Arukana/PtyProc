use ::libc;

use std::io::Result;
use std::io::{self, Write};
use std::mem;

pub struct Termios {
  fd: libc::c_int,
  config: libc::termios,
}

impl Termios {

  /// The constructor method `new` setups the terminal.
  pub fn new(fd: libc::c_int) -> Result<Self> {
    unsafe {
      let mut config: libc::termios = mem::zeroed();

      libc::ioctl(fd, 0x00005401, &config);

      let setup: Termios = Termios {
        fd: fd,
        config: config,
      };
      try!(setup.enter_raw_mode());
      Ok(setup)
    }
  }

  fn enter_raw_mode(&self) -> Result<()> {
    unsafe {
      let mut new_termios: libc::termios = mem::zeroed();

      libc::ioctl(self.fd, 0x00005401, &new_termios);
      new_termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
      new_termios.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
      new_termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
      new_termios.c_cflag |= libc::CS8;
      new_termios.c_oflag &= !(libc::OPOST);
      new_termios.c_cc[libc::VMIN] = 1;
      new_termios.c_cc[libc::VTIME] = 0;
      libc::ioctl(self.fd, 0x00005402, &new_termios);
      io::stdout().write(b"\x1b[?1002h\x1b[?1015h\x1b[?1006h").unwrap(); // MOUSE ON
      Ok(())
    }
  }
}

impl Default for Termios {
  fn default() -> Termios {
    Termios::new(
      libc::STDIN_FILENO
    ).expect(
      &format!("{}", ::errno::errno())
    )
  }
}

impl Drop for Termios {
  fn drop(&mut self) {
    unsafe {
      if libc::ioctl(self.fd, 0x00005402, &self.config)
              .eq(&-1) {
        panic!("{}", ::errno::errno());
      }
    }
  }
}
