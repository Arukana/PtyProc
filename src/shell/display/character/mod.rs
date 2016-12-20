pub mod attribute;
pub mod color;

use std::io::{self, Write};
use std::ops::BitAnd;
use std::ops::Not;
use std::fmt;
use std::mem;
use std::char;

use ::libc;

use self::attribute::Attribute;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Character {
    /// Attribute.
    attribute: libc::c_uchar,
    /// Text color.
    foreground: [libc::c_uchar; 3],
    /// Background color.
    background: [libc::c_uchar; 3],
    /// Glyph.
    glyph: libc::c_uint,
}

impl Character {
    pub fn is_enter(&self) -> bool { self.glyph.eq(&10) }

    pub fn is_space(&self) -> bool { self.glyph.eq(&32) }

    pub fn is_null(&self) -> bool {
        self.glyph.eq(&0)
    }

    pub fn is_bold(&self) -> bool {
        (self.attribute & Attribute::Bold as u8).eq(&0).not()
    }

    pub fn is_dim(&self) -> bool {
        (self.attribute & Attribute::Dim as u8).eq(&0).not()
    }

    pub fn is_italic(&self) -> bool {
        (self.attribute & Attribute::Italic as u8).eq(&0).not()
    }

    pub fn is_underline(&self) -> bool {
        (self.attribute & Attribute::Underline as u8).eq(&0).not()
    }

    pub fn is_blink(&self) -> bool {
        (self.attribute & Attribute::Blink as u8).eq(&0).not()
    }
 
    pub fn is_reverse(&self) -> bool {
        (self.attribute & Attribute::Reverse as u8).eq(&0).not()
    }

    pub fn is_hidden(&self) -> bool {
        (self.attribute & Attribute::Hidden as u8).eq(&0).not()
    }

    pub fn get_foreground(&self) -> [libc::c_uchar; 3] {
        self.foreground
    }

    pub fn get_background(&self) -> [libc::c_uchar; 3] {
        self.background
    }

    pub fn get_glyph(&self) -> char {
        unsafe {
            mem::transmute::<libc::c_uint, char>(self.glyph)
        }
    }

    pub fn set_foreground(&mut self, fore: [libc::c_uchar; 3]) {
        self.foreground = fore;
    }

    pub fn set_background(&mut self, back: [libc::c_uchar; 3]) {
        self.background = back;
    }

    pub fn add_attribute(&mut self, attr: Attribute) {
        self.attribute |= attr as u8;
    }

    pub fn sub_attribute(&mut self, attr: Attribute) {
        self.attribute &= (attr as u8).not();
    }

    pub fn set_attribute(&mut self, attr: Attribute) {
        self.attribute = attr as u8;
    }

    pub fn set_glyph(&mut self, glyph: char) {
        self.glyph = glyph as libc::c_uint;
    }

    /// The method `clear` resets the term character.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.attribute.eq(&0)
        .bitand(self.foreground.eq(&color::DEFAULT_FOREGROUND)
        .bitand(self.background.eq(&color::DEFAULT_BACKGROUND)))
        .not() {
            if self.attribute.gt(&0)
            { try!("\x1B[".fmt(f));
              if self.is_bold() {
                try!(Attribute::Bold.fmt(f))
              }
              if self.is_dim() {
                try!(Attribute::Dim.fmt(f))
              }
              if self.is_italic() {
                try!(Attribute::Italic.fmt(f))
              }
              if self.is_underline() {
                try!(Attribute::Underline.fmt(f))
              }
              if self.is_blink() {
                try!(Attribute::Blink.fmt(f))
              }
              if self.is_reverse() {
                try!(Attribute::Reverse.fmt(f))
              }
              if self.is_hidden() {
                try!(Attribute::Hidden.fmt(f))
              }
              try!(format!("m").fmt(f)) }
            if self.foreground.eq(&[0, 0, 0]).not()
            { try!(format!("\x1B[38;2;{};{};{};2m",
                         self.foreground[0],
                         self.foreground[1],
                         self.foreground[2]).fmt(f)) }
            if self.background.eq(&[255, 255, 255]).not()
            { try!(format!("\x1B[48;2;{};{};{};2m",
                         self.background[0],
                         self.background[1],
                         self.background[2]).fmt(f)) }
        } else {
            try!(format!("\x1B[m").fmt(f))
        }
        unsafe {
            try!(format!("{}", char::from_u32_unchecked(self.glyph)).fmt(f))
        }
        Ok(())
    }
}

impl From<char> for Character {
    fn from(glyph: char) -> Character {
        Character {
           attribute: 0,
           foreground: color::Black,
           background: color::White,
           glyph: glyph as libc::c_uint,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character::from(' ')
    }
}
