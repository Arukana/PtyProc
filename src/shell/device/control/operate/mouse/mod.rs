mod err;

pub use self::err::{MouseError, Result};

/// Note that the button "Command" is like the function button
/// between Ctrl and Alt on Azerty keyboards
#[derive(Clone, Copy, Debug)]
pub enum Mouse {
   /// The left mouse button is pressed.
   Left = 0isize,
   /// The right mouse button is pressed.
   Right = 1isize,
   /// The mouse wheel button is pressed.
   Wheel = 2isize,
   /// The mouse wheel is going up.
   WheelUp = 64isize,
   /// The mouse wheel is going down.
   WheelDown = 65isize,
   /// The left mouse button is held while moving pointer.
   LeftDrag = 32isize,
   /// The wheel mouse button is held while moving pointer.
   WheelDrag = 33isize,
   /// The right mouse button is held while moving pointer.
   RightDrag = 34isize,
   /// The left mouse button is pressed while helding Shift.
   ShiftLeft = 4isize,
   /// The wheel mouse button is pressed while helding Shift.
   ShiftWheel = 5isize,
   /// The right mouse button is pressed while helding Shift.
   ShiftRight = 6isize,
   /// The left mouse button and Shift are held while moving pointer.
   ShiftLeftDrag = 36isize,
   /// The wheel mouse button and Shift are held while moving pointer.
   ShiftWheelDrag = 37isize,
   /// The right mouse button and Shift are held while moving pointer.
   ShiftRightDrag = 38isize,
   /// The left mouse button is pressed while helding Ctrl
   CtrlLeft = 16isize,
   /// The wheel mouse button is pressed while helding Ctrl
   CtrlWheel = 17isize,
   /// The right mouse button is pressed while helding Ctrl
   CtrlRight = 18isize,
   /// The mouse wheel is going up while helding Ctrl
   CtrlWheelUp = 80isize,
   /// The mouse wheel is going down while helding Ctrl
   CtrlWheelDown = 81isize,
   /// The left mouse button and Ctrl are held while moving pointer
   CtrlLeftDrag = 48isize,
   /// The wheel mouse button and Ctrl are held while moving pointer
   CtrlWheelDrag = 49isize,
   /// The right mouse button and Ctrl are held while moving pointer
   CtrlRightDrag = 50isize,
   /// The left mouse button is pressed while Ctrl and Shift are held
   ShiftCtrlLeft = 20isize,
   /// The wheel mouse button is pressed while Ctrl and Shift are held
   ShiftCtrlWheel = 21isize,
   /// The right mouse button is pressed while Ctrl and Shift are held
   ShiftCtrlRight = 22isize,
   /// The left mouse button, Ctrl and Shift are held while moving pointer
   ShiftCtrlLeftDrag = 52isize,
   /// The wheel mouse button, Ctrl and Shift are held while moving pointer
   ShiftCtrlWheelDrag = 53isize,
   /// The right mouse button, Ctrl and Shift are held while moving pointer
   ShiftCtrlRightDrag = 54isize,
   /// The left mouse button is pressed while helding Command
   CmdLeft = 8isize,
   /// The wheel mouse button is pressed while helding Command
   CmdWheel = 9isize,
   /// The right mouse button is pressed while helding Command
   CmdRight = 10isize,
   /// The mouse wheel is going up while helding Command
   CmdWheelUp = 72isize,
   /// The mouse wheel is going down while helding Command
   CmdWheelDown = 73isize,
   /// The left mouse button and Command are held while moving pointer
   CmdLeftDrag = 40isize,
   /// The wheel mouse button and Command are held while moving pointer
   CmdWheelDrag = 41isize,
   /// The right mouse button and Command are held while moving pointer
   CmdRightDrag = 42isize,
   /// The left mouse button is pressed while helding Command and Shift
   CmdShiftLeft = 12isize,
   /// The wheel mouse button is pressed while helding Command and Shift
   CmdShiftWheel = 13isize,
   /// The right mouse button is pressed while helding Command and Shift
   CmdShiftRight = 14isize,
   /// The left mouse button, Shift and Command are held while moving pointer
   CmdShiftLeftDrag = 44isize,
   /// The wheel mouse button, Shift and Command are held while moving pointer
   CmdShiftWheelDrag = 45isize,
   /// The right mouse button, Shift and Command are held while moving pointer
   CmdShiftRightDrag = 46isize,
   /// The left mouse button is pressed while helding Command and Ctrl
   CmdCtrlLeft = 24isize,
   /// The wheel mouse button is pressed while helding Command and Ctrl
   CmdCtrlWheel = 25isize,
   /// The right mouse button is pressed while helding Command and Ctrl
   CmdCtrlRight = 26isize,
   /// The mouse wheel is going up while helding Command and Ctrl
   CmdCtrlWheelUp = 88isize,
   /// The mouse wheel is going down while helding Command and Ctrl
   CmdCtrlWheelDown = 89isize,
   /// The left mouse button, Ctrl and Command are held while moving pointer
   CmdCtrlLeftDrag = 56isize,
   /// The wheel mouse button, Ctrl and Command are held while moving pointer
   CmdCtrlWheelDrag = 57isize,
   /// The right mouse button, Ctrl and Command are held while moving pointer
   CmdCtrlRightDrag = 58isize,
   /// The left mouse button is pressed while helding Command, Shift and Ctrl
   CmdShiftCtrlLeft = 28isize,
   /// The wheel mouse button is pressed while helding Command, Shift and Ctrl
   CmdShiftCtrlWheel = 29isize,
   /// The right mouse button is pressed while helding Command, Shift and Ctrl
   CmdShiftCtrlRight = 30isize,
   /// The left mouse button, Ctrl, Shift and Command are held while moving pointer
   CmdShiftCtrlLeftDrag = 60isize,
   /// The wheel mouse button, Ctrl, Shift and Command are held while moving pointer
   CmdShiftCtrlWheelDrag = 61isize,
   /// The right mouse button, Ctrl, Shift and Command are held while moving pointer
   CmdShiftCtrlRightDrag = 62isize,
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
