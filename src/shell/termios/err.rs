use std::error::Error;
use std::fmt;

/// The enum `TermiosError` defines the possible errors from constructor Termios.

#[derive(Clone, Copy, Debug)]
pub enum TermiosError {
    TcgGet,
    TcgSet,
    WriteMouseOn,
}

impl fmt::Display for TermiosError {

   /// The function `fmt` formats the value using the given formatter.
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", ::errno::errno())
   }
}

impl Error for TermiosError {

    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            TermiosError::TcgGet => "ioctl(2) TCGETS has occured an error.",
            TermiosError::TcgSet => "ioctl(2) TCSETS has occured an error.",
            TermiosError::WriteMouseOn => "Can't write the MouseOn term.",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}
