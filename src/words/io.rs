use crate::interpreter::{Interpreter, Word};
use crate::types::Type;

pub fn register_io_words(interp: &mut Interpreter) {
    // Output and Consume the top-most value from the stack
    interp.dictionary.insert(
        ".".to_string(),
        Word::Native(|interp| match interp.stack.pop() {
            Some(Type::Int(n)) => {
                println!("{}", n);
                Ok(())
            }
            Some(Type::String(s)) => {
                println!("{}", s);
                Ok(())
            }
            Some(Type::Float(f)) => {
                println!("{}", f);
                Ok(())
            }
            Some(Type::Bool(b)) => {
                println!("{}", b);
                Ok(())
            }
            None => Err("Stack is empty!".to_string()),
        }),
    );
}
