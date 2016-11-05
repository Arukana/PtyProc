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
/// fn goto_home(&mut self) -> io::Result<libc::size_t>
fn test_home()
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

  // Print "hello"
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home
  assert_eq!(display.write(b"\x1B[H").ok(), Some(0usize));

  // Print "world"
  assert_eq!(display.write(b"world").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'w', b'o', b'r', b'l', b'd', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }


#[test]
/// These functions in `test_move` are tested with argument 'mv' == 1.
/// fn goto_up(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_down(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_right(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_left(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
fn test_move()
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

  // Print "hello"
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // 3 times Goto::Down
  assert_eq!(display.write(b"\x1B[B\x1B[B\x1B[B").ok(), Some(0usize));

  // Print "hi"
  assert_eq!(display.write(b"hi").ok(), Some(2usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'i', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // 2 times Goto::Left, then 1 time Goto::Up
  assert_eq!(display.write(b"\x1B[D\x1B[D\x1B[A").ok(), Some(0usize));

  // Print "yolo"
  assert_eq!(display.write(b"yolo").ok(), Some(4usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'y', b'o', b'l', b'o', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'i', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home, then 1 time Goto::Down, then 2 times Goto::Right
  assert_eq!(display.write(b"\x1B[H\x1B[B\x1B[C\x1B[C").ok(), Some(0usize));

  // Print "bonjour"
  assert_eq!(display.write(b"bonjour").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ',
           b' ', b' ', b' ', b' ', b' ', b'y', b'o', b'l', b'o', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'i', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }



#[test]
/// fn goto_up(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_down(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_right(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_left(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
fn test_n_move()
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

  // Print "hello"
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // 6 times Goto::Down
  assert_eq!(display.write(b"\x1B[6B").ok(), Some(0usize));

  // Print "hey"
  assert_eq!(display.write(b"hey").ok(), Some(3usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'e', b'y', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // 3 times Goto::Up, then 3 times Goto::Left
  assert_eq!(display.write(b"\x1B[3A\x1B[3D").ok(), Some(0usize));

  // Print "yolo"
  assert_eq!(display.write(b"yolo").ok(), Some(4usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'y', b'o', b'l', b'o', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'e', b'y', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::Home, then 2 times Goto::Down, then 2 times Goto::Right
  assert_eq!(display.write(b"\x1B[H\x1B[2B\x1B[2C").ok(), Some(0usize));

  // Print "bonjour"
  assert_eq!(display.write(b"bonjour").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ',
           b' ', b' ', b' ', b' ', b' ', b'y', b'o', b'l', b'o', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'e', b'y', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }


#[test]
/// fn goto_coord(&mut self, x: libc::size_t, y: libc::size_t)
fn test_move_coord()
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

  // Print "hello"
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::(1, 7)
  assert_eq!(display.write(b"\x1B[8;2H").ok(), Some(0usize));

  // Print "coucou"
  assert_eq!(display.write(b"coucou").ok(), Some(6usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b'c', b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ' ]);

  // Goto::(6, 3)
  assert_eq!(display.write(b"\x1B[4;7H").ok(), Some(0usize));

  // Print "hi"
  assert_eq!(display.write(b"hi").ok(), Some(2usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b'h', b'i', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b'c', b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ' ]); }

#[test]
/// fn save_position(&mut self)
/// fn restore_position(&mut self)
fn test_position_save()
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

  // Print "hello"
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Position Save
  assert_eq!(display.write(b"\x1B[s").ok(), Some(0usize));

  // Print " lorem ipsum dolor sit amet"
  assert_eq!(display.write(b" lorem ipsum dolor sit amet").ok(), Some(27usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Position Restore
  assert_eq!(display.write(b"\x1B[u").ok(), Some(0usize));

  // Print " rust c'est la vie! "
  assert_eq!(display.write(b" rust c est la vie! ").ok(), Some(20usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'r', b'u', b's', b't',
           b' ', b'c', b' ', b'e', b's', b't', b' ', b'l', b'a', b' ',
           b'v', b'i', b'e', b'!', b' ', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Position Save
  assert_eq!(display.write(b"\x1B[s").ok(), Some(0usize));

  // Print "bonjour"
  assert_eq!(display.write(b"bonjour").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'r', b'u', b's', b't',
           b' ', b'c', b' ', b'e', b's', b't', b' ', b'l', b'a', b' ',
           b'v', b'i', b'e', b'!', b' ', b'b', b'o', b'n', b'j', b'o',
           b'u', b'r', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Position Restore
  assert_eq!(display.write(b"\x1B[u").ok(), Some(0usize));

  // Print "COUCOU!"
  assert_eq!(display.write(b"COUCOU!").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'r', b'u', b's', b't',
           b' ', b'c', b' ', b'e', b's', b't', b' ', b'l', b'a', b' ',
           b'v', b'i', b'e', b'!', b' ', b'C', b'O', b'U', b'C', b'O',
           b'U', b'!', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }
