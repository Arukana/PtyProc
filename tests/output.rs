extern crate pty_proc;
extern crate libc;

use std::io::Write;

use self::pty_proc::prelude::*;

#[test]
fn test_new() {
  assert!(Display::new(libc::STDOUT_FILENO).is_ok());
}

#[test]
fn test_hello() {
  let mut display: Display = Display::from_winszed(
      Winszed {
          ws_row: 2,
          ws_col: 3,
          ws_xpixel: 0,
          ws_ypixel: 0,
      },
  );

  assert_eq!(display.get_ref(), &vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.get_ref(), &vec![b'h', b'e', b'l', b'l', b'o', b' ']);
}

#[test]
fn test_goto() {
  let mut display: Display = Display::from_winszed(
      Winszed {
          ws_row: 2,
          ws_col: 3,
          ws_xpixel: 0,
          ws_ypixel: 0,
      },
  );

  assert_eq!(display.write(b"old").ok(), Some(3usize));
  assert_eq!(display.get_ref(), &vec![b'o', b'l', b'd', b' ', b' ', b' ']);
  assert_eq!(display.write(b"\x1B[;Hnew").ok(), Some(3usize));
  assert_eq!(display.get_ref(), &vec![b'n', b'e', b'w', b' ', b' ', b' ']);
}
