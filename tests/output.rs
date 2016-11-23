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
fn test_new()
{ assert!(Display::new(libc::STDOUT_FILENO).is_ok()); }


#[test]
/// fn scroll_down(&mut self)
/// fn scroll_up(&mut self)
fn test_scroll()
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
  assert_eq!(display.newlines(), &vec![(9, 0), (9, 1), (9, 2), (9, 3),
                                       (9, 4), (9, 5), (9, 6), (9, 7)]);

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
  assert_eq!(display.newlines(), &vec![(9, 7)]);

  // 1 time Scroll::Up
  assert_eq!(display.write(b"\x1BS").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm' , b'e', b't', b' ', b'b', b'o', b'n', b'j',
           b'o', b'u', b'r', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.newlines(), &vec![(9, 6), (9, 7)]);

  // 1 time Scroll::Down
  assert_eq!(display.write(b"\x1BT").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm', b'e', b't', b' ', b'b', b'o', b'n', b'j',
           b'o', b'u', b'r', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.newlines(), &vec![(9, 0), (9, 7)]);

  // 3 times Scroll::Down
  assert_eq!(display.write(b"\x1B3T").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm' ]);
  assert_eq!(display.newlines(), &vec![(9, 0), (9, 1), (9, 2), (9, 3), (9, 7)]);

  // 5 times Scroll::Up
  assert_eq!(display.write(b"\x1B5S").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.newlines(), &vec![(9, 2), (9, 3), (9, 4), (9, 5),
                                       (9, 6), (9, 7)]);

}

#[test]
#[ignore]
fn test_save_terminal()
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

  // Save Terminal
  assert_eq!(display.write(b"\x1B[?1049h").ok(), Some(0usize));

  // Clear Screen, then Goto::Home
  assert_eq!(display.write(b"\x1B[2J\x1B[H").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Print "hello world!"
  assert_eq!(display.write(b"hello world!").ok(), Some(12usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l',
           b'd', b'!', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Restore Terminal
  assert_eq!(display.write(b"\x1B[?1049l").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b'd', b'o',
           b'l', b'o', b'r', b' ', b's', b'i', b't', b' ', b'a', b'm',
           b'e', b't', b' ', b'h', b'e', b'l', b'l', b'o', b' ', b'l',
           b'o', b'r', b'e', b'm', b' ', b'i', b'p', b's', b'u', b'm',
           b' ', b'd', b'o', b'l', b'o', b'r', b' ', b's', b'i', b't',
           b' ', b'a', b'm', b'e', b't', b' ', b'b', b'o', b'n', b'j',
           b'o', b'u', b'r', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  // Clear Screen, then Goto::Home
  assert_eq!(display.write(b"\x1B[2J\x1B[H").ok(), Some(0usize));
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

  // Restore Terminal
  assert_eq!(display.write(b"\x1B[?1049l").ok(), Some(0usize));
  assert_eq!(display.into_bytes(),
      vec![b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }


/*
#[test]
fn test_iter()
{ let mut display: Display = Display::from_winszed(SIZE);

  // Print "Léopard"
  assert_eq!(display.write(b"L\xE9opard").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'L', b'é', b'o', b'p', b'a', b'r', b'd', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);

  let mut iterator = display.into_iter();
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'L'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'\xE9'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'o'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'p'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'a'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b' '][..]); }
*/
