mod err;

use std::ops::Mul;

use ::libc;
pub use self::err::{WinszedError, Result};

/// The enum `Winszed` is the size of the tty window.

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Winszed {
    /// Rows, in characters.
    pub ws_row: libc::c_ushort,
    /// Columns, in characters.
    pub ws_col: libc::c_ushort,
    /// Horizontal size, pixels.
    pub ws_xpixel: libc::c_ushort,
    /// Vertical size, pixels.
    pub ws_ypixel: libc::c_ushort, 
}

impl Winszed {

    /// The constructor method `new` returns the window size.
    pub fn new(fd: libc::c_int) -> Result<Self> {
        unsafe {
            let winsz: Winszed = Winszed::default();

            match libc::ioctl(fd, libc::TIOCGWINSZ, &winsz) {
                -1 => Err(WinszedError::GwFail),
                _ => Ok(winsz),
            }
        }
    }

    /// The accessor function `get_row` returns the number of rows.
    pub fn get_row(&self) -> libc::size_t {
      self.ws_row as libc::size_t
    }

    /// The accessor function `get_col` returns the number of columns.
    pub fn get_col(&self) -> libc::size_t {
      self.ws_col as libc::size_t
    }

    /// The accessor function `get_irow` returns the number of rows.
    pub fn get_irow(&self) -> libc::ssize_t {
        self.ws_row as libc::ssize_t
    }

    /// The accessor function `get_icol` returns the number of columns.
    pub fn get_icol(&self) -> libc::ssize_t {
        self.ws_col as libc::ssize_t
    }

    /// The accessor function `get_row` returns the multiplication
    /// of row by colum.
    pub fn row_by_col(&self) -> libc::size_t {
        self.get_row().mul(&self.get_col())
    }


    /// The accessor function `get_ref_row` returns a ref on the number of rows.
    pub fn get_ref_row(&mut self) -> &mut libc::c_ushort {
        &mut self.ws_row
    }

    /// The accessor function `get_xpixel` returns the horizontal size.
    pub fn get_xpixel(&self) -> libc::c_uint {
        self.ws_xpixel as libc::c_uint
    }

    /// The accessor function `get_ypixel` returns the vertical size.
    pub fn get_ypixel(&self) -> libc::c_uint {
        self.ws_ypixel as libc::c_uint
    }

    /// The method `from_winsized` changes the window size.
    #[allow(dead_code)]
    pub fn from_winsized(fd: libc::c_int, winsize: &Winszed) -> Result<()> {
        unsafe {
            match libc::ioctl(fd, libc::TIOCSWINSZ, winsize) {
                -1 => Err(WinszedError::GsFail),
                _ => Ok(()),
            }
        }
    }
}
