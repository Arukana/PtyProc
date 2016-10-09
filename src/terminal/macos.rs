use ::libc;

use std::io::Result;
use std::mem;
use std::io::{self, Write};

pub struct Termios {
  fd: libc::c_int,
  config: libc::termios,
}

impl Termios {
  /// The constructor method `new` setups the terminal.
  pub fn new(fd: libc::c_int) -> Result<Self> {
    unsafe {
      let config: libc::termios = mem::zeroed();

      libc::ioctl(
        fd,
        (0x40000000 | (116 << 8) | 19 |
        (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
        &config
      );
      
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

      libc::ioctl(
        self.fd,
        (0x40000000 | (116 << 8) | 19 |
        (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
        &new_termios
      );
      new_termios.c_lflag ^= !(libc::ICANON);
      new_termios.c_lflag ^= !(libc::ECHO);
      new_termios.c_cc[libc::VMIN] = 1;
      new_termios.c_cc[libc::VTIME] = 0;
      libc::ioctl(
        self.fd,
        (0x80000000 | (116 << 8) | 20 |
        (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
        &new_termios
      );
      io::stdout().write(super::SPEC_MOUSE_ON).unwrap(); // MOUSE ON
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
      if io::stdout().write(super::SPEC_MOUSE_OFF).is_err().bitor(
        libc::ioctl(
          self.fd,
          (0x80000000 | (116 << 8) | 20 |
          (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
          &self.config
        ).eq(&-1)
      ) {
        panic!("{}", ::errno::errno());
      }
    }
  }
}
