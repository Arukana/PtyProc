extern crate pty_proc;
extern crate libc;

use std::io::Write;

use self::pty_proc::prelude::*;

const SIZE: Winszed = Winszed
{ ws_row: 2,
  ws_col: 3,
  ws_xpixel: 0,
  ws_ypixel: 0, };

/*
hel
lo.
*/

#[test]
fn test_new()
{ assert!(Display::new(libc::STDOUT_FILENO).is_ok()); }

#[test]
fn test_hello()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(&[b'a']).ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'a', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'a', b'h', b'e', b'l', b'l', b'o']); }

#[test]
fn test_home()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.write(b"old").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'o', b'l', b'd', b' ', b' ', b' ']);
  assert_eq!(display.write(b"\x1B[;Hnew").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'n', b'e', b'w', b' ', b' ', b' ']); }

#[test]
fn test_move()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.write(b"old").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'o', b'l', b'd', b' ', b' ', b' ']);
  assert_eq!(display.write(b"\x1B[Dnew").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'o', b'l', b'n', b'e', b'w', b' ']);
  assert_eq!(display.write(b"\x1B[Aj\x1B[Ak").ok(), Some(2usize));
  assert_eq!(display.into_bytes(), vec![b'k', b'l', b'j', b'e', b'w', b' ']);
  assert_eq!(display.write(b"\x1B[CZ").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'k', b'l', b'Z', b'e', b'w', b' ']);
  assert_eq!(display.write(b"\x1B[D\x1B[BX").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'k', b'l', b'Z', b'e', b'w', b'X']);
  assert_eq!(display.write(b"\x1B[D\x1B[A\x1B[Dhello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'k', b'h', b'e', b'l', b'l', b'o']); }

#[test]
fn test_enter()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hi\na").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'i', b' ', b' ', b' ', b'a']);
  assert_eq!(display.write(b"\rQJ").ok(), Some(2usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'i', b' ', b'Q', b'J', b'a']);
  assert_eq!(display.write(b"\x1B[;HK\n\rH").ok(), Some(2usize));
  assert_eq!(display.into_bytes(), vec![b'K', b'i', b' ', b'H', b'J', b'a']);
  assert_eq!(display.write(b"\nb").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'H', b'J', b'a', b' ', b'b', b' ']);
  assert_eq!(display.write(b"\rCLG").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'H', b'J', b'a', b'C', b'L', b'G']); }

#[test]
fn test_clear()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[2J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']); }

#[test]
fn test_erase_left_right()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[A\x1B[K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b' ', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[C\x1B[1K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b' ', b' ', b'o', b' ']); }

#[test]
fn test_erase_line()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[A\x1B[D\x1B[2K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[B\x1B[2K").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']); }

#[test]
fn test_erase_up_down()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[D\x1B[D\x1B[1J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[;Hhey").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'y', b' ', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[D\x1B[D\x1B[J").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b' ', b' ', b' ', b' ', b' ']); }

#[test]
fn test_insert_empty_line()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[A\x1B[D\x1B[L").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b' ', b' ', b' ', b'e', b'l']); }

#[test]
fn test_goto()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[2;1Hx").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'x', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[1;3Hu").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'u', b'x', b'o', b' ']); }

#[test]
fn test_n_move()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.write(b"old").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'o', b'l', b'd', b' ', b' ', b' ']);
  assert_eq!(display.write(b"\x1B[2Dnew").ok(), Some(3usize));
  assert_eq!(display.into_bytes(), vec![b'o', b'n', b'e', b'w', b' ', b' ']);
  assert_eq!(display.write(b"\x1B[1A\x1B[3C\x1B[1Ahello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'o', b'h', b'e', b'l', b'l', b'o']); }

#[test]
fn test_position_save()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"h\x1B[sello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[uQ").ok(), Some(1usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'Q', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[;Hh\x1B[sel\x1B[slo").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[uJ\x1B[uK").ok(), Some(2usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'K', b'o', b' ']); }

#[test]
fn test_scroll()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1BD").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b'h', b'e', b'l']);
  assert_eq!(display.write(b"\x1B[;Hhi").ok(), Some(2usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'i', b' ', b'h', b'e', b'l']);
  assert_eq!(display.write(b"\x1BM").ok(), Some(0usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b' ', b' ', b' ']); }

#[test]
fn test_iter()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.write(b"L\xE9opar").ok(), Some(6usize));
  let mut iterator = display.into_iter();
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'L'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'\xE9'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'o'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'p'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'a'][..]);
  assert_eq!(iterator.next().unwrap_or_default().get_ref(), &[b'r'][..]);
  assert!(iterator.next().is_none()); }

#[test]
fn test_save_terminal()
{ let mut display: Display = Display::from_winszed(SIZE);
  assert_eq!(display.into_bytes(), vec![b' ', b' ', b' ', b' ', b' ', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"\x1B[1049h").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']);
  assert_eq!(display.write(b"hello").ok(), Some(5usize));
  assert_eq!(display.into_bytes(), vec![b'h', b'e', b'l', b'l', b'o', b' ']); }
