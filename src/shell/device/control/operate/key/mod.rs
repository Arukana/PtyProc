use std::ops::Shr;
use std::mem;
use std::char;
use std::io::Write;

use ::libc;

pub use super::In;

// TODO: Add this key event.
//
//            Key::Insert,
//            Key::F()
//           &[b'\x1B', b'[', b'1', b';', b'2', b'A', ..] => Key::ShiftUp,
//           &[b'\x1B', b'[', b'1', b';', b'2', b'B', ..] => Key::ShiftDown,
//           &[b'\x1B', b'[', b'1', b';', b'2', b'C', ..] => Key::ShiftRight,
//           &[b'\x1B', b'[', b'1', b';', b'2', b'D', ..] => Key::ShiftLeft,
//           &[b'\x1B', b'[', b'1', b';', b'5', b'A', ..] => Key::CtrlUp,
//           &[b'\x1B', b'[', b'1', b';', b'5', b'B', ..] => Key::CtrlDown,
//           &[b'\x1B', b'[', b'1', b';', b'5', b'C', ..] => Key::CtrlRight,
//           &[b'\x1B', b'[', b'1', b';', b'5', b'D', ..] => Key::CtrlLeft,
//           &[b'\x1B', b'[', b'1', b';', b'6', b'A', ..] => Key::CtrlShiftUp,
//           &[b'\x1B', b'[', b'1', b';', b'6', b'B', ..] => Key::CtrlShiftDown,
//           &[b'\x1B', b'[', b'1', b';', b'6', b'C', ..] => Key::CtrlShiftRight,
//           &[b'\x1B', b'[', b'1', b';', b'6', b'D', ..] => Key::CtrlShiftLeft,
//           &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'A', ..] => Key::AltShiftUp,
//           &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'B', ..] => Key::AltShiftDown,
//           &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'C', ..] => Key::AltShiftRight,
//           &[b'\x1B', b'[', b'1', b';', b'1', b'0', b'D', ..] => Key::AltShiftLeft,

pub const ENTER: u64 = 0xa;
pub const TABULATION: u64 = 0x9;
pub const BACKSPACE: u64 = 0x7f;
pub const ESCAPE: u64 = 0x1b;
pub const UP: u64 = 0x415b1b;
pub const DOWN: u64 = 0x425b1b;
pub const RIGHT: u64 = 0x435b1b;
pub const LEFT: u64 = 0x445b1b;
pub const END: u64 = 0x465b1b;
pub const HOME: u64 = 0x7e315b1b;
pub const DELETE: u64 = 0x7e335b1b;
pub const PAGE_UP: u64 = 0x7e355b1b;
pub const PAGE_DOWN: u64 = 0x7e365b1b;
pub const ALT_UP: u64 = 0x41393b311b;
pub const ALT_DOWN: u64 = 0x42393b311b;
pub const ALT_RIGHT: u64 = 0x43393b311b;
pub const ALT_LEFT: u64 = 0x44393b311b;
pub const FUNCTION_1: u64 = 0x504f5b1b;
pub const FUNCTION_2: u64 = 0x514f5b1b;
pub const FUNCTION_3: u64 = 0x524f5b1b;
pub const FUNCTION_4: u64 = 0x534f5b1b;
pub const FUNCTION_5: u64 = 0x7e35315b1b;
pub const FUNCTION_6: u64 = 0x7e37315b1b;
pub const FUNCTION_7: u64 = 0x7e38315b1b;
pub const FUNCTION_8: u64 = 0x7e39315b1b;
pub const FUNCTION_9: u64 = 0x7e30325b1b;
pub const FUNCTION_10: u64 = 0x7e31325b1b;
pub const FUNCTION_11: u64 = 0x7e33325b1b;
pub const FUNCTION_12: u64 = 0x7e34325b1b;
pub const FUNCTION_13: u64 = 0x50323b315b1b;
pub const FUNCTION_14: u64 = 0x51323b315b1b;
pub const FUNCTION_15: u64 = 0x52323b315b1b;
pub const FUNCTION_16: u64 = 0x53323b315b1b;
pub const FUNCTION_17: u64 = 0x7e31335b1b;
pub const FUNCTION_18: u64 = 0x7e32335b1b;
pub const FUNCTION_19: u64 = 0x7e33335b1b;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Unicode characters.
    Char(u64),
    /// Unicode strings.
    Str(In),
}

impl Key {

