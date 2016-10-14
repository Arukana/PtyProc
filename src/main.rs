extern crate pty_proc;
extern crate libc;

use pty_proc::prelude as shell;
use pty_proc::prelude::*;

use std::io::{self, Write};

fn main()
{ 
  let mut display: Display = Display::new(0).unwrap();
  display.clear();
  print!("{}{}", display, "\x1B[1;1H");
  let mut shell: shell::Shell = shell::Shell::from_mode(None, shell::Mode::Character).unwrap();
//  println!("Welcome {}-{}", shell.get_pid(), unsafe { libc::getpid() } );
  while let Some(event) = shell.next()
  { if let Some(ref o) = event.is_in_text()
    { //println!("POS::({}, {})", display.get_current_position(), *display.get_position());
      match std::str::from_utf8(o)
      { Ok(line) =>
          {/* for i in line.to_string().chars()
            { print!("I({} | {})", i, i as u32); }*/},
        Err(_) => {}, }}
    if let Some(ref o) = event.is_out_text()
    { io::stdout().write(o.as_slice()).unwrap();
      match std::str::from_utf8(o)
      { Ok(line) =>
          { /*for i in line.to_string().chars()
            { print!("O({} | {})", i, i as u32); }*/
            display.write(line.to_string().as_bytes()); },
        Err(_) => {}, }
      io::stdout().flush().unwrap(); }
    if let Some(ref s) = event.is_signal()
    { println!("{}", s); }}}
