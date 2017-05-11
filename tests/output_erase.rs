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
/// fn clear(&mut self) -> io::Result<libc::size_t>
fn test_clear()
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

  // Print "hello lorem ipsum dolor sit amet"
  assert_eq!(display.write(b"hello lorem ipsum dolor sit amet").ok(), Some(32usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Clear Screen
  assert_eq!(display.write(b"\x1B[2J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "hello"
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home, then Print "bonjour"
  assert_eq!(display.write(b"\x1B[Hbonjour").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Clear Screen
  assert_eq!(display.write(b"\x1B[2J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }


#[test]
/// fn erase_right_line(&mut self)
/// fn erase_left_line(&mut self)
/// fn erase_line(&mut self)
fn test_erase_right_left()
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

  // Print "bonjour
  //        hello
  //        hi
  //        coucou
  //        "
  assert_eq!(display.write(b"bonjour\n\rhello world!\n\rhi\n\rcoucou\n\r").ok(), Some(27usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l',
           b'd', b'!', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'c', b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.get_newline(), &vec![(9, 0), (9, 2), (9, 3), (9, 4),
                                       (9, 5), (9, 6), (9, 7)]);

  // Goto::Home, then Goto::Down, then 3 times Goto::Right, then EraseRightLine
  assert_eq!(display.write(b"\x1B[H\x1B[B\x1B[3C\x1B[0K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b'h', b'e', b'l', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'c', b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.get_newline(), &vec![(9, 0), (9, 2), (9, 3), (9, 4),
                                       (9, 5), (9, 6), (9, 7)]);

  // 3 times Goto::Down, then EraseLeftLine
  assert_eq!(display.write(b"\x1B[3B\x1B[1K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b'h', b'e', b'l', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.get_newline(), &vec![(9, 0), (9, 2), (9, 3), (9, 4),
                                       (9, 5), (9, 6), (9, 7)]);

  // 4 times Goto::Up, then EraseLine
  assert_eq!(display.write(b"\x1B[4A\x1B[2K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'e', b'l', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.get_newline(), &vec![(9, 0), (9, 2), (9, 3), (9, 4),
                                       (9, 5), (9, 6), (9, 7)]);

  // Print "coucou"
  assert_eq!(display.write(b"coucou").ok(), Some(6usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b'c', b'o', b'u', b'c', b'o', b'u', b' ',
           b'h', b'e', b'l', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.get_newline(), &vec![(9, 0), (9, 2), (9, 3), (9, 4),
                                       (9, 5), (9, 6), (9, 7)]);

  // 4 times Goto::Left, then EraseRightLine
  assert_eq!(display.write(b"\x1B[4D\x1B[0K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b'c', b'o', b' ', b' ', b' ', b' ', b' ',
           b'h', b'e', b'l', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.get_newline(), &vec![(9, 0), (9, 2), (9, 3), (9, 4),
                                       (9, 5), (9, 6), (9, 7)]); }

#[test]
fn test_erase_up_down()
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

  // Print "hello lorem ipsum dolor sit amet "
  assert_eq!(display.write(b"hello lorem ipsum dolor sit amet ").ok(), Some(33usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "hello lorem ipsum dolor sit amet "
  assert_eq!(display.write(b"hello lorem ipsum dolor sit amet ").ok(), Some(33usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm', b'e', b't', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::(7, 3)
  assert_eq!(display.write(b"\x1B[4;8H").ok(), Some(0usize));

  // EraseDown
  assert_eq!(display.write(b"\x1B[0J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // EraseUp
  assert_eq!(display.write(b"\x1B[1J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }

#[test]
/// fn erase_chars(&mut self, mv: libc::size_t)
fn test_erase_chars()
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

  // Print "hello
  //        bonjour
  //        hi"
  assert_eq!(display.write(b"hello\n\rbonjour\n\rhi").ok(), Some(14usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home, then 3 times EraseChar
  assert_eq!(display.write(b"\x1B[H\x1B[3P").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'l', b'o', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Down, then 50 times EraseChar
  assert_eq!(display.write(b"\x1B[B\x1B[50P").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'l', b'o', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "coucou"
  assert_eq!(display.write(b"coucou").ok(), Some(6usize));
  assert_eq!(display.into_bytes(),
      vec![b'l', b'o', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'c', b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home, then Goto::Down, then 3 times Goto::Right, then 2 times EraseChar
  assert_eq!(display.write(b"\x1B[H\x1B[B\x1B[3C\x1B[2P").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'l', b'o', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'c', b'o', b'u', b'u', b' ', b' ', b' ', b' ', b' ', b' ',
           b'h', b'i', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Clear Screen, then Goto::Home
  assert_eq!(display.write(b"\x1B[2J\x1B[H").ok(), Some(0usize));

  // Print "hello lorem ipsum dolor sit amet 
  //       "
  assert_eq!(display.write(b"hello lorem ipsum dolor sit amet \n\r").ok(), Some(33usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home, then 3 times Goto::Right, then 10 times EraseChar
  assert_eq!(display.write(b"\x1B[H\x1B[3C\x1B[10P").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  }
