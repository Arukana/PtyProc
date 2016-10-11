use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, MouseError>;

/// The enum `MouseError` defines the possible errors from constructor Mouse.

#[derive(Clone, Copy, Debug)]
pub enum MouseError {
  /// The pattern isn't defined.
  NotImplemented,
}

impl fmt::Display for MouseError {

   /// The function `fmt` formats the value using the given formatter.
   fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
   }
}

impl Error for MouseError {

    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            MouseError::NotImplemented => "The pattern isn't defined.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
