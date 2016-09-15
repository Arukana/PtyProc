use std::io::{self, Write};
use std::ops::BitOr;

use ::libc;

use super::Display;
use super::device::{DeviceState, Control};

#[derive(Default, Clone)]
pub struct ShellState {
  /// Update.
  idle: Option<()>,
  /// Signal.
  sig: Option<libc::c_int>,
  /// The current character.
  in_text: Option<Control>,
  /// The past character.
  in_text_past: Option<Control>,
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

  /// The accessor method `is_signal` returns the Signal event.
  pub fn is_signal(&self) -> Option<libc::c_int> {
    self.sig
  }

  /// The method `is_resized` returns the Option for the WINCH Signal event.
  pub fn is_resized(&self) -> Option<()> {
    if let Some(libc::SIGWINCH) = self.sig {
      Some(())
    } else {
      None
    }
  }

  /// The accessor method `is_keydown` returns the KeyDown event.
  pub fn is_keydown(&self) -> Option<libc::c_uchar> {
    if let Some(ref event) = self.in_text {
      event.is_char()
    } else {
      None
    }
  }

  /// The accessor method `is_out_text` returns the Output text event.
  pub fn is_out_text(&self) -> Option<&Vec<libc::c_uchar>> {
    if let Some(ref out) = self.out_text {
      Some(out)
    } else {
      None
    }
  }

  /// The accessor method `is_in_text` returns the Input text event.
  pub fn is_in_text(&self) -> Option<&[libc::c_uchar]> {
    if let Some(ref int) = self.in_text {
      Some(int.as_slice())
    } else {
      None
    }
  }

  /// The accessor method `is_out_screen` returns the Output screen event.
  pub fn is_out_screen(&self) -> Option<&Display> {
    if self.idle.is_none().bitor(
      self.sig.eq(&Some(libc::SIGWINCH))
    ) {
      Some(&self.out_screen)
    } else {
      None
    }
  }

  /// The accessor method `is_line` returns the Output line event.
  pub fn is_line(&self) -> Option<&Vec<libc::c_uchar>> {
    if self.in_line_ready {
      Some(&self.in_line)
    } else {
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
    self.in_text_past = self.in_text;
    self.in_text = event.is_input();
    self.out_text = event.is_out_text();
    if self.in_line_ready {
      self.in_line.clear();
      self.in_line_ready = false;
    }
    if let Some(ref event) = self.in_text {
      self.in_line.extend_from_slice(event.as_slice());
      if event.is_enter().is_some() {
        self.in_line_ready = true;
      }
    }
    if let Some(ref buf) = self.out_text {
      try!(self.out_screen.write(buf.as_slice()));
    }
    Ok(())
  }
}
