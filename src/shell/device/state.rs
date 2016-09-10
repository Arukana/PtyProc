#[derive(Clone)]
pub enum DeviceState {
  /// Update
  Idle,
  /// The output of new lines
  OutText(Vec<u8>),
  /// The current character
  InCharacter(u8),
}

impl DeviceState {

  /// The constructor method `from_idle` returns a Update's event.
  pub fn from_idle() -> Self {
    DeviceState::Idle
  }

  /// The constructor method `from_out` returns a text Output's event.
  pub fn from_out(buf: &[u8]) -> Self {
    DeviceState::OutText(Vec::from(buf))
  }

  /// The constructor method `from_out` returns a key Input's event.
  pub fn from_in(key: u8) -> Self {
    DeviceState::InCharacter(key)
  }

  /// The accessor method `is_idle` returns a Option for Update's event.
  pub fn is_idle(&self) -> Option<()> {
    match *self {
      DeviceState::Idle => Some(()),
      _ => None,
    }
  }

  /// The accessor method `is_out_text` returns a Option for Ouput's event.
  pub fn is_out_text(self) -> Option<Vec<u8>> {
    match self {
      DeviceState::OutText(buf) => Some(buf),
      _ => None,
    }
  }

  /// The accessor method `is_character` returns a Option for key Input's event.
  pub fn is_character(&self) -> Option<u8> {
    match *self {
      DeviceState::InCharacter(key) => Some(key),
      _ => None,
    }
  }
}
