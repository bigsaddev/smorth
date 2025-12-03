use crate::interpreter::Interpreter;
use crate::value::Value;

pub fn register_logic_words(interp: &mut Interpreter) {
    interp.dictionary.insert("and".to_string(), |interp| {
        let b = match interp.stack.pop() {
            Some(Value::Bool(b)) => b,
            _ => return Err("Expected bool".to_string()),
        };
        let a = match interp.stack.pop() {
            Some(Value::Bool(b)) => b,
            _ => return Err("Expected bool".to_string()),
        };
        interp.stack.push(Value::Bool(a && b));
        Ok(())
    });

    interp.dictionary.insert("or".to_string(), |interp| {
        let b = match interp.stack.pop() {
            Some(Value::Bool(b)) => b,
            _ => return Err("Expected bool".to_string()),
        };
        let a = match interp.stack.pop() {
            Some(Value::Bool(b)) => b,
            _ => return Err("Expected bool".to_string()),
        };
        interp.stack.push(Value::Bool(a || b));
        Ok(())
    });

    interp.dictionary.insert("not".to_string(), |interp| {
        let a = match interp.stack.pop() {
            Some(Value::Bool(b)) => b,
            _ => return Err("Expected bool".to_string()),
        };
        interp.stack.push(Value::Bool(!a));
        Ok(())
    });
}
