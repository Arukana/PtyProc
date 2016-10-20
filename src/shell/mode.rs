/// The enum `Mode` describes how to write the user input.
#[derive(Debug, Copy, Clone)]
pub enum Mode {
  /// Character-at-a-time.
  Character,
  /// Line-at-a-time.
  Line,
  /// Screen-at-a-time.
  Block,
  /// Undefined.
  None,
}
