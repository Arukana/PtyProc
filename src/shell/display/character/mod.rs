pub mod operate;

use std::io::{self, Write};
use std::fmt;
use std::mem;

use ::libc;

use self::operate::Operate;

#[derive(Clone, Copy, Debug, Default)]
pub struct Character {
   /// Term's buffer.
   buf: [libc::c_uchar; 4],
   /// Term's Length.
   len: libc::size_t,
   /// Operation.
   operate: Operate,
}

impl Character {
    pub fn new(c: char) -> Self {
        unsafe {
            Character::from_slice (
                &mem::transmute::<char, [libc::c_uchar; 4]>(c)
            )
        }
    }

    pub fn space() -> Self {
        Character::new(' ')
    }

    /// The constructor method `new` returns a term character.
    pub fn from_slice(buf: &[libc::c_uchar]) -> Self {
        let mut control: Character = Character::default();

        control.write(buf);
        control
    }

    pub fn is_enter(&self) -> Option<()> {
        if self.buf.first().eq(&Some(&b'\n')) {
            Some(())
        } else {
            None
        }
    }

    pub fn is_space(&self) -> Option<()> {
        if self.buf.first().eq(&Some(&b' ')) {
            Some(())
        } else {
            None
        }
    }

    /// The accessor method `get_ref` returns a reference on term character buffer.
    pub fn get_unicode(&self) -> char {
        unsafe {
            mem::transmute::<[libc::c_uchar; 4], char>(self.buf)
        }
    }

    /// The accessor method `get_ref` returns a reference on term character buffer.
    pub fn get_ref(&self) -> &[libc::c_uchar] {
        &self.buf[..self.len]
    }

    /// The method `clear` resets the term character.
    pub fn clear(&mut self) -> io::Result<usize> {
        *self = Default::default();
        self.write(&[b' '][..])
    }
}

impl From<char> for Character {

    fn from(glyph: char) -> Character {
        unsafe {
            Character {
                buf: mem::transmute::<char, [libc::c_uchar; 4]>(glyph),
                len: glyph.len_utf8(),
                operate: Operate::default(),
            }
        }
    }
}

impl Write for Character {

    /// The method `write` from trait `io::Write` inserts a new list of terms
    /// from output.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
      println!("BUF::{:?}", buf);
        self.len = buf.len();
        (&mut self.buf[..]).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.buf[0] as char)
    }
}
