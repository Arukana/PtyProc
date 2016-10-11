use std::error::Error;
use std::fmt;

use super::mouse::MouseError;

pub type Result<T> = ::std::result::Result<T, OperateError>;

/// The enum `OperateError` defines the possible errors from constructor Operate.

#[derive(Clone, Copy, Debug)]
pub enum OperateError {
  /// There isn't ';' coordinate separator found.
  PositionNotFound,
  /// The number can't get parsed.
  FromStrFail,
  /// Isn't a mouse pattern.
  NotMouse,
  /// The Mouse as meet an error.
  MouseFail(MouseError),
}

impl fmt::Display for OperateError {

   /// The function `fmt` formats the value using the given formatter.
   fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
   }
}

impl Error for OperateError {

    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            OperateError::PositionNotFound => "There isn't ';' coordinate separator found.",
            OperateError::FromStrFail => "The number can't get parsed.",
            OperateError::NotMouse => "Isn't a mouse pattern.",
            OperateError::MouseFail(_) => "The Mouse as meet an error.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            OperateError::MouseFail(ref why) => Some(why),
            _ => None,
        }
    }
}
