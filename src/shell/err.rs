use ::pty::prelude::ForkError;
use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, ShellError>;

/// The enum `ShellError` defines the possible errors from constructor Shell.

#[derive(Clone, Copy, Debug)]
pub enum ShellError {
  BadFork(ForkError),
  BadIoctl,
}


impl fmt::Display for ShellError {

   /// The function `fmt` formats the value using the given formatter.
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", self)
   }
}

impl Error for ShellError {

    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            ShellError::BadFork(_) => "Winszed interface has occured an error.",
            ShellError::BadIoctl => "ioctl(2) has occured an error.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            ShellError::BadFork(ref err) => Some(err),
            _ => None,
        }
    }
}
