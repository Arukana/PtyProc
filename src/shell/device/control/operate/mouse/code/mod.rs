mod err;

pub use self::err::{CodeError, Result};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Code {
    /// The left mouse button is pressed.
    Left = 0,
    /// The right mouse button is pressed.
    Right = 1,
    /// The mouse wheel button is pressed.
    Wheel = 2,
    /// The mouse wheel is going up.
    WheelUp = 64,
    /// The mouse wheel is going down.
    WheelDown = 65,
    /// The left mouse button is held while moving pointer.
    LeftDrag = 32,
    /// The wheel mouse button is held while moving pointer.
    WheelDrag = 33,
    /// The right mouse button is held while moving pointer.
    RightDrag = 34,
    /// The left mouse button is pressed while helding Shift.
    ShiftLeft = 4,
    /// The wheel mouse button is pressed while helding Shift.
    ShiftWheel = 5,
    /// The right mouse button is pressed while helding Shift.
    ShiftRight = 6,
    /// The left mouse button and Shift are held while moving pointer.
    ShiftLeftDrag = 36,
    /// The wheel mouse button and Shift are held while moving pointer.
    ShiftWheelDrag = 37,
    /// The right mouse button and Shift are held while moving pointer.
    ShiftRightDrag = 38,
    /// The left mouse button is pressed while helding Ctrl
    CtrlLeft = 16,
    /// The wheel mouse button is pressed while helding Ctrl
    CtrlWheel = 17,
    /// The right mouse button is pressed while helding Ctrl
    CtrlRight = 18,
    /// The mouse wheel is going up while helding Ctrl
    CtrlWheelUp = 80,
    /// The mouse wheel is going down while helding Ctrl
    CtrlWheelDown = 81,
    /// The left mouse button and Ctrl are held while moving pointer
    CtrlLeftDrag = 48,
    /// The wheel mouse button and Ctrl are held while moving pointer
    CtrlWheelDrag = 49,
    /// The right mouse button and Ctrl are held while moving pointer
    CtrlRightDrag = 50,
    /// The left mouse button is pressed while Ctrl and Shift are held
    ShiftCtrlLeft = 20,
    /// The wheel mouse button is pressed while Ctrl and Shift are held
    ShiftCtrlWheel = 21,
    /// The right mouse button is pressed while Ctrl and Shift are held
    ShiftCtrlRight = 22,
    /// The left mouse button, Ctrl and Shift are held while moving pointer
    ShiftCtrlLeftDrag = 52,
    /// The wheel mouse button, Ctrl and Shift are held while moving pointer
    ShiftCtrlWheelDrag = 53,
    /// The right mouse button, Ctrl and Shift are held while moving pointer
    ShiftCtrlRightDrag = 54,
    /// The left mouse button is pressed while helding Command
    CmdLeft = 8,
    /// The wheel mouse button is pressed while helding Command
    CmdWheel = 9,
    /// The right mouse button is pressed while helding Command
    CmdRight = 10,
    /// The mouse wheel is going up while helding Command
    CmdWheelUp = 72,
    /// The mouse wheel is going down while helding Command
    CmdWheelDown = 73,
    /// The left mouse button and Command are held while moving pointer
    CmdLeftDrag = 40,
    /// The wheel mouse button and Command are held while moving pointer
    CmdWheelDrag = 41,
    /// The right mouse button and Command are held while moving pointer
    CmdRightDrag = 42,
    /// The left mouse button is pressed while helding Command and Shift
    CmdShiftLeft = 12,
    /// The wheel mouse button is pressed while helding Command and Shift
    CmdShiftWheel = 13,
    /// The right mouse button is pressed while helding Command and Shift
    CmdShiftRight = 14,
    /// The left mouse button, Shift and Command are held while moving pointer
    CmdShiftLeftDrag = 44,
    /// The wheel mouse button, Shift and Command are held while moving pointer
    CmdShiftWheelDrag = 45,
    /// The right mouse button, Shift and Command are held while moving pointer
    CmdShiftRightDrag = 46,
    /// The left mouse button is pressed while helding Command and Ctrl
    CmdCtrlLeft = 24,
    /// The wheel mouse button is pressed while helding Command and Ctrl
    CmdCtrlWheel = 25,
    /// The right mouse button is pressed while helding Command and Ctrl
    CmdCtrlRight = 26,
    /// The mouse wheel is going up while helding Command and Ctrl
    CmdCtrlWheelUp = 88,
    /// The mouse wheel is going down while helding Command and Ctrl
    CmdCtrlWheelDown = 89,
    /// The left mouse button, Ctrl and Command are held while moving pointer
    CmdCtrlLeftDrag = 56,
    /// The wheel mouse button, Ctrl and Command are held while moving pointer
    CmdCtrlWheelDrag = 57,
    /// The right mouse button, Ctrl and Command are held while moving pointer
    CmdCtrlRightDrag = 58,
    /// The left mouse button is pressed while helding Command, Shift and Ctrl
    CmdShiftCtrlLeft = 28,
    /// The wheel mouse button is pressed while helding Command, Shift and Ctrl
    CmdShiftCtrlWheel = 29,
    /// The right mouse button is pressed while helding Command, Shift and Ctrl
    CmdShiftCtrlRight = 30,
    /// The left mouse button, Ctrl, Shift and Command are held while moving pointer
    CmdShiftCtrlLeftDrag = 60,
    /// The wheel mouse button, Ctrl, Shift and Command are held while moving pointer
    CmdShiftCtrlWheelDrag = 61,
    /// The right mouse button, Ctrl, Shift and Command are held while moving pointer
    CmdShiftCtrlRightDrag = 62,
}

