use std::ops::{Index, IndexMut, RangeTo, Deref, DerefMut};
use std::{fmt, mem};
use std::io;

use ::libc;

pub struct In(pub [libc::c_uchar; 496]);

impl In {
    pub fn as_slice(&self) -> &[libc::c_uchar; 496] {
        &self.0
    }
}

impl Default for In {
    fn default() -> In {
        unsafe {
            In(mem::zeroed())
        }
    }
}

impl Eq for In {}

impl PartialEq for In {
    fn eq(&self, other: &In) -> bool {
        self.0.eq(&other.0[..])
    }
}

impl Deref for In {
   type Target = [libc::c_uchar];

   fn deref<'a>(&'a self) -> &[libc::c_uchar] {
       &self.0
   }
}

impl DerefMut for In {
   fn deref_mut(&mut self) -> &mut [libc::c_uchar] {
       &mut self.0
   }
}

impl Index<libc::size_t> for In {
    type Output = libc::c_uchar;

    fn index(&self, count: libc::size_t) -> &libc::c_uchar {
        &self.0[count]
    }
}

impl IndexMut<libc::size_t> for In {
    fn index_mut(&mut self, count: libc::size_t) -> &mut libc::c_uchar {
        &mut self.0[count]
    }
}

impl Index<RangeTo<libc::size_t>> for In {
    type Output = [libc::c_uchar];

    fn index(&self, range: RangeTo<libc::size_t>) -> &[libc::c_uchar] {
        &self.0[range]
    }
}

impl Copy for In {}

impl Clone for In {
    fn clone(&self) -> Self {
        In(self.0)
    }
}

impl fmt::Debug for In {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0) )
    }
}

impl io::Write for In {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.deref_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.deref_mut().flush()
    }
}
