#![feature(slice_patterns, integer_atomics)]
#![feature(advanced_slice_patterns)]

#[macro_use(chan_select)] extern crate chan;
extern crate pty;
extern crate libc;
extern crate time;
extern crate errno;

pub mod terminal;
pub mod shell;
pub mod prelude;
mod fork;
