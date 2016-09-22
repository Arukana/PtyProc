pub mod lr; // synthesized by LALRPOP
pub mod util;
use std::io::prelude::*;
use std::io;

fn main() {

    let mut input_line = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush(); // Make sure the '>' prints

        // Read in a string from stdin
        io::stdin().read_line(&mut input_line).ok().expect("The read line failed O:");
        
        // If 'exit' break out of the loop.
        match input_line.trim() {
            "exit" => break, //This could interfere with some parsers, so be careful
            line => {
                 println!("{:?}", lr::parse_Term(&line.to_string()));
                
            }
        }
        
        // Clear the input line so we get fresh input
        input_line.clear();
    }

    println!("Smell ya later! ;D");
}
