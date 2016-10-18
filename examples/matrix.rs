extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;

use std::io::{self, Write};

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let mut shell: shell::Shell = shell::Shell::from_mode(None, shell::Mode::Character).unwrap();

//    println!("Welcome {}-{}", shell.get_pid(), unsafe { libc::getpid() });
    while let Some(event) = shell.next() {
        if let Some(screen) = event.is_screen() {
            print!("{}", screen);
        }
    }
}
