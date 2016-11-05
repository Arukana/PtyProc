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


/*
#[test]
fn test_scroll()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1BD").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b'h', b'e', b'l']);
  assert_eq!(display.write(b"\x1B[;Hhi\x1B[B\x1B[D").ok(), Some(2usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'i', b' ', b'h', b'e', b'l']);
  assert_eq!(display.write(b"\x1BMa").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b' ', b'a', b' ']);
  assert_eq!(display.write(b"\x1BM").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b'a', b' ', b' ', b' ', b' ']); }


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


#[test]
fn test_save_terminal()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[?1049h").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[2J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"\x1B[?1049l").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']); }
*/
