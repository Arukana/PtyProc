pub mod lr; // synthesized by LALRPOP
pub mod outlr;
pub mod util;
pub mod outp;
extern crate libc;
use std::io::prelude::*;
use std::io;

fn main() {

    let mut input_line = String::new();

        print!("\x1b[?1002h\x1b[?1015h\x1b[?1006h");
        let _ = io::stdout().flush(); // Make sure the '>' prints

    loop {
        print!("> ");
        let _ = io::stdout().flush(); // Make sure the '>' prints

        // Read in a string from stdin
        io::stdin().read_line(&mut input_line).ok().expect("The read line failed O:");
        
        // If 'exit' break out of the loop.
        match input_line.trim() {
            "exit" => break, //This could interfere with some parsers, so be careful
            line => {
              
              match lr::parse_MouseUse(&line.to_string())
              { Ok(v) => println!("{:?}", v),
                Err(_) => { match lr::parse_KeysUse(&line.to_string())
                { Ok(u) => println!("{:?}", u),
                  Err(_) => { match outlr::parse_CursorUse(&line.to_string())
                  { Ok(t) => println!("{:?}", t),
                    Err(_) => println!("Unimplemented :'("), }}}}}
                
            }
        }
        
        // Clear the input line so we get fresh input
        input_line.clear();
    }

    print!("\x1b[?1006l\x1b[?1015l\x1b[?1002l");
        let _ = io::stdout().flush(); // Make sure the '>' prints

    println!("Smell ya later! ;D");
}
