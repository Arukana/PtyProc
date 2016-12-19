use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Attribute {
    None = 0x00,
    Dim = 0x02,
    Bold = 0x01,
    Italic = 0x04,
    Underline = 0x08,
    Blink = 0x10,
    Reverse = 0x20,
    Hidden = 0x40,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Attribute::None => "",
            Attribute::Bold => "1;",
            Attribute::Dim => "2;",
            Attribute::Italic => "3;",
            Attribute::Underline => "4;",
            Attribute::Blink => "5;",
            Attribute::Reverse => "7;",
            Attribute::Hidden => "8;",
        })
    }
}
