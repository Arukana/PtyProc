extern crate pty_shell_mode;
extern crate libc;
extern crate example;

use example::{lr, outlr, outp, util};

use pty_shell_mode::prelude as shell;

use std::io::{self, Write};

fn pattern_matching(line: &[u8], from: &str)
{ match lr::parse_KeysUse(&line.to_string())
  { Ok(v) => print!("\n\r{}::{:?}\n\r", from, v),
    Err(_) => { match lr::parse_MouseUse(&line.to_string())
    { Ok(u) => print!("\n\r{}::{:?}\n\r", from, u), 
      Err(_) => { match outlr::parse_CursorUse(&line.to_string())
      { Ok(t) => print!("\n\r{}::{:?}\n\r", from, t),
        Err(_) => { for i in line.as_bytes()
        { if i == '\x1B'
          { print!(" ** "); }
          print!("Err{}::{}", from, i);
          print!(" !{}! ", from) }}, }}, }; }, }, }

fn main() {
  let mut shell: shell::Shell = shell::Shell::from_mode(None, shell::Mode::Character).unwrap();
  println!("Welcome {}-{}", shell.get_pid(), unsafe { libc::getpid() } );

  while let Some(event) = shell.next() {
    if let Some(ref o) = event.is_in_text()
    { let _ = match std::str::from_utf8(o)
      { Ok(line) => { pattern_matching(line, &"In"); },
        Err(_) => {}, }; }
    if let Some(ref o) = event.is_out_text()
    { io::stdout().write(o.as_slice()).unwrap();
      let _ = match std::str::from_utf8(o)
      { Ok(line) => { pattern_matching(line, &"Out") },
        Err(_) => {}, };
      io::stdout().flush().unwrap(); }
//    if let Some(ref k) = event.is_out_screen()
//    { ; }
//    if let Some(ref s) = event.is_signal()
//    { println!("{}", s); }
  }
}
