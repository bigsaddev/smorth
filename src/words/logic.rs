use crate::interpreter::{Interpreter, Word};
use crate::types::Type;

pub fn register_logic_words(interp: &mut Interpreter) {
    // Logical and
    interp.dictionary.insert(
        "and".to_string(),
        Word::Native(|interp| {
            let b = match interp.stack.pop() {
                Some(Type::Bool(b)) => b,
                _ => return Err("Expected bool".to_string()),
            };
            let a = match interp.stack.pop() {
                Some(Type::Bool(b)) => b,
                _ => return Err("Expected bool".to_string()),
            };
            interp.stack.push(Type::Bool(a && b));
            Ok(())
        }),
    );

    // Logical or
    interp.dictionary.insert(
        "or".to_string(),
        Word::Native(|interp| {
            let b = match interp.stack.pop() {
                Some(Type::Bool(b)) => b,
                _ => return Err("Expected bool".to_string()),
            };
            let a = match interp.stack.pop() {
                Some(Type::Bool(b)) => b,
                _ => return Err("Expected bool".to_string()),
            };
            interp.stack.push(Type::Bool(a || b));
            Ok(())
        }),
    );

    // Logical not
    interp.dictionary.insert(
        "not".to_string(),
        Word::Native(|interp| {
            let a = match interp.stack.pop() {
                Some(Type::Bool(b)) => b,
                _ => return Err("Expected bool".to_string()),
            };
            interp.stack.push(Type::Bool(!a));
            Ok(())
        }),
    );
}
