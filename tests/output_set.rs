extern crate pty_proc;
extern crate libc;

use std::io::Write;

use self::pty_proc::prelude::*;

const SIZE: Winszed = Winszed
{ ws_row: 8,
  ws_col: 10,
  ws_xpixel: 0,
  ws_ypixel: 0, };


#[test]
fn test_output_set()
{
    let mut display: Display = Display::from_winszed(SIZE);

    display.write(&[97u8, 100, 106, 105, 118, 97, 115, 64, 37, 109, 58, 37, 126, 47, 32, 40, 109, 97, 115, 116, 101,
    114, 41, 32]);
}
