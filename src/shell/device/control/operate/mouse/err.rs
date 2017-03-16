use std::error::Error;
use std::fmt;

use super::code::CodeError;

pub type Result<T> = ::std::result::Result<T, MouseError>;

/// The enum `CodeError` defines the possible errors from constructor Code.

#[derive(Clone, Copy, Debug)]
pub enum MouseError {
    /// The Mouse as meet an error.
    Code(CodeError),
    /// There isn't ';' coordinate separator found.
    PositionNotFound,
    /// The number can't get parsed.
    FromStrFail,
    /// Isn't a mouse pattern.
    Other,
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
            MouseError::Code(_) => "The code has occured an error.",
            MouseError::PositionNotFound => "There isn't ';' coordinate separator found.",
            MouseError::FromStrFail => "The number can't get parsed.",
            MouseError::Other => "Isn't a mouse pattern.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            MouseError::Code(ref why) => why.cause(),
            _ => None,
        }
    }
}
