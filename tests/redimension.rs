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
/// fn resize(&mut self) -> Result<()>
fn test_redimension()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Stock Terminal size for the reset
  let stock : Winszed = Winszed::default();

  // Print "hello lorem ipsum dolor sit amet hello lorem ipsum dolor sit amet bonjour"
  assert_eq!(display.write(b"hello lorem ipsum dolor sit amet hello lorem ipsum dolor sit amet bonjour").ok(), Some(73usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm', b'e', b't', b' ', b'b', b'o', b'n', b'j',
           b'o', b'u', b'r', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  let mut winsz: Winszed = Winszed
  { ws_row: 10,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Add 2 lines to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm', b'e', b't', b' ', b'b', b'o', b'n', b'j',
           b'o', b'u', b'r', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  winsz = Winszed
  { ws_row: 5,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Remove 5 lines to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm' ]);

  winsz = Winszed
  { ws_row: 8,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Add 3 lines to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  winsz = Winszed
  { ws_row: 8,
    ws_col: 12,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Add 2 columns to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e', b' ', b' ',
     b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o', b' ', b' ',
     b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm', b' ', b' ',
     b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l', b' ', b' ',
     b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  winsz = Winszed
  { ws_row: 8,
    ws_col: 8,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Remove 4 columns to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &stock); }
  unsafe { libc::ioctl(0, 536900721); }
}
