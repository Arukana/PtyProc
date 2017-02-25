use std::fmt;
use std::io;

use ::pty::prelude as pty;
use ::libc;

use super::shell::display::winsz::Winszed;
use super::shell::display::Display;

pub trait Parent : Default + Iterator + io::Write + fmt::Display + fmt::Debug + Drop {
    fn write_screen(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn get_pid(&self) -> libc::pid_t; 
    fn get_speudo(&self) -> &pty::Master;
    fn get_screen(&self) -> &Display;
    fn get_window_size(&self) -> &Winszed;
    fn set_window_size(&mut self);
    fn set_window_size_with(&mut self, size: &Winszed);
}
