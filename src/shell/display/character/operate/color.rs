
pub const DEFAULT_FOREGROUND: Color = Color::Black;
pub const DEFAULT_BACKGROUND: Color = Color::White;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color
{ Black,
  White,
  Red,
  Green,
  Blue,
  Yellow,
  Cyan,
  Magenta,
  Custom(u8, u8, u8), }
