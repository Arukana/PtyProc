use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, CodeError>;

/// The enum `CodeError` defines the possible errors from constructor Code.

#[derive(Clone, Copy, Debug)]
pub enum CodeError {
  /// The pattern isn't defined.
  NotImplemented,
}

impl fmt::Display for CodeError {

   /// The function `fmt` formats the value using the given formatter.
   fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
   }
}

impl Error for CodeError {

    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            CodeError::NotImplemented => "The pattern isn't defined.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
