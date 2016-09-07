use super::Display;

pub struct ShellState {
  /// Update
  idle: Option<()>,
  /// The output of screen
  out_screen: Option<Display>,
  /// The output of new lines
  out_text: Option<Vec<u8>>,
  /// The current character
  in_character: Option<u8>,
  /// The last line
  in_line: Option<Vec<u8>>,
}
