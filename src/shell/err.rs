use ::pty::prelude::ForkError;
use std::error::Error;
use std::fmt;

use super::termios::TermiosError;

pub type Result<T> = ::std::result::Result<T, ShellError>;

/// The enum `ShellError` defines the possible errors from constructor Shell.

#[derive(Clone, Copy, Debug)]
pub enum ShellError {
    ForkFail(ForkError),
    TermiosFail(TermiosError),
    NotFound,
}


impl fmt::Display for ShellError {

    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for ShellError {

    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            ShellError::ForkFail(_) => "The pseudo tty has occured an error.",
            ShellError::TermiosFail(_) => "The termios has occured an error.",
            ShellError::NotFound => "The $SHELL variable of environement \
                                     was empty during the compile time",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            ShellError::ForkFail(ref err) => Some(err),
            ShellError::TermiosFail(ref err) => Some(err),
            _ => None,
        }
    }
}
