pub mod clone;

pub const DEFAULT_KEY_REPEAT: libc::c_long = 1_000_000_000i64;

use std::io::Write;
use std::ops::BitOr;

use ::libc;

use super::Display;
use super::device::control::Control;

use self::clone::Clone;
pub use super::device::{Out, DeviceState};
pub use super::device::control::operate::key::Key;
pub use super::device::control::operate::mouse::Mouse;

pub struct ShellState {
  /// Key repeat interval time.
  repeat: libc::c_long,
  /// Update.
  idle: Option<()>,
  /// Signal.
  sig: Option<libc::c_int>,
  /// The pressed character.
  in_down: Option<Control>,
  /// The released character.
  in_up: Option<Control>,
  /// The output of last text printed.
  out_last: Option<(Out, libc::size_t)>,
  /// The output of matrix Screen interface.
  out_screen: Display,
}

impl ShellState {

    /// The constructor method `new` returns a empty ShellState.
    pub fn new(repeat: Option<libc::c_long>, fd: libc::c_int) -> Self {
        ShellState {
            repeat: repeat.unwrap_or(DEFAULT_KEY_REPEAT),
            idle: None,
            sig: None,
            in_down: None,
            in_up: None,
            out_last: None,
            out_screen: Display::new(fd).unwrap(),
        }
    }

    /// The mutator method `set_repeat` change the interval.
    pub fn set_repeat(&mut self, repeat: libc::c_long) {
        self.repeat = repeat;
    }

    /// The mutator method `set_idle` update the idle event status.
    pub fn set_idle(&mut self, entry: Option<()>) {
        self.idle = entry;
    }

    /// The mutator method `set_signal` update the signal
    /// and can resize the Display interface.
    pub fn set_signal(&mut self, signal: Option<libc::c_int>) {
        self.sig = signal;
        if let Some(()) = self.is_signal_resized() {
            self.out_screen.resize().unwrap();
        }
    }

    /// The mutator method `set_input` update the `in_text`
    /// and save the old `in_text` to `in_text_past`.
    pub fn set_input(&mut self, down: Option<Control>) {
        self.in_up = self.in_down;
        self.in_down = down;
    }

    /// The mutator method `set_output` update the both `out_text`
    /// and `out_screen` variable.
    pub fn set_output(&mut self, entry: Option<(Out, libc::size_t)>) {
        if let Some((buf, len)) = entry {
            self.out_last = Some((buf, len));
            self.out_screen.write(&buf[..len]);
        } else {
            self.out_last = None;
        }
    }

  /// The accessor method `is_idle` returns the Idle event.
  pub fn is_idle(&self) -> Option<()> {
    self.idle
  }

  /// The accessor method `is_signal` returns the Signal event.
  pub fn is_signal(&self) -> Option<libc::c_int> {
    self.sig
  }

  /// The method `is_signal_resized` returns the Option for the WINCH Signal event.
  pub fn is_signal_resized(&self) -> Option<()> {
    if let Some(libc::SIGWINCH) = self.sig {
      Some(())
    } else {
      None
    }
  }

  /// The accessor method `is_input_keydown` returns the pressed Key event.
  pub fn is_input_keydown(&self) -> Option<Key> {
    if let Some(ref event) = self.in_down {
      event.is_key()
    } else {
      None
    }
  }

    /// The accessor method `is_input_keyup` returns the released Key event.
  pub fn is_input_keyup(&self) -> Option<Key> {
    if let Some(ref event) = self.in_up {
        event.is_key()
    } else {
        None
    }
  }

    /// The accessor method `is_input_keydown` returns the pressed Mouse event.
    pub fn is_input_mousedown(&self) -> Option<Mouse> {
        if let Some(ref event) = self.in_down {
            event.is_mouse()
        } else {
            None
        }
    }

    /// The accessor method `is_input_keyup` returns the released Mouse event.
    pub fn is_input_mouseup(&self) -> Option<Mouse> {
        if let Some(ref event) = self.in_up {
            event.is_mouse()
        } else {
            None
        }
    }

  /// The accessor method `is_in_slice` returns the bytes for a Input event.
  pub fn is_input_slice(&self) -> Option<&[libc::c_uchar]> {
    if let Some(ref control) = self.in_down {
        Some(control.as_slice())
    } else {
        None
    }
  }

  /// The accessor method `is_output_last` returns the Output text event.
  pub fn is_output_last(&self) -> Option<&[libc::c_uchar]> {
    if let Some((ref out, len)) = self.out_last {
      Some(&out[..len])
    } else {
      None
    }
  }

  /// The accessor method `is_output_screen` returns the Output screen event.
  pub fn is_output_screen(&self) -> Option<&Display> {
    if self.is_output_last().is_some().bitor(
        self.is_signal_resized().is_some()
    ) {
      Some(&self.out_screen)
    } else {
      None
    }
  }
}

impl Clone for ShellState {
    /// The method `clone` return a copy of the ShellState.
    fn clone(&self) -> Self {
        ShellState {
            repeat: self.repeat,
            idle: self.idle,
            sig: self.sig,
            in_down: self.in_down,
            in_up: self.in_up,
            out_last: self.out_last,
            out_screen: self.out_screen.clone(),
        }
    }

    /// The method `with_device` updates the state from
    /// the event DeviceState interface.
    fn clone_from(&mut self, event: DeviceState) {
        self.set_idle(event.is_idle());
        self.set_signal(event.is_signal());
        self.set_input(event.is_input());
        self.set_output(event.is_out_text());
    }
}
