#[derive(Clone, Copy, Debug)]
/// Note that the button "Command" is like the function button
/// between Ctrl and Alt on Azerty keyboards
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
