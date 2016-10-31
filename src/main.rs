extern crate pty_proc;
extern crate libc;

extern crate baum;
use baum::baum::{Baum};

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
        if let Some(screen) = event.is_output_screen() {
            print!("{}", screen);
        }
        if let Some(sig) = event.is_signal()
        { unsafe
          { let baum: Baum = Baum::new(libc::getpid());
            baum.childs.iter().map(|i|
            { libc::kill(i.pid, libc::SIGWINCH);
              true }); }}
    }
}
