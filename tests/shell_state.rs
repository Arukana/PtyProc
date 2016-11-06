extern crate pty_proc;
extern crate libc;

use std::{thread, time};

use self::pty_proc::prelude::*;

const A: In = [b'a',
               b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'
];

const B: In = [b'b',
               b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'
];

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn test_key_down() {
    let mut state: ShellState = ShellState::new(None, None, libc::STDIN_FILENO);

    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&A, 1)));
    state.set_input(Some(Control::new(B, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&B, 1)));
    state.set_input(None);
    assert_eq!(state.is_input_keydown(), None);
}

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn test_key_repeat() {
    let mut state: ShellState = ShellState::new(Some(REPEAT), None, libc::STDIN_FILENO);

    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    state.set_input(None);
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(2));
    thread::sleep(time::Duration::from_millis(REPEAT as u64 -200));
    state.set_input(None);
    assert_eq!(state.is_input_keyrepeat(), Some(2));
    thread::sleep(time::Duration::from_millis(REPEAT as u64));
    state.set_input(None);
    assert_eq!(state.is_input_keyrepeat(), None);

    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(1));
    state.set_input(Some(Control::new(B, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(0));
}

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn test_key_interval() {
    let mut state: ShellState = ShellState::new(None, Some(INTERVAL), libc::STDIN_FILENO);
    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));

    assert_eq!(state.is_input_keyrepeat(), Some(2));
    assert_eq!(state.is_input_keyinterval(), Some(0));

    thread::sleep(time::Duration::from_millis(REPEAT as u64 -200));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(3));
    thread::sleep(time::Duration::from_millis(REPEAT as u64 -200));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(4));
    assert_eq!(state.is_input_keyinterval(), Some(1));

    thread::sleep(time::Duration::from_millis(REPEAT as u64 -1));
    state.set_input(Some(Control::new(A, 1)));
    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keyrepeat(), Some(6));
    assert_eq!(state.is_input_keyinterval(), Some(2));

    thread::sleep(time::Duration::from_millis(REPEAT as u64));
    state.set_input(None);
    assert_eq!(state.is_input_keyrepeat(), None);
    assert_eq!(state.is_input_keyinterval(), None);
}
