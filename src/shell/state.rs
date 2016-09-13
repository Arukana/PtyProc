use std::io::{self, Write};
use std::ops::{BitOr, BitAnd, Not};

use ::libc;

use super::Display;
use super::DeviceState;

#[derive(Clone, Default)]
pub struct ShellState {
  /// Update.
  idle: Option<()>,
  /// Signal.
  sig: Option<libc::c_int>,
  /// The current character.
  in_character: Option<libc::c_uchar>,
  /// The output of new lines.
  out_text: Option<Vec<libc::c_uchar>>,
  /// The output of screen.
  out_screen: Display,
  /// The last line.
  in_line: Vec<libc::c_uchar>,
  in_line_ready: bool,
}

impl ShellState {

  /// The accessor method `is_idle` returns the Idle event.
  pub fn is_idle(&self) -> Option<()> {
    self.idle
  }

  /// The accessor method `is_keydown` returns the KeyDown event.
  pub fn is_keydown(&self) -> Option<u8> {
    self.in_character
  }

  /// The accessor method `is_out_text` returns the Output text event.
  pub fn is_out_text(&self) -> Option<&Vec<u8>> {
    if let Some(ref out) = self.out_text {
      Some(&out)
    }
    else {
      None
    }
  }

  /// The accessor method `is_out_screen` returns the Output screen event.
  pub fn is_out_screen(&self) -> Option<&Display> {
    if self.idle.is_none().bitand(
      self.sig.eq(&Some(libc::SIGWINCH)).not()
    ) {
      Some(&self.out_screen)
    }
    else {
      None
    }
  }

  /// The accessor method `is_line` returns the Output line event.
  pub fn is_line(&self) -> Option<&Vec<u8>> {
    if self.in_line_ready {
      Some(&self.in_line)
    }
    else {
      None
    }
  }

  /// The modifier method `with_device` updates the state according to
  /// the event's option.
  pub fn with_device (
    &mut self,
    event: DeviceState,
  ) -> io::Result<()> {
    self.idle = event.is_idle();
    self.sig = event.is_signal();
    self.in_character = event.is_character();
    self.out_text = event.to_owned().is_out_text();
    if self.in_line_ready {
      self.in_line.clear();
      self.in_line_ready = false;
    }
    if let Some(key) = self.in_character {
      self.in_line.push(key);
      if key.eq(&10).bitor(key.eq(&13)) {
        self.in_line_ready = true;
      }
    }
    if let Some(ref buf) = self.out_text {
      try!(self.out_screen.write(buf.as_slice()));
    }
    Ok(())
  }
}
