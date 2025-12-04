use crate::interpreter::{Interpreter, Word};
use crate::types::Type;

pub fn register_string_words(interp: &mut Interpreter) {
    // String concatenation
    interp.dictionary.insert(
        "..".to_string(),
        Word::Native(|interp| {
            let tos = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };
            let nos = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };

            interp.stack.push(Type::String(format!("{}{}", nos, tos)));
            Ok(())
        }),
    );

    // String length
    interp.dictionary.insert(
        "len".to_string(),
        Word::Native(|interp| {
            let tos = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };

            interp.stack.push(Type::Int(tos.len() as i64));
            Ok(())
        }),
    );

    // Reverse string
    interp.dictionary.insert(
        "reverse".to_string(),
        Word::Native(|interp| {
            let tos = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };

            let reversed: String = tos.chars().rev().collect();
            interp.stack.push(Type::String(reversed));
            Ok(())
        }),
    );

    // Uppercase string
    interp.dictionary.insert(
        "upper".to_string(),
        Word::Native(|interp| {
            let tos = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };

            interp.stack.push(Type::String(tos.to_uppercase()));
            Ok(())
        }),
    );
    // Lowercase string
    interp.dictionary.insert(
        "lower".to_string(),
        Word::Native(|interp| {
            let tos = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };

            interp.stack.push(Type::String(tos.to_lowercase()));
            Ok(())
        }),
    );
}
