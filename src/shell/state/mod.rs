pub mod clone;

use std::io::Write;
use std::ops::BitOr;

use ::libc;

use super::Display;
use super::device::control::Control;

use self::clone::Clone;
pub use super::device::{Out, DeviceState};

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
  out_text: Option<(Out, libc::size_t)>,
  /// The output of screen.
  out_screen: Display,
  /// The last line.
  in_line: Vec<libc::c_uchar>,
  in_line_ready: bool,
}

impl ShellState {

  /// The constructor method `new` returns a empty ShellState.
  pub fn new(fd: libc::c_int) -> Self {
      ShellState {
          idle: None,
          sig: None,
          in_text: None,
          in_text_past: None,
          out_text: None,
          out_screen: Display::new(fd).unwrap(),
          in_line: Vec::new(),
          in_line_ready: false,
      }
  }

    /// The murator method `set_signal` update the signal and can resize the Display interface.
    pub fn set_signal(&mut self, entry: Option<libc::c_int>) {
        self.sig = entry;
        if let Some(()) = self.is_resized() {
            self.out_screen.resize().unwrap();
        }
    }

    /// The mutator method `set_output` update the both `out_text` and `out_screen` variable.
    pub fn set_output(&mut self, entry: Option<(Out, libc::size_t)>) {
        if let Some((buf, len)) = entry {
            self.out_text = Some((buf, len));
            let buf: &[libc::c_uchar] = &buf[..len];

            self.out_screen.write(buf);
        } else {
            self.out_text = None;
        }
    }

  /// The method `is_screen` returns a screen interface.
  pub fn is_screen(&self) -> Option<&Display> {
      if self.out_text.is_some() {
          Some(&self.out_screen)
      } else {
          None
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

  /// The method `is_resized` returns the Option for the WINCH Signal event.
  pub fn is_resized(&self) -> Option<()> {
    if let Some(libc::SIGWINCH) = self.sig {
      Some(())
    } else {
      None
    }
  }

  /// The accessor method `is_unicode` returns the KeyDown event.
  pub fn is_unicode(&self) -> Option<&[libc::c_uchar]> {
    if let Some(ref event) = self.in_text {
      event.is_unicode()
    } else {
      None
    }
  }

  /// The accessor method `is_out_text` returns the Output text event.
  pub fn is_out_text(&self) -> Option<&[libc::c_uchar]> {
    if let Some((ref out, len)) = self.out_text {
      Some(&out[..len])
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
}

impl Clone for ShellState {
    fn clone(&self) -> Self {
        ShellState {
            idle: self.idle,
            sig: self.sig,
            in_text: self.in_text.clone(),
            in_text_past: self.in_text_past,
            out_text: self.out_text,
            out_screen: self.out_screen.clone(),
            in_line: self.in_line.clone(),
            in_line_ready: self.in_line_ready,
        }
    }

    /// The method `with_device` updates the state from
    /// the event DeviceState interface.
    fn clone_from(&mut self, event: DeviceState) {
        self.idle = event.is_idle();
        self.in_text_past = self.in_text;
        self.in_text = event.is_input();
        self.set_signal(event.is_signal());
        self.set_output(event.is_out_text());
        if self.in_line_ready {
            self.in_line.clear();
            self.in_line_ready = false;
        }
        if let Some(ref event) = self.in_text {
            self.in_line.extend_from_slice(event.as_slice());
            if let Some(last) = self.in_line.last() {
                if last.eq(&10).bitor(last.eq(&13)) {
                    self.in_line_ready = true;
                }
            }
        }
    }
}
