#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![feature(integer_atomics)]
#![feature(range_contains)]

#[macro_use(chan_select)] extern crate chan;
extern crate pty;
extern crate libc;
extern crate time;
extern crate errno;

#[macro_use]
mod macros;
mod fork;
pub mod shell;
pub mod prelude;
