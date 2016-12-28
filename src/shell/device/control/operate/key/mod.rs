use std::mem;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Unicode characters.
    Char(u32),
    /// Unicode strings.
    Str(In),
}

impl Key {
    /// The constructor method `new` returns a parsed Key.
    pub fn new(buf: In, len: libc::size_t) -> Self {
        match &buf[..len] {
            /// Up.
            &[b'\x1B', b'[', b'A'] => Key::Char(279165),
            /// Down.
            &[b'\x1B', b'[', b'B'] => Key::Char(279166),
            /// Right.
            &[b'\x1B', b'[', b'C'] => Key::Char(279167),
            /// Left.
            &[b'\x1B', b'[', b'D'] => Key::Char(279168),
            /// Delete.
            &[b'\x1B', b'[', b'3', b'~'] => Key::Char(279151126),
            /// Backspace.
            &[b'\x7F'] => Key::Char(127),
            /// Escape.
            &[b'\x1B'] => Key::Char(27),
            /// End.
            &[b'\x1B', b'[', b'F'] => Key::Char(279152126),
            /// Home.
            &[b'\x1B', b'[', b'1', b'~'] => Key::Char(279149126),
            /// Page Up.
            &[b'\x1B', b'[', b'5', b'~'] => Key::Char(279153126),
            /// Page Down.
            &[b'\x1B', b'[', b'6', b'~'] => Key::Char(279154126),
            /// Function 1.
            &[b'\x1B', b'O', b'P'] => Key::Char(277980),
            /// Function 2.
            &[b'\x1B', b'O', b'Q'] => Key::Char(277981),
            /// Function 3.
            &[b'\x1B', b'O', b'R'] => Key::Char(277982),
            /// Function 4.
            &[b'\x1B', b'O', b'S'] => Key::Char(277983),
            /// Function 5.
            &[b'\x1B', b'[', b'1', b'5', b'~'] => Key::Char(27914953126),
            /// Function 6.
            &[b'\x1B', b'[', b'1', b'7', b'~'] => Key::Char(27914955126),
            /// Function 7.
            &[b'\x1B', b'[', b'1', b'8', b'~'] => Key::Char(27914956126),
            /// Function 8.
            &[b'\x1B', b'[', b'1', b'9', b'~'] => Key::Char(27914957126),
            /// Function 9.
            &[b'\x1B', b'[', b'2', b'0', b'~'] => Key::Char(27915048126),
            /// Function 10.
            &[b'\x1B', b'[', b'2', b'1', b'~'] => Key::Char(27915049126),
            /// Function 11.
            &[b'\x1B', b'[', b'2', b'3', b'~'] => Key::Char(27915051126),
            /// Function 12.
            &[b'\x1B', b'[', b'2', b'4', b'~'] => Key::Char(27915052126),
            /// Function 13.
            &[b'\x1B', b'[', b'2', b'5', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'P'] => Key::Char(27915053126),
            /// Function 14.
            &[b'\x1B', b'[', b'2', b'6', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'Q'] => Key::Char(27915054126),
            /// Function 15.
            &[b'\x1B', b'[', b'2', b'8', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'R'] => Key::Char(27915056126),
            /// Function 16.
            &[b'\x1B', b'[', b'2', b'9', b'~'] | &[b'\x1B', b'[', b'1', b';', b'2', b'S'] => Key::Char(27915057126),
            /// Function 17.
            &[b'\x1B', b'[', b'3', b'1', b'~'] => Key::Char(27915149126),
            /// Function 18.
            &[b'\x1B', b'[', b'3', b'2', b'~'] => Key::Char(27915150126),
            /// Function 19.
            &[b'\x1B', b'[', b'3', b'3', b'~'] => Key::Char(27915151126),
            /// Tabulation.
            &[b'\t'] => Key::Char(9),
            /// Enter.
            &[b'\n'] | &[b'\r'] | &[b'\n', b'\r'] => Key::Char(10),
            /// Alt Up.
            &[b'\x1B', b'[', b'1', b';', b'9', b'A'] => Key::Char(27279165),
            //// Alt Down.
            &[b'\x1B', b'[', b'1', b';', b'9', b'B'] => Key::Char(27279166),
            /// Alt Right.
            &[b'\x1B', b'[', b'1', b';', b'9', b'C'] => Key::Char(27279167),
            /// Alt Left.
            &[b'\x1B', b'[', b'1', b';', b'9', b'D'] => Key::Char(27279168),

            &[u1 @ b'\xF0' ... b'\xF4',
              u2 @ b'\x8F' ... b'\x90',
              u3 @ b'\x80' ... b'\xBF',
              u4 @ b'\x80' ... b'\xBF'] => Key::from_utf8([u1, u2, u3, u4]),
            &[u1 @ b'\xE0' ... b'\xF0', u2 @ b'\x90' ... b'\xA0',
              u3 @ b'\x80' ... b'\xBF'] => Key::from_utf8([u1, u2, u3, b'\x00']),
            &[u1 @ b'\xC2' ... b'\xDF',
              u2 @ b'\x80' ... b'\xBF'] => Key::from_utf8([u1, u2, b'\x00', b'\x00']),
            &[u1] => Key::from_utf8([u1, b'\x00', b'\x00', b'\x00']),
            _ => Key::Str(buf),
        }
    }

