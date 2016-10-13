extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;

use std::io::{self, Write};

fn main() {
  let mut shell: shell::Shell = shell::Shell::from_mode(None, shell::Mode::Character).unwrap();

  println!("Welcome {}-{}", shell.get_pid(), unsafe { libc::getpid() } );
  while let Some(event) = shell.next() {
    if let Some(ref o) = event.is_out_text() {
      io::stdout().write(o.as_slice()).unwrap();
      io::stdout().flush().unwrap();
    }
    if let Some(ref s) = event.is_signal() {
      println!("{}", s);
    }
  }
}
