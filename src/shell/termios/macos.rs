use ::libc;

use std::io::{self, Write};
use std::ops::BitOr;
use std::mem;
use std::fmt;

pub use super::err::TermiosError;

pub struct Termios {
    pub fd: libc::c_int,
    /// Save the original configuration of terminal.
    pub config: libc::termios,
}

impl Termios {

    /// The constructor method `new` setups the terminal.
    pub fn new(fd: libc::c_int) -> Result<Self, TermiosError> {
        unsafe {
            let config: libc::termios = mem::zeroed();

            if libc::ioctl(fd,
                           (0x40000000 | (116 << 8) | 19 |
                            (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
                            &config
                          ).eq(&-1) {
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

    fn enter_raw_mode(&self) -> Result<(), TermiosError> {
        unsafe {
            let mut new_termios: libc::termios = mem::zeroed();

            if libc::ioctl(self.fd,
                           (0x40000000 | (116 << 8) | 19 |
                            (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
                            &new_termios
                          ).eq(&-1) {
                Err(TermiosError::TcgGet)
            } else {
                new_termios.c_lflag ^= libc::ICANON | libc::ISIG | libc::ECHO;
                new_termios.c_lflag ^= libc::BRKINT;
                new_termios.c_cc[libc::VMIN] = 1;
                new_termios.c_cc[libc::VTIME] = 0;
                if libc::ioctl(self.fd,
                               (0x80000000 | (116 << 8) | 20 |
                                (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
                                &new_termios
                              ).eq(&-1) {
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
        Termios::new(libc::STDOUT_FILENO).expect(&format!("{}", ::errno::errno()))
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
            if io::stdout().write(super::SPEC_MOUSE_OFF)
                           .is_err()
                           .bitor(libc::ioctl(self.fd,
                                (0x80000000 | (116 << 8) | 20 | (((mem::size_of::<libc::termios>() & 0x1FFF) << 16) as u64)),
                                &self.config).eq(&-1))
                            .bitor(io::stdout().write(b"\x1B[?25h").is_err()) {
                panic!("{}", ::errno::errno());
            }
        }
    }
}
