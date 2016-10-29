pub mod err;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

/// The constante `SPEC_MOUSE_OFF` is the terms to mute the mouse.
const SPEC_MOUSE_OFF: &'static [u8; 24] = b"\x1b[?1015l\x1b[?1002l\x1b[?1000l";
/// The constante `SPEC_MOUSE_OFF` is the terms to listen the mouse.
#[cfg(target_os = "linux")]
const SPEC_MOUSE_ON: &'static [u8; 24] = b"\x1b[?1002h\x1b[?1015h\x1b[?1006h";
#[cfg(target_os = "macos")]
const SPEC_MOUSE_ON: &'static [u8; 24] = b"\x1b[?1002h\x1b[?1015h\x1b[?1006h";
