extern crate pty_shell_mode;

use pty_shell_mode::prelude as shell;

use std::io::{self, Write};

fn main() {
  let mut shell: shell::Shell = shell::Shell::new(None).unwrap();

  while let Some(event) = shell.next() {
    if let Some(ref o) = event.is_out_text() {
      io::stdout().write(o.as_slice()).unwrap();
      io::stdout().flush().unwrap();
    }
    if let Some(_) = event.is_out_screen() {
      if let Some(ref sig) = event.is_signal() {
        println!("{}", sig);
      }
    }
    if let Some(i) = event.is_in_text()  {
      shell.write(i).unwrap();
      shell.flush().unwrap();
    }
/*    if let Some(l) = event.is_line() {
    }*/
  }
  println!("bye bye");
}
