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
fn test_hello()
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
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]); }


#[test]
/// fn print_enter(&mut self)
/// fn add_newline(&mut self)
/// fn check_newline(&mut self)
fn test_enter()
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
  //       "
  assert_eq!(display.write(b"hello\n\r").ok(), Some(5usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.newlines(), &vec![(5, 0), (9, 1), (9, 2), (9, 3),
                                       (9, 4), (9, 5), (9, 6), (9, 7)]);

  // Print "bonjour
  //       "
  assert_eq!(display.write(b"bonjour\n").ok(), Some(7usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.newlines(), &vec![(5, 0), (7, 1), (9, 2), (9, 3),
                                       (9, 4), (9, 5), (9, 6), (9, 7)]);

  // Print "hello lorem ipsum
  //       "
  assert_eq!(display.write(b"hello lorem ipsum\n\r").ok(), Some(17usize));
  assert_eq!(display.into_bytes(),
      vec![b'h', b'e', b'l', b'l', b'o', b' ', b' ', b' ', b' ', b' ',
           b'b', b'o', b'n', b'j', b'o', b'u', b'r', b' ', b' ', b' ',
           b'h', b'e', b'l', b'l', b'o', b' ', b'l', b'o', b'r', b'e',
           b'm', b' ', b'i', b'p', b's', b'u', b'm', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
           b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ' ]);
  assert_eq!(display.newlines(), &vec![(5, 0), (7, 1), (7, 3), (9, 4), 
                                       (9, 5), (9, 6), (9, 7)]);
}
