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

    // Swaps the top of the stack to the second on the stack and vice versa
    interp.dictionary.insert("swap".to_string(), |interp| {
        let tos = interp.stack.pop().ok_or("Stack underflow!")?;
        let nos = interp.stack.pop().ok_or("Stack underflow!")?;
        interp.stack.push(tos);
        interp.stack.push(nos);
        Ok(())
    });

    // Pops the top value from the stack permanently
    interp.dictionary.insert("drop".to_string(), |interp| {
        interp.stack.pop().ok_or("Stack underflow!")?;
        Ok(())
    });
}
