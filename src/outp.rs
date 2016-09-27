use ::libc;

#[derive(Clone, Copy, Debug)]
pub enum Cursor {
  
  ///-------- Terminal setups ---------
  ///Cursor automatically wraps to next line when it reach the border column
  LineWrap(bool),
  ///Scrolling enable flag for a definited part of the terminal (flag on/off, index of begin_line, index of end_line)
  ScrollEnable(bool, libc::c_ushort, libc::c_ushort),
  ///Terminal setups are rebooted to initial state
  TermReset,

  ///------- Cursor movements ---------
  ///Cursor moving to new coordinates (column index, line index)
  CursorGoto(libc::c_ushort, libc::c_ushort),
  ///Cursor is going up
  CursorUp,
  ///Cursor is going down
  CursorDown,
  ///Cursor is going right
  CursorRight,
  ///Cursor is going left
  CursorLeft,

  ///------- Position saving ---------
  ///Cursor position is saved at some coordinates (column index, line index)
  SaveCursor(libc::c_ushort, libc::c_ushort),
  ///Cursor position is restored to its saved coordinates (column index, line index)
  RestoreCursor(libc::c_ushort, libc::c_ushort),

  ///---------- Scrolling -----------
  ///Scroll the terminal one line up
  ScrollUp,
  ///Scroll the terminal one line down
  ScrollDown,

  /*  FEATURE
  ///Scroll the terminal one line right
  ScrollRight,
  ///Scroll the terminal one line left
  ScrollLeft,
      FEATURE  */

  ///----------- Erasing ------------
  ///The output screen is cleared
  Clear,
  ///Erase the current line from the cursor to the right border column (char under the cursor included)
  EraseRightLine,
  ///Erase the current line from the left border column to the cursor (char under the cursor included)
  EraseLeftLine,
  ///Erase the entire current line
  EraseLine,
  ///Erase all lines from the current line down to the bottom of the screen, and erase the current line from the cursor to the right border column (char under the cursor included)
  EraseDown,
  ///Erase all lines from the current line up to the top of the screen, and erase the current line from the left border column to the cursor (char under the cursor included)
  EraseUp,


  /* *******  MACOS/iTerm2 WTF Facts *******
  --> <ESC>[5i Close the fd 0 instead of echoing input on a printer
  --> <ESC>[g && <ESC>[3g Do some WTF things with output tabulations
  ****************************************** */

}

impl Cursor
{ pub fn new() -> Self
  { Cursor::Clear }}
