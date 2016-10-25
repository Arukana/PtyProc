extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;

use std::str;

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let mut shell: shell::Shell = shell::Shell::from_mode(
        None,
        None,
        None,
        shell::Mode::Character
    ).unwrap();
    while let Some(event) = shell.next() {
        if let Some(line) = event.is_output_last() {
            print!("{}", unsafe {
                    str::from_utf8_unchecked(&line)
            });
        }
    }
}
