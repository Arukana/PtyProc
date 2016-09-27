use ::libc;

#[derive(Clone, Copy, Debug)]
pub enum Cursor {
  
  ///-------- Terminal setups ---------
  ///Cursor automatically wraps to next line when it reach the border column
  LineWrap(bool),
  ///Enable the scrolling
  ScrollEnable,
  ///Terminal setups are rebooted to initial state
  TermReset,

  ///------- Cursor movements ---------
  ///Cursor moving to new coordinates (column index, line index)
  CursorGoto(libc::c_ushort, libc::c_ushort),
  ///Cursor is going up (number of ceils to travel)
  CursorUp(libc::c_ushort),
  ///Cursor is going down (number of ceils to travel)
  CursorDown(libc::c_ushort),
  ///Cursor is going right (number of ceils to travel)
  CursorRight(libc::c_ushort),
  ///Cursor is going left (number of ceils to travel)
  CursorLeft(libc::c_ushort),

  ///------- Position saving ---------
  ///Save the actual cursor position
  ///Only one cursor-coordinates can be safe at the same time, each SaveCursor call will replace the oldest coordinates by the newest
  SaveCursor,
  ///Cursor position is moved to its safe coordinates
  ///If no cursor-coordinates was safe with SaveCursor before calling RestoreCursor, this call does nothing
  ///Many RestoreCursor calls can be done, it will always move the cursor to the safe coordinates
  RestoreCursor,

  ///---------- Scrolling -----------
  ///Scroll the terminal one line up
  ScrollUp,
  ///Scroll the terminal one line down
  ScrollDown,

  ///----------- Erasing ------------
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
  ///The output screen is cleared
  Clear,


  /* ****************** FEATURES ***********************
  ///-- Save Cursor ---
  --> Save multiple Cursor positions

  ///----- Scroll -----
  --> ///Scrolling enable flag for a definited part of the terminal (flag on/off, index of begin_line, index of end_line)
    ScrollEnable(bool, libc::c_ushort, libc::c_ushort),
  --> ///Scroll the terminal one line right
    ScrollRight,
  --> ///Scroll the terminal one line left
    ScrollLeft,
  ********************* FEATURES *********************** */

  /* *******  MACOS/iTerm2 WTF Facts *******
  --> <ESC>[5i Close the fd 0 instead of echoing input on a printer
  --> <ESC>[g && <ESC>[3g Do some WTF things with output tabulations
  ****************************************** */

}

impl Cursor
{ pub fn new() -> Self
  { Cursor::Clear }}
