use crate::interpreter::Interpreter;
use crate::value::Value;

pub fn register_math_words(interp: &mut Interpreter) {
    // Math Operations
    interp
        .dictionary
        .insert("+".to_string(), |interp| interp.binary_op(|a, b| a + b));
    interp
        .dictionary
        .insert("-".to_string(), |interp| interp.binary_op(|a, b| a - b));
    interp
        .dictionary
        .insert("*".to_string(), |interp| interp.binary_op(|a, b| a * b));
    interp.dictionary.insert("/".to_string(), |interp| {
        interp.binary_op(|a: f64, b| a / b)
    });

    interp.dictionary.insert("sqrt".to_string(), |interp| {
        let (num, _) = interp.pop_number()?;
        interp.stack.push(Value::Float(num.sqrt()));
        Ok(())
    });
}
