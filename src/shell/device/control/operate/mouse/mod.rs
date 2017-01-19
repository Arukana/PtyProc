mod err;

pub use self::err::{MouseError, Result};

/// Note that the button "Command" is like the function button
/// between Ctrl and Alt on Azerty keyboards
#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Mouse {
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

impl Mouse {
  pub fn new(action: u8) -> Result<Self> {
    match action {
      b'\x00' => Ok(Mouse::Left),
      b'\x01' => Ok(Mouse::Wheel),
      b'\x02' => Ok(Mouse::Right),
      b'\x40' => Ok(Mouse::WheelUp),
      b'\x41' => Ok(Mouse::WheelDown),

      /// Drag.
      b'\x20' => Ok(Mouse::LeftDrag),
      b'\x21' => Ok(Mouse::WheelDrag),
      b'\x22' => Ok(Mouse::RightDrag),

      /// Shift Click.
      b'\x04' => Ok(Mouse::ShiftLeft),
      b'\x05' => Ok(Mouse::ShiftWheel),
      b'\x06' => Ok(Mouse::ShiftRight),

      /// Shift Drag.
      b'\x24' => Ok(Mouse::ShiftLeftDrag),
      b'\x25' => Ok(Mouse::ShiftWheelDrag),
      b'\x26' => Ok(Mouse::ShiftRightDrag),

      /// Control Click.
      b'\x10' => Ok(Mouse::CtrlLeft),
      b'\x11' => Ok(Mouse::CtrlWheel),
      b'\x12' => Ok(Mouse::CtrlRight),
      b'\x50' => Ok(Mouse::CtrlWheelUp),
      b'\x51' => Ok(Mouse::CtrlWheelDown),

      /// Control Drag.
      b'\x30' => Ok(Mouse::CtrlLeftDrag),
      b'\x31' => Ok(Mouse::CtrlWheelDrag),
      b'\x32' => Ok(Mouse::CtrlRightDrag),

      /// Control Shift Click.
      b'\x14' => Ok(Mouse::ShiftCtrlLeft),
      b'\x15' => Ok(Mouse::ShiftCtrlWheel),
      b'\x16' => Ok(Mouse::ShiftCtrlRight),

      /// Control Shift Drag.
      b'\x34' => Ok(Mouse::ShiftCtrlLeftDrag),
      b'\x35' => Ok(Mouse::ShiftCtrlWheelDrag),
      b'\x36' => Ok(Mouse::ShiftCtrlRightDrag),

      /// Command Click.
      b'\x08' => Ok(Mouse::CmdLeft),
      b'\x09' => Ok(Mouse::CmdWheel),
      b'\x0A' => Ok(Mouse::CmdRight),
      b'\x48' => Ok(Mouse::CmdWheelUp),
      b'\x49' => Ok(Mouse::CmdWheelDown),

      /// Command Drag.
      b'\x28' => Ok(Mouse::CmdLeftDrag),
      b'\x29' => Ok(Mouse::CmdWheelDrag),
      b'\x2A' => Ok(Mouse::CmdRightDrag),

      /// Command Shift Click.
      b'\x0C' => Ok(Mouse::CmdShiftLeft),
      b'\x0D' => Ok(Mouse::CmdShiftWheel),
      b'\x0E' => Ok(Mouse::CmdShiftRight),

      /// Command Shift Drag.
      b'\x2C' => Ok(Mouse::CmdShiftLeftDrag),
      b'\x2D' => Ok(Mouse::CmdShiftWheelDrag),
      b'\x2E' => Ok(Mouse::CmdShiftRightDrag),

      /// Command Control Click.
      b'\x18' => Ok(Mouse::CmdCtrlLeft),
      b'\x19' => Ok(Mouse::CmdCtrlWheel),
      b'\x1A' => Ok(Mouse::CmdCtrlRight),
      b'\x58' => Ok(Mouse::CmdWheelUp),
      b'\x59' => Ok(Mouse::CmdWheelDown),

      /// Command Control Drag.
      b'\x38' => Ok(Mouse::CmdCtrlLeftDrag),
      b'\x39' => Ok(Mouse::CmdCtrlWheelDrag),
      b'\x3A' => Ok(Mouse::CmdCtrlRightDrag),

      /// Command Shift Control Click.
      b'\x1C' => Ok(Mouse::CmdShiftCtrlLeft),
      b'\x1D' => Ok(Mouse::CmdShiftCtrlWheel),
      b'\x1E' => Ok(Mouse::CmdShiftCtrlRight),

      /// Command Shift Control Drag.
      b'\x3C' => Ok(Mouse::CmdShiftCtrlLeftDrag),
      b'\x3E' => Ok(Mouse::CmdShiftCtrlWheelDrag),
      b'\x3F' => Ok(Mouse::CmdShiftCtrlRightDrag),
      _ => Err(MouseError::NotImplemented),
    }
  }
}
