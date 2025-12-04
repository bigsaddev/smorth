use crate::interpreter::{Interpreter, Word};
use crate::types::Type;

pub fn register_math_words(interp: &mut Interpreter) {
    // Math Operations
    interp.dictionary.insert(
        "+".to_string(),
        Word::Native(|interp| interp.binary_op(|a, b| a + b)),
    );

    interp.dictionary.insert(
        "-".to_string(),
        Word::Native(|interp| interp.binary_op(|a, b| a - b)),
    );

    interp.dictionary.insert(
        "*".to_string(),
        Word::Native(|interp| interp.binary_op(|a, b| a * b)),
    );
    interp.dictionary.insert(
        "/".to_string(),
        Word::Native(|interp| interp.binary_op(|a, b| a / b)),
    );

    // Takes the value off the top of the stack and sqrts it
    interp.dictionary.insert(
        "sqrt".to_string(),
        Word::Native(|interp| {
            let (num, _) = interp.pop_number()?;
            interp.stack.push(Type::Float(num.sqrt()));
            Ok(())
        }),
    );
}
