#![feature(slice_patterns)]

#[macro_use(chan_select)] extern crate chan;
#[macro_use] extern crate sig;
extern crate pty;
extern crate libc;
extern crate time;
extern crate termios;
extern crate termion;

const DISPLAY_DEFAULT_COL: libc::c_ushort = 80;
const DISPLAY_DEFAULT_ROW: libc::c_ushort = 24;

mod winsize;
pub mod terminal;

pub mod shell;
pub mod prelude;
mod fork;
