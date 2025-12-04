use crate::interpreter::{Interpreter, Word};

pub fn register_stack_words(interp: &mut Interpreter) {
    // Stack manipulation
    interp.dictionary.insert(
        "dup".to_string(),
        Word::Native(|interp| match interp.stack.pop() {
            Some(val) => {
                interp.stack.push(val.clone());
                interp.stack.push(val);
                Ok(())
            }
            None => Err("Stack is empty!".to_string()),
        }),
    );

    // Swaps the top of the stack to the second on the stack and vice versa
    interp.dictionary.insert(
        "swap".to_string(),
        Word::Native(|interp| {
            let tos = interp.stack.pop().ok_or("Stack underflow!")?;
            let nos = interp.stack.pop().ok_or("Stack underflow!")?;
            interp.stack.push(tos);
            interp.stack.push(nos);
            Ok(())
        }),
    );

    // Pops the top value from the stack permanently
    interp.dictionary.insert(
        "drop".to_string(),
        Word::Native(|interp| {
            interp.stack.pop().ok_or("Stack underflow!")?;
            Ok(())
        }),
    );
}
