mod err;
pub mod code;

use std::str;
use std::io::Write;

use ::libc;

use super::In;
pub use self::err::{MouseError, Result};
use self::code::Code;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Mouse {
    code: Code,
    pressed: bool,
    x: libc::c_ushort,
    y: libc::c_ushort,
}

impl Mouse {
    fn with_code(action: u8, next: &[u8]) -> Result<Self> {
        match next {
            &[ref coordinate.., m @ b'M'...b'm'] => {
                match Code::new(action) {
                    Ok(code) => {
                        coordinate.iter().position(|&sep| sep.eq(&b';')).map(|index: usize| unsafe {
                            let (term_x, term_y): (&[u8], &[u8]) = coordinate.split_at(index);
                            let term_y: &[u8] = &term_y[1..term_y.len()];
                            if let (Ok(x), Ok(y)) = (
                                u16::from_str_radix(str::from_utf8_unchecked(term_x), 10),
                                u16::from_str_radix(str::from_utf8_unchecked(term_y), 10)
                            ) {
                                Ok(Mouse::from((code, m.eq(&b'M'), [x, y]))) 
                            } else {
                                Err(MouseError::FromStrFail)
                            }
                        }).unwrap_or(Err(MouseError::PositionNotFound))
                    },
                    Err(why) => Err(MouseError::Code(why)),
                }
            },
            _ => Err(MouseError::Other),
        }
    }

    pub fn new(buf: &[u8]) -> Result<Self> {
        match buf {
            &[b'\x1B', b'[', b'<', action1 @ b'0'...b'9', action2 @ b'0'...b'9', b';', ref next..] => {
                Mouse::with_code((action1 - 48) * 10 + action2 - 48, next)
            },
            &[b'\x1B', b'[', b'<', action @ b'0'...b'9', b';', ref next..] => {
                Mouse::with_code(action - 48, next)
            },
            _ => Err(MouseError::Other),
        }
    }

    pub fn as_input(&self) -> (In, libc::size_t) {
        let mut input: In = In::default();
        let mut length: libc::size_t = 3;
        let xx: String = self.x.to_string();
        let x: &[u8] = xx.as_bytes();
        let yy: String = self.y.to_string();
        let y: &[u8] = yy.as_bytes();

        let _ = input.as_mut()[..].as_mut().write(&[b'\x1B', b'[', b'<']);
        let _ = input.as_mut()[length..].as_mut().write(x);
        length += x.len();
        if self.pressed { 
            let _ = input.as_mut()[length..].as_mut().write(&[b'M']);
        } else {
            let _ = input.as_mut()[length..].as_mut().write(&[b'm']);
        }
        length += 1;
        let _ = input.as_mut()[length..].as_mut().write(y);
        length += y.len();
        (input, length)
    }
}

impl From<(Code, bool, [libc::c_ushort; 2])> for Mouse {
    fn from((code, pressed, [x, y]): (Code, bool, [libc::c_ushort; 2])) -> Mouse {
        Mouse {
            code: code,
            pressed: pressed,
            x: x,
            y: y,
        }
    }
}

impl From<(u8, bool, [libc::c_ushort; 2])> for Mouse {
    fn from((code, pressed, [x, y]): (u8, bool, [libc::c_ushort; 2])) -> Mouse {
        Mouse {
            code: Code::from(code),
            pressed: pressed,
            x: x,
            y: y,
        }
    }
}
