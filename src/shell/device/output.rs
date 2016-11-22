use std::ops::{Index, RangeTo, Deref, DerefMut};
use std::fmt;

use ::libc;

pub struct Out([libc::c_uchar; 496]);

impl Default for Out {
    fn default() -> Out {
        Out([0u8; 496])
    }
}

impl Deref for Out {
   type Target = [libc::c_uchar];

   fn deref<'a>(&'a self) -> &[libc::c_uchar] {
       &self.0
   }
}

impl DerefMut for Out {
   fn deref_mut(&mut self) -> &mut [libc::c_uchar] {
       &mut self.0
   }
}

impl Index<libc::size_t> for Out {
    type Output = libc::c_uchar;

    fn index(&self, count: libc::size_t) -> &libc::c_uchar {
        &self.0[count]
    }
}

impl Index<RangeTo<libc::size_t>> for Out {
    type Output = [libc::c_uchar];

    fn index(&self, range: RangeTo<libc::size_t>) -> &[libc::c_uchar] {
        &self.0[range]
    }
}

impl Copy for Out {}

impl Clone for Out {
    fn clone(&self) -> Self {
        Out(self.0)
    }
}

impl fmt::Debug for Out {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0) )
    }
}