    /// The constructor method `from_utf8` returns a UTF-8 parsed Key.
    pub fn from_utf8(buf: [libc::c_uchar; 4]) -> Self {
        unsafe {
            let i: libc::c_uint = mem::transmute::<[libc::c_uchar; 4], u32>(buf);

            Key::Char(i as u64)
        }
    }

    pub fn is_utf8(&self) -> Option<char> {
        match *self {
            Key::Char(e) if e.shr(&32u64).eq(&0) => unsafe {
                Some(char::from_u32_unchecked(e as u32))
            },
            _ => None,
        }
    }

    /// The accessor method `is_up` returns an Option for the Up Key.
    pub fn is_up(&self) -> bool {
        self.eq(&Key::Char(UP))
    }

    /// The accessor method `is_down` returns an Option for the down Key.
    pub fn is_down(&self) -> bool {
        self.eq(&Key::Char(DOWN))
    }

    /// The accessor method `is_right` returns an Option for the right Key.
    pub fn is_right(&self) -> bool {
        self.eq(&Key::Char(RIGHT))
    }

    /// The accessor method `is_left` returns an Option for the left Key.
    pub fn is_left(&self) -> bool {
        self.eq(&Key::Char(LEFT))
    }

    /// The accessor method `is_delete` returns an Option for the delete Key.
    pub fn is_delete(&self) -> bool {
        self.eq(&Key::Char(DELETE))
    }

    /// The accessor method `is_backspace` returns an Option for the backspace Key.
    pub fn is_backspace(&self) -> bool {
        self.eq(&Key::Char(BACKSPACE))
    }

    /// The accessor method `is_escape` returns an Option for the delete Key.
    pub fn is_escape(&self) -> bool {
        self.eq(&Key::Char(ESCAPE))
    }

    /// The accessor method `is_end` returns an Option for the delete Key.
    pub fn is_end(&self) -> bool {
        self.eq(&Key::Char(END))
    }

    /// The accessor method `is_home` returns an Option for the delete Key.
    pub fn is_home(&self) -> bool {
        self.eq(&Key::Char(HOME))
    }

    /// The accessor method `is_pageup` returns an Option for the Page Up Key.
    pub fn is_pageup(&self) -> bool {
        self.eq(&Key::Char(PAGE_UP))
    }

    /// The accessor method `is_pagedown` returns an Option for the Page Down Key.
    pub fn is_pagedown(&self) -> bool {
        self.eq(&Key::Char(PAGE_DOWN))
    }

