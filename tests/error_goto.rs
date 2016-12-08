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
/// fn goto_up(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_down(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_right(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
/// fn goto_left(&mut self, mv: libc::size_t) -> io::Result<libc::size_t>
fn error_move()
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

  // 20 times Goto::Down (Going outside down)
  assert_eq!(display.write(b"\x1B[20B").ok(), Some(0usize));

  // Print "hey"
  assert_eq!(display.write(b"hey").ok(), Some(3usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b'h', b'e', b'y', b' ', b' ' ]);

  // 20 times Goto::Left (Going outside left)
  assert_eq!(display.write(b"\x1B[20D").ok(), Some(0usize));

  // Print "yolo"
  assert_eq!(display.write(b"yolo").ok(), Some(4usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'y', b'o', b'l', b'o', b' ', b'h', b'e', b'y', b' ', b' ' ]);

  // 20 times Goto::Up (Going outside up)
  assert_eq!(display.write(b"\x1B[20A").ok(), Some(0usize));

  // Print "Meow"
  assert_eq!(display.write(b"Meow").ok(), Some(4usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'M', b'e', b'o', b'w', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'y', b'o', b'l', b'o', b' ', b'h', b'e', b'y', b' ', b' ' ]);

  // 20 times Goto::Right (Going outside right)
  assert_eq!(display.write(b"\x1B[20C").ok(), Some(0usize));

  // Print "Salut"
  assert_eq!(display.write(b"Salut").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'M', b'e', b'o', b'w', b' ', b'S',
           b'a', b'l', b'u', b't', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'y', b'o', b'l', b'o', b' ', b'h', b'e', b'y', b' ', b' ' ]); }

#[test]
/// fn goto_coord(&mut self, x: libc::size_t, y: libc::size_t)
fn error_move_coord()
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

  // Goto::(50, 4) (Going outside on x coordinates)
  assert_eq!(display.write(b"\x1B[5;51H").ok(), Some(0usize));

  // Print "coucou"
  assert_eq!(display.write(b"coucou").ok(), Some(6usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'c',
           b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::(2, 50) (Going outside on y coordinates)
  assert_eq!(display.write(b"\x1B[51;3H").ok(), Some(0usize));

  // Print "Hey"
  assert_eq!(display.write(b"Hey").ok(), Some(3usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'c',
           b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b'H', b'e', b'y', b' ', b' ', b' ', b' ', b' ' ]);

  // Goto::(50, 50) (Going outside on both coordinates)
  assert_eq!(display.write(b"\x1B[51;51H").ok(), Some(0usize));

  // Print "Medusa"
  assert_eq!(display.write(b"Medusa").ok(), Some(6usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'c',
           b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b'H', b'e', b'y', b' ', b' ', b' ', b' ', b'M',
           b'e', b'd', b'u', b's', b'a', b' ', b' ', b' ', b' ', b' ' ]); }


#[test]
/// fn save_position(&mut self)
/// fn restore_position(&mut self)
fn error_position_save()
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

  // Print "bonjour"
  assert_eq!(display.write(b"coucou").ok(), Some(6usize));
  assert_eq!(display.into_bytes(),
      vec![b'c', b'o', b'u', b'c', b'o', b'u', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Position Restore (without saving any position)
  assert_eq!(display.write(b"\x1B[u").ok(), Some(0usize));

  // Print "bonjour"
  assert_eq!(display.write(b"bonjour").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Position Save
  assert_eq!(display.write(b"\x1B[s").ok(), Some(0usize));

  let winsz: Winszed = Winszed
  { ws_row: 8,
    ws_col: 5,
    ws_xpixel: 0,
    ws_ypixel: 0, };

  // Remove 5 columns to the display
  display.resize_with(&winsz);
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ' ]);

  // Position Restore (at (7, 0), outside the screen)
  assert_eq!(display.write(b"\x1B[u").ok(), Some(0usize));

  // Print "Hello"
  assert_eq!(display.write(b"Hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'H',
           b'e', b'l', b'l', b'o', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ' ]);
}
