
pub mod color;

use std::ops::Not;

use self::color::{Color, DEFAULT_FOREGROUND, DEFAULT_BACKGROUND};

const IS_BOLD: u8 = 0x01;
const IS_UNDERLINE: u8 = 0x02;
const IS_BLINK: u8 = 0x04;
const IS_REVERSE: u8 = 0x08;
const IS_HIDDEN: u8 = 0x10;

#[derive(Clone, Copy, Debug)]
pub struct Operate
{ attr: u8,
  foreground: Color,
  background: Color, }

impl Operate
{ pub fn new(attr: u8, fore: Color, back: Color) -> Self
  { Operate
    { attr: attr,
      foreground: fore,
      background: back, }}

  fn has_attributes(&self) -> bool
  { self.attr.eq(&0).not() }

  fn is_bold(&self) -> bool
  { (self.attr & IS_BOLD).eq(&0).not() }

  fn is_underline(&self) -> bool
  { (self.attr & IS_UNDERLINE).eq(&0).not() }

  fn is_blink(&self) -> bool
  { (self.attr & IS_BLINK).eq(&0).not() }

  fn is_reverse(&self) -> bool
  { (self.attr & IS_REVERSE).eq(&0).not() }

  fn is_hidden(&self) -> bool
  { (self.attr & IS_HIDDEN).eq(&0).not() }

  fn get_foreground(&self) -> &Color
  { &self.foreground }

  fn get_background(&self) -> &Color
  { &self.background }

  fn set_foreground(&mut self, fore: Color)
  { self.foreground = fore; }

  fn set_background(&mut self, back: Color)
  { self.background = back; }

  fn set_attributes(&mut self, attr: u8)
  { self.attr = attr; }
}

impl Default for Operate
{ fn default() -> Self
  { Operate
    { attr: 0,
      foreground: DEFAULT_FOREGROUND,
      background: DEFAULT_BACKGROUND, }}}
