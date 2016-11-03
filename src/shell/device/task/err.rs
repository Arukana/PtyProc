use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, ProcError>;

/// The enum `ProcError` defines the possible errors
/// from constructor Proc.
#[derive(Debug)]
pub enum ProcError {
    /// Can't read the sub-directory.
    ReadDir(io::Error),
}

impl fmt::Display for ProcError {
  /// The function `fmt` formats the value using
  /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
    }
}

impl Error for ProcError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
      match *self {
          ProcError::ReadDir(_) => "Can't read the sub-directory.",
      }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
      match *self {
          ProcError::ReadDir(ref why) => Some(why),
      }
  }
}
