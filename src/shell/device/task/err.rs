use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, ProcError>;

/// The enum `ProcError` defines the possible errors
/// from constructor Proc.
#[derive(Debug)]
pub enum ProcError {
    /// Can't read the sub-directory.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    ReadDir(io::Error),
    /// Can't count the number of process.
    #[cfg(target_os = "macos")]
    ListAllPidLen,
    /// There isn't a valid number of process.
    #[cfg(target_os = "macos")]
    ListAllPidLenUnvalid,
}

impl fmt::Display for ProcError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for ProcError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
      match *self {
          #[cfg(any(target_os = "linux", target_os = "android"))]
          ProcError::ReadDir(_) => "Can't read the sub-directory.",
          #[cfg(target_os = "macos")]
          ProcError::ListAllPidLen => "Can't count the number of process.",
          #[cfg(target_os = "macos")]
          ProcError::ListAllPidLenUnvalid => "There isn't a valid number of process.",
      }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
      match *self {
          #[cfg(any(target_os = "linux", target_os = "android"))]
          ProcError::ReadDir(ref why) => Some(why),
          #[cfg(target_os = "macos")]
          _ => None,
      }
  }
}
