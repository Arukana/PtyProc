use std::fmt;
use std::error::Error;

/// The alias `Result` learns `WinszedError` possibility.

pub type Result<T> = ::std::result::Result<T, WinszedError>;

/// The enum `WinszedError` defines the possible errors from constructor Winszed.

#[derive(Clone, Copy, Debug)]
pub enum WinszedError {
  /// TIOCSWINSZED ioctl.
  GsFail,
  /// TIOCGWINSZED ioctl.
  GwFail,
}

impl fmt::Display for WinszedError {

  /// The function `fmt` formats the value using the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for WinszedError {

  /// The function `description` returns a short description of the error.
  fn description(&self) -> &str {
    match *self {
      WinszedError::GsFail => "The TIOCSWINSZED ioctl has occured an error",
      WinszedError::GwFail => "The TIOCGWINSZED ioctl has occured an error",
    }
  }

  /// The function `cause` returns the lower-level cause of this error, if any.
  fn cause(&self) -> Option<&Error> {
    None
  }
}
