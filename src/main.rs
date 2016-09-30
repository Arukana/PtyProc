extern crate pty_shell_mode;
extern crate libc;
extern crate example;

use example::{lr, outlr, outp, util};

use pty_shell_mode::prelude as shell;

use std::io::{self, Write};

fn main() {
  let mut shell: shell::Shell = shell::Shell::from_mode(None, shell::Mode::Character).unwrap();

  let mut p = 0;

  println!("Welcome {}-{}", shell.get_pid(), unsafe { libc::getpid() } );
  while let Some(event) = shell.next() {
    if let Some(ref o) = event.is_in_text()
    {/* let _ = match std::str::from_utf8(o)
      { Ok(line) => match lr::parse_KeysUse(&line.to_string())
        { Ok(v) => { match v
          { util::Key::Left => print!("{:?}", v),
            util::Key::Backspace => print!("{:?}", v),
            _ => {}, }; },
          Err(_) => {}, },
        Err(_) => {}, };
    */}

    if let Some(ref o) = event.is_out_text() {
      io::stdout().write(o.as_slice()).unwrap();
      io::stdout().flush().unwrap();
    }

    if let Some(ref k) = event.is_out_screen()
    { /*let s = k.as_slice();
      let _ = match std::str::from_utf8(k)
      { Ok(line) => match lr::parse_KeysUse(&line.to_string())
        { Ok(v) => { match v
          { util::Key::Left => print!("{:?}", v),
            util::Key::Backspace => print!("{:?}", v),
            _ => {}, }; },
          Err(_) => {}, },
        Err(_) => {}, };*/
     // print!("\n\rp::{}", p);
      p += 1;
      print!("{}", k);
    }

    if let Some(ref s) = event.is_signal() {
      println!("{}", s);
    }
  }
}
