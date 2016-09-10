#[macro_use(chan_select)]
extern crate chan;
extern crate pty;
extern crate libc;
extern crate termios;
extern crate termion;
extern crate itertools;

const DISPLAY_DEFAULT_COL: libc::c_ushort = 80;
const DISPLAY_DEFAULT_ROW: libc::c_ushort = 24;

mod winsize;
pub mod terminal;

mod ffi;
pub mod shell;
pub mod prelude;
mod fork;
