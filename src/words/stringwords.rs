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

    // String interpolation
    interp.dictionary.insert(
        "format".to_string(),
        Word::Native(|interp| {
            // Pop the format string
            let fmt = match interp.stack.pop() {
                Some(Type::String(s)) => s,
                Some(_) => return Err("Expected format string".to_string()),
                None => return Err("Stack underflow!".to_string()),
            };

            // Count placeholders
            let placeholder_count = fmt.matches("$").count();

            // Pop values (in reverse since stack is LIFO)
            let mut values = Vec::new();
            for _ in 0..placeholder_count {
                let val = interp
                    .stack
                    .pop()
                    .ok_or("Not enough values for format string")?;
                values.push(val);
            }
            values.reverse(); // Reverse to get correct order

            // Build the result string
            let mut result = fmt.clone();
            for val in values {
                let val_str = match val {
                    Type::Int(n) => n.to_string(),
                    Type::Float(f) => f.to_string(),
                    Type::String(s) => s,
                    Type::Bool(b) => b.to_string(),
                };
                result = result.replacen("$", &val_str, 1);
            }

            interp.stack.push(Type::String(result));
            Ok(())
        }),
    );
}