impl Code {
    pub fn new(action: u8) -> Result<Self> {
        match action {
            b'\x00' => Ok(Code::Left),
            b'\x01' => Ok(Code::Wheel),
            b'\x02' => Ok(Code::Right),
            b'\x40' => Ok(Code::WheelUp),
            b'\x41' => Ok(Code::WheelDown),

            /// Drag.
            b'\x20' => Ok(Code::LeftDrag),
            b'\x21' => Ok(Code::WheelDrag),
            b'\x22' => Ok(Code::RightDrag),

            /// Shift Click.
            b'\x04' => Ok(Code::ShiftLeft),
            b'\x05' => Ok(Code::ShiftWheel),
            b'\x06' => Ok(Code::ShiftRight),

            /// Shift Drag.
            b'\x24' => Ok(Code::ShiftLeftDrag),
            b'\x25' => Ok(Code::ShiftWheelDrag),
            b'\x26' => Ok(Code::ShiftRightDrag),

            /// Control Click.
            b'\x10' => Ok(Code::CtrlLeft),
            b'\x11' => Ok(Code::CtrlWheel),
            b'\x12' => Ok(Code::CtrlRight),
            b'\x50' => Ok(Code::CtrlWheelUp),
            b'\x51' => Ok(Code::CtrlWheelDown),

            /// Control Drag.
            b'\x30' => Ok(Code::CtrlLeftDrag),
            b'\x31' => Ok(Code::CtrlWheelDrag),
            b'\x32' => Ok(Code::CtrlRightDrag),

            /// Control Shift Click.
            b'\x14' => Ok(Code::ShiftCtrlLeft),
            b'\x15' => Ok(Code::ShiftCtrlWheel),
            b'\x16' => Ok(Code::ShiftCtrlRight),

            /// Control Shift Drag.
            b'\x34' => Ok(Code::ShiftCtrlLeftDrag),
            b'\x35' => Ok(Code::ShiftCtrlWheelDrag),
            b'\x36' => Ok(Code::ShiftCtrlRightDrag),

            /// Command Click.
            b'\x08' => Ok(Code::CmdLeft),
            b'\x09' => Ok(Code::CmdWheel),
            b'\x0A' => Ok(Code::CmdRight),
            b'\x48' => Ok(Code::CmdWheelUp),
            b'\x49' => Ok(Code::CmdWheelDown),

            /// Command Drag.
            b'\x28' => Ok(Code::CmdLeftDrag),
            b'\x29' => Ok(Code::CmdWheelDrag),
            b'\x2A' => Ok(Code::CmdRightDrag),

            /// Command Shift Click.
            b'\x0C' => Ok(Code::CmdShiftLeft),
            b'\x0D' => Ok(Code::CmdShiftWheel),
            b'\x0E' => Ok(Code::CmdShiftRight),

            /// Command Shift Drag.
            b'\x2C' => Ok(Code::CmdShiftLeftDrag),
            b'\x2D' => Ok(Code::CmdShiftWheelDrag),
            b'\x2E' => Ok(Code::CmdShiftRightDrag),

            /// Command Control Click.
            b'\x18' => Ok(Code::CmdCtrlLeft),
            b'\x19' => Ok(Code::CmdCtrlWheel),
            b'\x1A' => Ok(Code::CmdCtrlRight),
            b'\x58' => Ok(Code::CmdWheelUp),
            b'\x59' => Ok(Code::CmdWheelDown),

            /// Command Control Drag.
            b'\x38' => Ok(Code::CmdCtrlLeftDrag),
            b'\x39' => Ok(Code::CmdCtrlWheelDrag),
            b'\x3A' => Ok(Code::CmdCtrlRightDrag),

            /// Command Shift Control Click.
            b'\x1C' => Ok(Code::CmdShiftCtrlLeft),
            b'\x1D' => Ok(Code::CmdShiftCtrlWheel),
            b'\x1E' => Ok(Code::CmdShiftCtrlRight),

            /// Command Shift Control Drag.
            b'\x3C' => Ok(Code::CmdShiftCtrlLeftDrag),
            b'\x3E' => Ok(Code::CmdShiftCtrlWheelDrag),
            b'\x3F' => Ok(Code::CmdShiftCtrlRightDrag),
            _ => Err(CodeError::NotImplemented),
        }
    }
}

impl From<u8> for Code {
    fn from(action: u8) -> Code {
        Code::new(action).ok().unwrap_or_default()
    }
}

impl Default for Code {
    fn default() -> Code {
        Code::Left
    }
}
