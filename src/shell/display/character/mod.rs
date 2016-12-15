pub mod operate;

use std::io::{self, Write};
use std::fmt;
use std::mem;

use ::libc;

use self::operate::Operate;
use self::operate::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Character {
    /// Glyph.
    glyph: char,
    /// Operation.
    operate: Operate,
}

impl Character {
    pub fn get_attributes(&self) -> &Operate { &self.operate }

    pub fn is_enter(&self) -> bool { self.glyph.eq(&'\n') }

    pub fn is_space(&self) -> bool { self.glyph.eq(&' ') }

    /// The accessor method `get_ref` returns a reference on term character buffer.
    pub fn get_glyph(&self) -> char { self.glyph }

    /// The method `clear` resets the term character.
    pub fn clear(&mut self) { *self = Self::default(); }
}

impl From<(char, Operate)> for Character {
    fn from((glyph, operate): (char, Operate)) -> Character {
        unsafe {
            Character {
                glyph: glyph,
                operate: operate,
            }
        }
    }
}

impl From<char> for Character {
    fn from(glyph: char) -> Character {
        unsafe {
            Character {
                glyph: glyph,
                operate: Operate::default(),
            }
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character::from(' ')
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.operate, self.glyph)
    }
}
