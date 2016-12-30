use std::ops::Index;
use std::ops::{Deref, DerefMut};
use std::ops::RangeTo;

use ::libc;

pub struct Buf(pub [libc::c_uchar; 100], pub usize);

impl Default for Buf {
    fn default() -> Buf {
        Buf([0u8; 100], 0)
    }
}

impl Deref for Buf {
    type Target = [libc::c_uchar];
    fn deref<'a>(&'a self) -> &[libc::c_uchar] {
        &self.0
    }
}

impl DerefMut for Buf {
    fn deref_mut(&mut self) -> &mut [libc::c_uchar] {
        &mut self.0
    }
}

impl Index<libc::size_t> for Buf {
    type Output = libc::c_uchar;
    fn index(&self, count: libc::size_t) -> &libc::c_uchar {
        &self.0[count]
    }
}

impl Index<RangeTo<libc::size_t>> for Buf {
    type Output = [libc::c_uchar];
    fn index(&self, range: RangeTo<libc::size_t>) -> &[libc::c_uchar] {
        &self.0[range]
    }
}

impl Clone for Buf {
    fn clone(&self) -> Self {
        Buf(self.0, self.1)
    }
}

impl Copy for Buf {
}
