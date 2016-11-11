extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;

use std::str;

fn main() {
    print!("\x1B[?25l\x1B[H\x1B[2J");

    let mut shell: shell::Shell = shell::Shell::from_mode(
        None,
        None,
        None,
        shell::Mode::Character
    ).unwrap();
    while let Some(event) = shell.next() {
        if let Some(screen) = event.is_output_screen() {
            print!("{}", screen);
        }
    }
}
