mod interpreter;
mod tokenizer;
mod types;
mod words;

use interpreter::Interpreter;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut interp = Interpreter::new();

    // If a file is provided, run it
    if args.len() > 1 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(contents) => {
                if let Err(e) = interp.eval(&contents) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                interp.show_stack();
            }
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
        return;
    }
    repl(&mut interp);
}

fn repl(interp: &mut Interpreter) {
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
