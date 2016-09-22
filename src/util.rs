use std::fmt;

pub enum Mouse {
  Press,
  Release,
}

impl Mouse {
    pub fn new(s: i32) -> Self {
        match s {
            1 => Mouse::Press,
            _ => Mouse::Release,
        }
    }
}

impl fmt::Debug for Mouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mouse::Press => write!(f, "Pressed"),
            Mouse::Release => write!(f, "Released"),
        }
    }
}
