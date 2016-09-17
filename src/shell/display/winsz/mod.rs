mod err;

use ::libc;
pub use self::err::{WinszedError, Result};

/// The enum `Winszed` is the size of the tty window.

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

impl Winszed {

  /// The constructor method `new` returns the window size.
  pub fn new(fd: libc::c_int) -> Result<Self> {
    unsafe {
      let winsz: Winszed = Winszed::default();

      match libc::ioctl(fd, libc::TIOCGWINSZ, &winsz) {
        -1 => Err(WinszedError::GwFail),
        _ => Ok(winsz),
      }
    }
  }

  pub fn get_row(&self) -> libc::size_t {
    self.ws_row as libc::size_t
  }

  pub fn get_col(&self) -> libc::size_t {
    self.ws_col as libc::size_t
  }

  /// The method `from_winsized` changes the window size.
  #[allow(dead_code)]
  pub fn from_winsized(fd: libc::c_int, winsize: &Winszed) -> Result<()> {
    unsafe {
      match libc::ioctl(fd, libc::TIOCSWINSZ, winsize) {
        -1 => Err(WinszedError::GsFail),
        _ => Ok(()),
      }
    }
  }
}
