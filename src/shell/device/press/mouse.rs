pub enum Mouse {
  /// The left mouse button is pressed.
  Left,
  /// The right mouse button is pressed.
  Right,
  /// The mouse wheel button is pressed.
  Wheel,
  /// The mouse wheel is going up.
  WheelUp,
  /// The mouse wheel is going down.
  WheelDown,
  /// The left mouse button is held while moving pointer.
  LeftDrag,
  /// The wheel mouse button is held while moving pointer.
  WheelDrag,
  /// The right mouse button is held while moving pointer.
  RightDrag,
  /// The left mouse button is pressed while helding Shift.
  ShiftLeft,
  /// The wheel mouse button is pressed while helding Shift.
  ShiftWheel,
  /// The right mouse button is pressed while helding Shift.
  ShiftRight,
  /// The left mouse button and Shift are held while moving pointer.
  ShiftLeftDrag,
  /// The wheel mouse button and Shift are held while moving pointer.
  ShiftWheelDrag,
  /// The right mouse button and Shift are held while moving pointer.
  ShiftRightDrag,
  /// The left mouse button is pressed while helding Ctrl
  CtrlLeft,
  /// The wheel mouse button is pressed while helding Ctrl
  CtrlWheel,
  /// The right mouse button is pressed while helding Ctrl 
  CtrlRight,
  /// The mouse wheel is going up while helding Ctrl 
  CtrlWheelUp,
  /// The mouse wheel is going down while helding Ctrl 
  CtrlWheelDown,
  /// The left mouse button and Ctrl are held while moving pointer
  CtrlLeftDrag,
  /// The wheel mouse button and Ctrl are held while moving pointer
  CtrlWheelDrag,
  /// The right mouse button and Ctrl are held while moving pointer
  CtrlRightDrag,
  /// The left mouse button is pressed while Ctrl and Shift are held
  ShiftCtrlLeft,
  /// The wheel mouse button is pressed while Ctrl and Shift are held
  ShiftCtrlWheel,
  /// The right mouse button is pressed while Ctrl and Shift are held
  ShiftCtrlRight,
  /// The left mouse button, Ctrl and Shift are held while moving pointer
  ShiftCtrlLeftDrag,
  /// The wheel mouse button, Ctrl and Shift are held while moving pointer
  ShiftCtrlWheelDrag,
  /// The right mouse button, Ctrl and Shift are held while moving pointer
  ShiftCtrlRightDrag,
}
