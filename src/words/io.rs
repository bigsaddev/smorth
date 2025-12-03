use crate::interpreter::Interpreter;
use crate::value::Value;

pub fn register_io_words(interp: &mut Interpreter) {
    // Output and Consume the top-most value from the stack
    interp
        .dictionary
        .insert(".".to_string(), |interp| match interp.stack.pop() {
            Some(Value::Int(n)) => {
                println!("{}", n);
                Ok(())
            }
            Some(Value::String(s)) => {
                println!("{}", s);
                Ok(())
            }
            Some(Value::Float(f)) => {
                println!("{}", f);
                Ok(())
            }
            Some(Value::Bool(b)) => {
                println!("{}", b);
                Ok(())
            }
            None => Err("Stack is empty!".to_string()),
        });
}
