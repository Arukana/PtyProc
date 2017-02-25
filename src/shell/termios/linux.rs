use ::libc;

use std::io::{self, Write};
use std::ops::BitOr;
use std::mem;
use std::fmt;

pub use super::err::{TermiosError, Result};

pub struct Termios {
    pub fd: libc::c_int,
    /// Save the original configuration of terminal.
    pub config: libc::termios,
}

impl Termios {

    /// The constructor method `new` setups the terminal.
    pub fn new(fd: libc::c_int) -> Result<Self> {
        unsafe {
            let config: libc::termios = mem::zeroed();

            if libc::ioctl(fd, 0x00005401, &config).eq(&-1) {
                Err(TermiosError::TcgGet)
            } else {
                let setup: Termios = Termios {
                    fd: fd,
                    config: config,
                };
                setup.enter_raw_mode().and(Ok(setup))
            }
        }
    }

    fn enter_raw_mode(&self) -> Result<()> {
        unsafe {
            let mut new_termios: libc::termios = mem::zeroed();

            if libc::ioctl(self.fd, 0x00005401, &new_termios).eq(&-1) {
                Err(TermiosError::TcgGet)
            } else {
                new_termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG);
                new_termios.c_iflag &= !(libc::BRKINT);
                new_termios.c_cc[libc::VMIN] = 1;
                new_termios.c_cc[libc::VTIME] = 0;
                if libc::ioctl(self.fd, 0x00005402, &new_termios).eq(&-1) {
                    Err(TermiosError::TcgSet)
                } else {
                    if io::stdout().write(super::SPEC_MOUSE_ON).is_err() {
                        Err(TermiosError::WriteMouseOn)
                    } else {
                        Ok(())
                    }
                }
            }
        }
    }
}

impl Default for Termios {
    fn default() -> Termios {
        Termios::new(
            libc::STDOUT_FILENO
            ).expect(
                &format!("{}", ::errno::errno())
                )
    }
}

impl fmt::Debug for Termios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Termios {{ fd: {:?} }}", self.fd)
    }
}

impl Drop for Termios {
    fn drop(&mut self) {
        unsafe {
            if io::stdout().write(super::SPEC_MOUSE_OFF).is_err().bitor(
                libc::ioctl(self.fd, 0x00005402, &self.config).eq(&-1)
                ) {
                panic!("{}", ::errno::errno());
            }
        }
    }
}
