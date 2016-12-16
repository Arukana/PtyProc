use ::std;
use ::libc;

use std::io::{self, Write};

use super::character::Character;
use super::character::operate::Operate;

#[derive(Debug, Clone)]
pub struct Cursor<T> {
    pos: usize,
    inner: T,
}

impl <T> Cursor <T> {
    /// Create a new cursor wrapping the provided underlying I/O object.
    pub fn new(inner: T) -> Cursor<T> {
        Cursor {
            pos: 0,
            inner: inner,
        }
    }

    /// Consume this cursor, returning the underlying value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Get a reference to the underlying value in this cursor.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the underlying value in this cursor.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying value as it may corrupt this cursor's position.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the current value of this cursor
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Sets the value of this cursor
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }
}

impl Cursor<Vec<Character>>
{ pub fn write_with_color(&mut self, glyph: char, ope: Operate) -> io::Result<usize>
  { unsafe
    { *self.inner.get_unchecked_mut(self.pos) = Character::new(glyph, ope);
      self.pos += 1;
      Ok(1) }}
}
