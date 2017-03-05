use std::fmt;
use std::io;

use ::pty::prelude as pty;
use ::libc;

use super::shell::display::winsz::Winszed;
use super::shell::display::Display;
use super::shell::state::ShellState;

pub trait Parent : Iterator<Item=ShellState> + io::Write + fmt::Display + fmt::Debug + Drop {
    fn get_pid(&self) -> libc::pid_t; 
    fn get_speudo(&self) -> &pty::Master;
    fn get_screen(&self) -> &Display;
    fn get_window_size(&self) -> &Winszed;
    fn set_window_size(&mut self);
    fn set_window_size_with(&mut self, size: &Winszed);
}
