mod interpreter;
mod tokenizer;
mod value;
mod words;

use interpreter::Interpreter;
use std::io::{self, Write};

// Entry Point
fn main() {
    let mut interp = Interpreter::new();

    println!("Smorth | Stack Language");
    println!("Type 'bye' to exit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "bye" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        match interp.eval(input) {
            Ok(_) => interp.show_stack(),
            Err(e) => println!("Error: {}", e),
        }
    }
}
