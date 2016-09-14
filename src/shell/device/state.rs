use ::libc;

use super::In;
use super::Out;
use super::Sig;

pub enum DeviceState {
  /// Update.
  Idle,
  /// As catched a signal.
  Sig(Sig),
  /// The output of new lines.
  OutText(Out, libc::size_t),
  /// The current character.
  InCharacter(In),
}

impl DeviceState {

  /// The constructor method `from_idle` returns a Update's event.
  pub fn from_idle() -> Self {
    DeviceState::Idle
  }

  /// The constructor method `from_out` returns a text Output's event.
  pub fn from_out(buf: Out, len: libc::size_t) -> Self {
    DeviceState::OutText(buf, len)
  }

  /// The constructor method `from_out` returns a key Input's event.
  pub fn from_in(key: libc::c_uchar) -> Self {
    DeviceState::InCharacter(key)
  }

  /// The constructor method `from_ig` returns a Signal's event.
  pub fn from_sig(sig: libc::c_int) -> Self {
    DeviceState::Sig(sig)
  }

  /// The accessor method `is_idle` returns a Option for Update's event.
  pub fn is_idle(&self) -> Option<()> {
    match *self {
      DeviceState::Idle => Some(()),
      _ => None,
    }
  }

  /// The accessor method `is_out_text` returns a Option for Ouput's event.
  pub fn is_out_text(self) -> Option<Vec<libc::c_uchar>> {
    match self {
      DeviceState::OutText(buf, len) => Some(Vec::from(&buf[..len])),
      _ => None,
    }
  }

  /// The accessor method `is_character` returns a Option for key Input's event.
  pub fn is_character(&self) -> Option<libc::c_uchar> {
    match *self {
      DeviceState::InCharacter(key) => Some(key),
      _ => None,
    }
  }

  /// The accessor method `is_signal` returns a Option for Signal's event.
  pub fn is_signal(&self) -> Option<libc::c_int> {
    match *self {
      DeviceState::Sig(sig) => Some(sig),
      _ => None,
    }
  }
}
