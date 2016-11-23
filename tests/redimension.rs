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
#[ignore] // For travis
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

  // Print " A"
  assert_eq!(display.write(b" A").ok(), Some(2usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm', b'e', b't', b' ', b'b', b'o', b'n', b'j',
           b'o', b'u', b'r', b' ', b'A', b' ', b' ', b' ', b' ', b' ',
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

  // Print "A"
  assert_eq!(display.write(b"A").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'A',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);


  winsz = Winszed
  { ws_row: 8,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Add 3 lines to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'A',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "Q"
  assert_eq!(display.write(b"Q").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'A',
           b'Q', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
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
vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o', b' ', b' ',
     b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm', b' ', b' ',
     b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l', b' ', b' ',
     b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'A', b' ', b' ',
     b'Q', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "B"
  assert_eq!(display.write(b"B").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o', b' ', b' ',
     b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm', b' ', b' ',
     b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l', b' ', b' ',
     b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'A', b' ', b' ',
     b'Q', b'B', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
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
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's',
           b'Q', b'B', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "A"
  assert_eq!(display.write(b"A").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's',
           b'Q', b'B', b'A', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &stock); }}

#[test]
//#[ignore] // For travis
/// fn resize(&mut self) -> Result<()>
fn hard_redimension()
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

  let mut winsz = Winszed
  { ws_row: 5,
    ws_col: 7,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Remove 3 lines and 3 columns to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b'l', b'o', b'r', b' ', b's', b'i', b't',
           b'e', b't', b' ', b'h', b'e', b'l', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p' ]);

  // Print "A"
  assert_eq!(display.write(b"A").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b'l', b'o', b'r', b' ', b's', b'i', b't',
           b'e', b't', b' ', b'h', b'e', b'l', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'A',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  winsz = Winszed
  { ws_row: 10,
    ws_col: 12,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Add 5 lines and 5 columns to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b' ', b' ', b' ', b' ',
     b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b' ', b' ', b' ', b' ',
     b'e', b't', b' ', b'h', b'e', b'l', b'l', b' ', b' ', b' ', b' ', b' ',
     b'o', b'r', b'e', b'm', b' ', b'i', b'A', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "B"
  assert_eq!(display.write(b"B").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b' ', b' ', b' ', b' ',
     b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b' ', b' ', b' ', b' ',
     b'e', b't', b' ', b'h', b'e', b'l', b'l', b' ', b' ', b' ', b' ', b' ',
     b'o', b'r', b'e', b'm', b' ', b'i', b'A', b' ', b' ', b' ', b' ', b' ',
     b'B', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  winsz = Winszed
  { ws_row: 12,
    ws_col: 5,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Add 2 lines and remove 7 columns to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
vec![b'm', b' ', b'i', b'p', b's',
     b'l', b'o', b'r', b' ', b's',
     b'e', b't', b' ', b'h', b'e',
     b'o', b'r', b'e', b'm', b' ',
     b'B', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ' ]);

  // Print "X"
  assert_eq!(display.write(b"X").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
vec![b'm', b' ', b'i', b'p', b's',
     b'l', b'o', b'r', b' ', b's',
     b'e', b't', b' ', b'h', b'e',
     b'o', b'r', b'e', b'm', b' ',
     b'B', b'X', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ',
     b' ', b' ', b' ', b' ', b' ' ]);

  winsz = Winszed
  { ws_row: 3,
    ws_col: 10,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Remove 9 lines and add 5 columns to the display
  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &winsz); }
  display.resize();
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b' ', b' ', b' ', b' ', b' ',
           b'l', b'o', b'r', b' ', b's', b' ', b' ', b' ', b' ', b' ',
           b'e', b't', b' ', b'h', b'e', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "S"
  assert_eq!(display.write(b"S").ok(), Some(1usize));
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b' ', b' ', b' ', b' ', b' ',
           b'l', b'o', b'r', b' ', b's', b' ', b' ', b' ', b' ', b' ',
           b'e', b't', b' ', b'h', b'S', b' ', b' ', b' ', b' ', b' ' ]);

  unsafe { libc::ioctl(0, libc::TIOCSWINSZ, &stock); }}
