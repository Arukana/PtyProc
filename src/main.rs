extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;

use std::io::Write;
use std::str;

fn main() {
    print!("\x1B[?25l\x1B[H\x1B[2J");

    let mut shell: shell::Shell = shell::Shell::new(
        None,
        None,
        None,
    ).unwrap();
    while let Some(event) = shell.next() {
        if let Some(ref text) = event.is_input_slice() {
            shell.write(text).unwrap();
            shell.flush().unwrap();
        }
        if let Some(()) = event.is_output_screen() {
            print!("\x1B[H{}", shell);
            //print!("{}", shell);
        }
    }
}
