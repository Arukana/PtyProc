#![feature(range_contains)]

#[macro_use]
extern crate pty_proc;

#[test]
fn test_macros () {
  {
    let buf: [u8; 0] = [];

    assert_eq!(parse_number!(buf), None);
  }
  {
    let buf: [u8; 1] = [b'_'];

    assert_eq!(parse_number!(buf), None);
  }
  {
    let buf: [u8; 1] = [b'0'];

    assert_eq!(parse_number!(buf), Some((None, 0, (&[][..]))));
  }
  {
    let buf: [u8; 2] = [b'4', b'2'];

    assert_eq!(parse_number!(buf), Some((None, 42, (&[][..]))));
  }
  {
    let buf: [u8; 3] = [b'4', b'2', b';'];

    assert_eq!(parse_number!(buf), Some((Some(&b';'), 42, (&[b';'][..]))));
  }
  {
    let buf: [u8; 4] = [b'4', b'2', b';', b'0'];

    assert_eq!(parse_number!(buf), Some((Some(&b';'), 42, (&[b';', b'0'][..]))));
  }
}
