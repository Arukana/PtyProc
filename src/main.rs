extern crate pty_proc;
extern crate libc;

extern crate baum;
use baum::baum::{Baum};

use pty_proc::prelude as shell;

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

           // --- Envoie un signal à chaque input, ça fonctionne ---
           // unsafe
           // { let baum = Baum::new(libc::getpid());
           //   println!("BAUM::{}", baum.pid);
           // for x in baum.childs
           // { println!("PID::{}", x.pid);
           //   libc::kill(x.pid, libc::SIGWINCH); }}

            print!("{}", screen);
        }
        if let Some(ref s) = event.is_signal()
        { unsafe
          { let baum = Baum::new(libc::getpid());
            baum.childs.iter().map(|x| libc::kill(x.pid, *s)); }}
    }
}
