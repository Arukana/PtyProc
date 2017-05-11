use super::winsz::WinszedError;

use std::fmt;
use std::error::Error;

/// The enum `DisplayError` defines the possible errors from constructor Display.

#[derive(Clone, Copy, Debug)]
pub enum DisplayError {
    /// Winszed has occured an error.
    WinszedFail(WinszedError),
}

impl fmt::Display for DisplayError {

   /// The function `fmt` formats the value using the given formatter.
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", self)
   }
}

impl Error for DisplayError {


    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            DisplayError::WinszedFail(_) => "Winszed interface has occured an error.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            DisplayError::WinszedFail(ref err) => Some(err),
        }
    }
}
