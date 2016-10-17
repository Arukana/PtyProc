extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;

use std::io::{self, Write};

fn main() {
    let mut display: Display = Display::new(0).unwrap();
    display.clear();
    print!("{}{}", display, "\x1B[1;1H");

    let mut shell: shell::Shell = shell::Shell::from_mode(None, shell::Mode::Character).unwrap();

    println!("Welcome {}-{}", shell.get_pid(), unsafe { libc::getpid() });
    while let Some(event) = shell.next() {
        if let Some(ref o) = event.is_out_text() {
            io::stdout().write(o.as_slice()).unwrap();
            match std::str::from_utf8(o) {
                Ok(line) => {
                    display.write(line.to_string().as_bytes());
                }
                Err(_) => {}
            }
            io::stdout().flush().unwrap();
        }
        if let Some(screen) = event.is_screen() {
            println!("{}", screen);
        }
    }
}
