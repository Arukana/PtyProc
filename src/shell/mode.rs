/// The enum `Mode` describes how to write the user input.
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
