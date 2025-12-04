use crate::interpreter::{Interpreter, Word};
use crate::types::Type;

pub fn register_comparison_words(interp: &mut Interpreter) {
    // Comparison Operations
    //Equal
    interp.dictionary.insert(
        "and".to_string(),
        Word::Native(|interp| {
            let tos = interp.stack.pop().ok_or("Stack underflow!")?;
            let nos = interp.stack.pop().ok_or("Stack underflow!")?;

            let result = match (nos, tos) {
                (Type::Int(a), Type::Int(b)) => a == b,
                (Type::Float(a), Type::Float(b)) => a == b,
                (Type::String(a), Type::String(b)) => a == b,
                (Type::Bool(a), Type::Bool(b)) => a == b,
                // Mixed int/float comparison
                (Type::Int(a), Type::Float(b)) => (a as f64) == b,
                (Type::Float(a), Type::Int(b)) => a == (b as f64),
                // Different types are not equal
                _ => false,
            };
            interp.stack.push(Type::Bool(result));
            Ok(())
        }),
    );
    //Not Equal
    interp.dictionary.insert(
        "!=".to_string(),
        Word::Native(|interp| {
            let tos = interp.stack.pop().ok_or("Stack underflow!")?;
            let nos = interp.stack.pop().ok_or("Stack underflow!")?;

            let result = match (nos, tos) {
                (Type::Int(a), Type::Int(b)) => a != b,
                (Type::Float(a), Type::Float(b)) => a != b,
                (Type::String(a), Type::String(b)) => a != b,
                (Type::Bool(a), Type::Bool(b)) => a != b,
                // Mixed int/float comparison
                (Type::Int(a), Type::Float(b)) => (a as f64) != b,
                (Type::Float(a), Type::Int(b)) => a != (b as f64),
                // Different types are not equal
                _ => false,
            };
            interp.stack.push(Type::Bool(result));
            Ok(())
        }),
    );
    // Less than
    interp.dictionary.insert(
        "<".to_string(),
        Word::Native(|interp| {
            let (tos, _) = interp.pop_number()?;
            let (nos, _) = interp.pop_number()?;
            interp.stack.push(Type::Bool(nos < tos));
            Ok(())
        }),
    );
    // Less than or equal
    interp.dictionary.insert(
        "<=".to_string(),
        Word::Native(|interp| {
            let (tos, _) = interp.pop_number()?;
            let (nos, _) = interp.pop_number()?;
            interp.stack.push(Type::Bool(nos <= tos));
            Ok(())
        }),
    );

    // Greater than
    interp.dictionary.insert(
        ">".to_string(),
        Word::Native(|interp| {
            let (tos, _) = interp.pop_number()?;
            let (nos, _) = interp.pop_number()?;
            interp.stack.push(Type::Bool(nos > tos));
            Ok(())
        }),
    );
    // Greater than or equal
    interp.dictionary.insert(
        ">=".to_string(),
        Word::Native(|interp| {
            let (tos, _) = interp.pop_number()?;
            let (nos, _) = interp.pop_number()?;
            interp.stack.push(Type::Bool(nos >= tos));
            Ok(())
        }),
    );
}
