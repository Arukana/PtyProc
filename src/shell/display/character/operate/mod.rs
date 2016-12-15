
pub mod color;

use std::ops::{BitAnd, Not};

pub use self::color::{Color, DEFAULT_FOREGROUND, DEFAULT_BACKGROUND};

const IS_BOLD: u8 = 0x01;
const IS_UNDERLINE: u8 = 0x02;
const IS_BLINK: u8 = 0x04;
const IS_REVERSE: u8 = 0x08;
const IS_HIDDEN: u8 = 0x10;
const IS_ITALIC: u8 = 0x20;

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

  pub fn has_attributes(&self) -> bool
  { (self.attr.eq(&0).bitand(self.foreground.eq(&DEFAULT_FOREGROUND)).bitand(self.background.eq(&DEFAULT_BACKGROUND))).not() }

  pub fn set_bold(&mut self)
  { self.attr |= IS_BOLD; }

  pub fn set_underline(&mut self)
  { self.attr |= IS_UNDERLINE; }

  pub fn set_blink(&mut self)
  { self.attr |= IS_BLINK; }

  pub fn set_reverse(&mut self)
  { self.attr |= IS_REVERSE; }

  pub fn set_hidden(&mut self)
  { self.attr |= IS_HIDDEN; }

  pub fn set_italic(&mut self)
  { self.attr |= IS_ITALIC; }

  pub fn unset_bold(&mut self)
  { self.attr &= !IS_BOLD; }

  pub fn unset_underline(&mut self)
  { self.attr &= !IS_UNDERLINE; }

  pub fn unset_blink(&mut self)
  { self.attr &= !IS_BLINK; }

  pub fn unset_reverse(&mut self)
  { self.attr &= !IS_REVERSE; }

  pub fn unset_hidden(&mut self)
  { self.attr &= !IS_HIDDEN; }

  pub fn unset_italic(&mut self)
  { self.attr &= !IS_ITALIC; }

  pub fn is_bold(&self) -> bool
  { (self.attr & IS_BOLD).eq(&0).not() }

  pub fn is_underline(&self) -> bool
  { (self.attr & IS_UNDERLINE).eq(&0).not() }

  pub fn is_blink(&self) -> bool
  { (self.attr & IS_BLINK).eq(&0).not() }

  pub fn is_reverse(&self) -> bool
  { (self.attr & IS_REVERSE).eq(&0).not() }

  pub fn is_hidden(&self) -> bool
  { (self.attr & IS_HIDDEN).eq(&0).not() }

  pub fn is_italic(&self) -> bool
  { (self.attr & IS_ITALIC).eq(&0).not() }

  pub fn get_foreground(&self) -> &Color
  { &self.foreground }

  pub fn get_background(&self) -> &Color
  { &self.background }

  pub fn set_foreground(&mut self, fore: Color)
  { self.foreground = fore; }

  pub fn set_background(&mut self, back: Color)
  { self.background = back; }

  pub fn clear(&mut self)
  { self.attr = 0;
    self.foreground = DEFAULT_FOREGROUND;
    self.background = DEFAULT_BACKGROUND; }
}

impl Default for Operate
{ fn default() -> Self
  { Operate
    { attr: 0,
      foreground: DEFAULT_FOREGROUND,
      background: DEFAULT_BACKGROUND, }}}