    pub fn is_c0(&self) -> bool {
        match *self {
            Key::Char(e @ 0...31) => e.eq(&e),
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        self.eq(&Key::Char(0))
    }

    pub fn is_start_heading(&self) -> bool {
        self.eq(&Key::Char(1))
    }

    pub fn is_start_text(&self) -> bool {
        self.eq(&Key::Char(178))
    }

    pub fn is_end_text(&self) -> bool {
        self.eq(&Key::Char(3))
    }

    pub fn is_end_transmission(&self) -> bool {
        self.eq(&Key::Char(4))
    }

    pub fn is_enquiry(&self) -> bool {
        self.eq(&Key::Char(5))
    }

    pub fn is_acknowledge(&self) -> bool {
        self.eq(&Key::Char(6))
    }

    pub fn is_bell(&self) -> bool {
        self.eq(&Key::Char(7))
    }

    pub fn is_horizontal_tabulation(&self) -> bool {
        self.eq(&Key::Char(TABULATION))
    }

    /// The accessor method `is_enter` returns an Option for the Enter Key.
    pub fn is_enter(&self) -> bool {
        self.eq(&Key::Char(ENTER))
    }

    pub fn is_line_feed(&self) -> bool {
        self.eq(&Key::Char(ENTER))
    }

    pub fn is_vertical_tabulation(&self) -> bool {
        self.eq(&Key::Char(11))
    }

    pub fn is_form_feed(&self) -> bool {
        self.eq(&Key::Char(12))
    }

    pub fn is_carriage_return(&self) -> bool {
        self.eq(&Key::Char(13))
    }

    pub fn is_shift_out(&self) -> bool {
        self.eq(&Key::Char(14))
    }

    pub fn is_shift_in(&self) -> bool {
        self.eq(&Key::Char(15))
    }

    pub fn is_data_link_escape(&self) -> bool {
        self.eq(&Key::Char(16))
    }

    pub fn is_device_control_one(&self) -> bool {
        self.eq(&Key::Char(17))
    }

    pub fn is_device_control_two(&self) -> bool {
        self.eq(&Key::Char(18))
    }

    pub fn is_device_control_three(&self) -> bool {
        self.eq(&Key::Char(19))
    }

    pub fn is_device_control_four(&self) -> bool {
        self.eq(&Key::Char(20))
    }

    pub fn is_negative_acknowledge(&self) -> bool {
        self.eq(&Key::Char(21))
    }

    pub fn is_synchronous_idle(&self) -> bool {
        self.eq(&Key::Char(22))
    }

    pub fn is_end_transmission_block(&self) -> bool {
        self.eq(&Key::Char(23))
    }

    pub fn is_cancel(&self) -> bool {
        self.eq(&Key::Char(24))
    }

    pub fn is_end_of_medium(&self) -> bool {
        self.eq(&Key::Char(25))
    }

    pub fn is_substitute(&self) -> bool {
        self.eq(&Key::Char(26))
    }

    pub fn is_file_separator(&self) -> bool {
        self.eq(&Key::Char(28))
    }

    pub fn is_group_separator(&self) -> bool {
        self.eq(&Key::Char(29))
    }

    pub fn is_record_separator(&self) -> bool {
        self.eq(&Key::Char(30))
    }

    pub fn is_unit_separator(&self) -> bool {
        self.eq(&Key::Char(31))
    }

    pub fn is_space(&self) -> bool {
        self.eq(&Key::Char(32))
    }

    pub fn is_c1(&self) -> bool {
        match *self {
            Key::Char(e @ 128...159) => e.eq(&e),
            _ => false,
        }
    }

    pub fn as_input(&self) -> (In, libc::size_t) {
        match *self {
            Key::Char(key) => unsafe {
                let mut input: In = In::default();
                let buf: [u8; 8] = mem::transmute::<u64, [u8; 8]>(key);
                let _ = input.write(&buf[..]);

                (input, input.iter().position(|c| c.eq(&b'\0')).unwrap_or_default())
            },
            Key::Str(input) => {
                (input, input.iter().position(|c| c.eq(&b'\0')).unwrap_or_default())
            },
        }
    }
}

impl From<u32> for Key {
    fn from(code: u32) -> Self {
        match code {
            /// Enter.
            13 => Key::Char(ENTER),
            /// BackSpace.
            127 => Key::Char(BACKSPACE),
            /// Up.
            63232 => Key::Char(UP),
            /// Down.
            63233 => Key::Char(DOWN),
            /// Right.
            63235 => Key::Char(RIGHT),
            /// Left.
            63234 => Key::Char(LEFT),
            /// End.
            63275 => Key::Char(END),
            /// Home.
            63273 => Key::Char(HOME),
            /// Delete.
            63272 => Key::Char(DELETE),
            /// Page Up.
            63276 => Key::Char(PAGE_UP),
            /// Page Down.
            63277 => Key::Char(PAGE_DOWN),
            /// Function 1.
            63236 => Key::Char(FUNCTION_1),
            /// Function 2.
            63237 => Key::Char(FUNCTION_2),
            /// Function 3.
            63238 => Key::Char(FUNCTION_3),
            /// Function 4.
            63239 => Key::Char(FUNCTION_4),
            /// Function 5.
            63240 => Key::Char(FUNCTION_5),
            /// Function 6.
            63241 => Key::Char(FUNCTION_6),
            /// Function 7.
            63242 => Key::Char(FUNCTION_7),
            /// Function 8.
            63243 => Key::Char(FUNCTION_8),
            /// Function 9.
            63244 => Key::Char(FUNCTION_9),
            /// Function 10.
            63245 => Key::Char(FUNCTION_10),
            /// Function 11.
            63246 => Key::Char(FUNCTION_11),
            /// Function 12.
            63247 => Key::Char(FUNCTION_12),
            code => Key::Char(code as u64),
        }
    }
}

impl From<(In, libc::size_t)> for Key {
    /// The constructor method `new` returns a parsed Key.
    fn from((buf, len): (In, libc::size_t)) -> Self {
        match &buf[..len] {
            /// Enter.
            &[b'\n'] | &[b'\r'] | &[b'\n', b'\r'] => Key::Char(ENTER),
            /// BackSpace.
            &[b'\x7f'] => Key::Char(BACKSPACE),
            /// Up.
            &[b'\x1B', b'[', b'A'] => Key::Char(UP),
            /// Down.
            &[b'\x1B', b'[', b'B'] => Key::Char(DOWN),
            /// Right.
            &[b'\x1B', b'[', b'C'] => Key::Char(RIGHT),
            /// Left.
            &[b'\x1B', b'[', b'D'] => Key::Char(LEFT),
            /// End.
            &[b'\x1B', b'[', b'F'] => Key::Char(END),
            /// Home.
            &[b'\x1B', b'[', b'1', b'~'] => Key::Char(HOME),
            /// Delete.
            &[b'\x1B', b'[', b'3', b'~'] => Key::Char(DELETE),
            /// Page Up.
            &[b'\x1B', b'[', b'5', b'~'] => Key::Char(PAGE_UP),
            /// Page Down.
            &[b'\x1B', b'[', b'6', b'~'] => Key::Char(PAGE_DOWN),
            /// Alt Up.
            &[b'\x1B', b'[', b'1', b';', b'9', b'A'] => Key::Char(ALT_UP),
            //// Alt Down.
            &[b'\x1B', b'[', b'1', b';', b'9', b'B'] => Key::Char(ALT_DOWN),
            /// Alt Right.
            &[b'\x1B', b'[', b'1', b';', b'9', b'C'] => Key::Char(ALT_RIGHT),
            /// Alt Left.
            &[b'\x1B', b'[', b'1', b';', b'9', b'D'] => Key::Char(ALT_LEFT),
            /// Function 1.
            &[b'\x1B', b'O', b'P'] => Key::Char(FUNCTION_1),
            /// Function 2.
            &[b'\x1B', b'O', b'Q'] => Key::Char(FUNCTION_2),
            /// Function 3.
            &[b'\x1B', b'O', b'R'] => Key::Char(FUNCTION_3),
            /// Function 4.
            &[b'\x1B', b'O', b'S'] => Key::Char(FUNCTION_4),
            /// Function 5.
            &[b'\x1B', b'[', b'1', b'5', b'~'] => Key::Char(FUNCTION_5),
            /// Function 6.
            &[b'\x1B', b'[', b'1', b'7', b'~'] => Key::Char(FUNCTION_6),
            /// Function 7.
            &[b'\x1B', b'[', b'1', b'8', b'~'] => Key::Char(FUNCTION_7),
            /// Function 8.
            &[b'\x1B', b'[', b'1', b'9', b'~'] => Key::Char(FUNCTION_8),
            /// Function 9.
            &[b'\x1B', b'[', b'2', b'0', b'~'] => Key::Char(FUNCTION_9),
            /// Function 10.
            &[b'\x1B', b'[', b'2', b'1', b'~'] => Key::Char(FUNCTION_10),
            /// Function 11.
            &[b'\x1B', b'[', b'2', b'3', b'~'] => Key::Char(FUNCTION_11),
            /// Function 12.
            &[b'\x1B', b'[', b'2', b'4', b'~'] => Key::Char(FUNCTION_12),
            /// Function 13.
            &[b'\x1B', b'[', b'2', b'5', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'P'] => Key::Char(FUNCTION_13),
            /// Function 14.
            &[b'\x1B', b'[', b'2', b'6', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'Q'] => Key::Char(FUNCTION_14),
            /// Function 15.
            &[b'\x1B', b'[', b'2', b'8', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'R'] => Key::Char(FUNCTION_15),
            /// Function 16.
            &[b'\x1B', b'[', b'2', b'9', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'S'] => Key::Char(FUNCTION_16),
            /// Function 17.
            &[b'\x1B', b'[', b'3', b'1', b'~'] => Key::Char(FUNCTION_17),
            /// Function 18.
            &[b'\x1B', b'[', b'3', b'2', b'~'] => Key::Char(FUNCTION_18),
            /// Function 19.
            &[b'\x1B', b'[', b'3', b'3', b'~'] => Key::Char(FUNCTION_19),
            /// Byte 0.
            &[u1] => Key::Char(u1 as u64),
            /// Byte 1.
            &[u1, u2] => unsafe {
                Key::Char(mem::transmute::<[u8; 2], u16>([u1, u2]) as u64)
            },
            /// Byte 2.
            &[u1, u2, u3]  => unsafe {
                Key::Char(mem::transmute::<[u8; 4], u32>([u1, u2, u3, b'\0']) as u64)
            },
            /// Byte 3.
            &[u1, u2, u3, u4] => unsafe {
                Key::Char(mem::transmute::<[u8; 4], u32>([u1, u2, u3, u4]) as u64)
            },
            _ => Key::Str(buf),
        }
    }
}
