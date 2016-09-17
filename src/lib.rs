#![feature(slice_patterns, integer_atomics)]

#[macro_use(chan_select)] extern crate chan;
extern crate pty;
extern crate libc;
extern crate time;
//extern crate termios;
extern crate termion;

const DISPLAY_DEFAULT_COL: libc::c_ushort = 80;
const DISPLAY_DEFAULT_ROW: libc::c_ushort = 24;

mod winsize;
pub mod terminal;

pub mod shell;
pub mod prelude;
mod fork;
