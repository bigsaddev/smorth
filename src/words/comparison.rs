use crate::interpreter::Interpreter;
use crate::value::Value;

pub fn register_comparison_words(interp: &mut Interpreter) {
    // Comparison Operations
    //Equal
    interp.dictionary.insert("==".to_string(), |interp| {
        let tos = interp.stack.pop().ok_or("Stack underflow!")?;
        let nos = interp.stack.pop().ok_or("Stack underflow!")?;

        let result = match (nos, tos) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            // Mixed int/float comparison
            (Value::Int(a), Value::Float(b)) => (a as f64) == b,
            (Value::Float(a), Value::Int(b)) => a == (b as f64),
            // Different types are not equal
            _ => false,
        };
        interp.stack.push(Value::Bool(result));
        Ok(())
    });
    //Not Equal
    interp.dictionary.insert("!=".to_string(), |interp| {
        let tos = interp.stack.pop().ok_or("Stack underflow!")?;
        let nos = interp.stack.pop().ok_or("Stack underflow!")?;

        let result = match (nos, tos) {
            (Value::Int(a), Value::Int(b)) => a != b,
            (Value::Float(a), Value::Float(b)) => a != b,
            (Value::String(a), Value::String(b)) => a != b,
            (Value::Bool(a), Value::Bool(b)) => a != b,
            // Mixed int/float comparison
            (Value::Int(a), Value::Float(b)) => (a as f64) != b,
            (Value::Float(a), Value::Int(b)) => a != (b as f64),
            // Different types are not equal
            _ => false,
        };
        interp.stack.push(Value::Bool(result));
        Ok(())
    });
    // Less than
    interp.dictionary.insert("<".to_string(), |interp| {
        let (tos, _) = interp.pop_number()?;
        let (nos, _) = interp.pop_number()?;
        interp.stack.push(Value::Bool(nos < tos));
        Ok(())
    });
    // Less than or equal
    interp.dictionary.insert("<=".to_string(), |interp| {
        let (tos, _) = interp.pop_number()?;
        let (nos, _) = interp.pop_number()?;
        interp.stack.push(Value::Bool(nos <= tos));
        Ok(())
    });
    // Greater than
    interp.dictionary.insert(">".to_string(), |interp| {
        let (tos, _) = interp.pop_number()?;
        let (nos, _) = interp.pop_number()?;
        interp.stack.push(Value::Bool(nos > tos));
        Ok(())
    });
    // Greater than or equal
    interp.dictionary.insert(">=".to_string(), |interp| {
        let (tos, _) = interp.pop_number()?;
        let (nos, _) = interp.pop_number()?;
        interp.stack.push(Value::Bool(nos >= tos));
        Ok(())
    });
}