    /// The constructor method `from_utf8` returns a UTF-8 parsed Key.
    pub fn from_utf8(buf: [libc::c_uchar; 4]) -> Self {
        unsafe {
            let i: libc::c_uint = mem::transmute::<[libc::c_uchar; 4], libc::c_uint>(buf);

            Key::Char(i)
        }
    }

    /// The accessor method `is_up` returns an Option for the Up Key.
    pub fn is_up(&self) -> bool {
        self.eq(&Key::Char(279165))
    }

    /// The accessor method `is_down` returns an Option for the down Key.
    pub fn is_down(&self) -> bool {
        self.eq(&Key::Char(279166))
    }

    /// The accessor method `is_right` returns an Option for the right Key.
    pub fn is_right(&self) -> bool {
        self.eq(&Key::Char(279167))
    }

    /// The accessor method `is_left` returns an Option for the left Key.
    pub fn is_left(&self) -> bool {
        self.eq(&Key::Char(279168))
    }

    /// The accessor method `is_delete` returns an Option for the delete Key.
    pub fn is_delete(&self) -> bool {
        self.eq(&Key::Char(279151126))
    }

    /// The accessor method `is_backspace` returns an Option for the backspace Key.
    pub fn is_backspace(&self) -> bool {
        self.eq(&Key::Char(127))
    }

    /// The accessor method `is_escape` returns an Option for the delete Key.
    pub fn is_escape(&self) -> bool {
        self.eq(&Key::Char(27))
    }

    /// The accessor method `is_end` returns an Option for the delete Key.
    pub fn is_end(&self) -> bool {
        self.eq(&Key::Char(279152126))
    }

    /// The accessor method `is_home` returns an Option for the delete Key.
    pub fn is_home(&self) -> bool {
        self.eq(&Key::Char(279149126))
    }

    /// The accessor method `is_pageup` returns an Option for the Page Up Key.
    pub fn is_pageup(&self) -> bool {
        self.eq(&Key::Char(279153126))
    }

    /// The accessor method `is_pagedown` returns an Option for the Page Down Key.
    pub fn is_pagedown(&self) -> bool {
        self.eq(&Key::Char(279154126))
    }

    pub fn is_c0(&self) -> bool {
        match *self {
            Key::Char(e @ 0...31) => true,
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
        self.eq(&Key::Char(9))
    }

    /// The accessor method `is_enter` returns an Option for the Enter Key.
    pub fn is_enter(&self) -> bool {
        self.eq(&Key::Char(10))
    }

    pub fn is_line_feed(&self) -> bool {
        self.eq(&Key::Char(10))
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
            Key::Char(e @ 128...159) => true,
            _ => false,
        }
    }
}
