extern crate pty_proc;
extern crate libc;

use self::pty_proc::prelude::*;

const KEY_REPEAT: libc::c_long = 1_000_000_000i64;

const A: In = [b'a',
               b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'
];

const B: In = [b'b',
               b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00'
];

#[test]
fn test_new() {
    let mut state: ShellState = ShellState::new(Some(self::KEY_REPEAT), libc::STDIN_FILENO);

    state.set_input(Some(Control::new(A, 1)));
    assert_eq!(state.is_input_keydown(), Some(Key::new(&A, 1)));
}
