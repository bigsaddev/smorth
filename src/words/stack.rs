use crate::interpreter::Interpreter;

pub fn register_stack_words(interp: &mut Interpreter) {
    // Stack manipulation
    interp.dictionary.insert("dup".to_string(), |interp| {
        match interp.stack.pop() {
            Some(val) => {
                interp.stack.push(val.clone()); // Push first copy
                interp.stack.push(val); // Push second copy
                Ok(())
            }
            None => Err("Stack is empty!".to_string()),
        }
    });
    interp.dictionary.insert("swap".to_string(), |interp| {
        let tos = interp.stack.pop().ok_or("Stack underflow!")?;
        let nos = interp.stack.pop().ok_or("Stack underflow!")?;
        interp.stack.push(tos);
        interp.stack.push(nos);
        Ok(())
    });
    interp.dictionary.insert("drop".to_string(), |interp| {
        interp.stack.pop().ok_or("Stack underflow!")?;
        Ok(())
    });
}
