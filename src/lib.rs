#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![feature(range_contains)]

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]

#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file="clippy.toml")))]

#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]
#![deny(
    unused_import_braces,
)]

extern crate pty;
extern crate libc;
extern crate time;
extern crate errno;

#[macro_use] mod macros;
mod fork;
pub mod shell;
pub mod prelude;
