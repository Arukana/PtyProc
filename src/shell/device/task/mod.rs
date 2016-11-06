mod err;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

pub use self::err::{ProcError, Result};

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;

/// The proc directory.
const SPEC_PROC: &'static str = "/proc";

/// The status's sub-directory.
const SPEC_SUBD_STATUS: &'static str = "status";

/// The default capacity of texel dictionary.
const SPEC_CAPACITY_PROC: usize = 255;
