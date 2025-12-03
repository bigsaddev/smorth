use crate::tokenizer::tokenize;
use crate::value::Value;
use crate::words;
use std::collections::HashMap;

pub struct Interpreter {
    pub stack: Vec<Value>,
    pub dictionary: HashMap<String, fn(&mut Interpreter) -> Result<(), String>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Interpreter {
            stack: Vec::new(),
            dictionary: HashMap::new(),
        };

        words::register_math_words(&mut interp);
        words::register_stack_words(&mut interp);
        words::register_comparison_words(&mut interp);
        words::register_logic_words(&mut interp);
        words::register_io_words(&mut interp);

        interp
    }

    // Helper functions
    pub fn pop_number(&mut self) -> Result<(f64, bool), String> {
        match self.stack.pop() {
            Some(Value::Int(n)) => Ok((n as f64, true)), // true = was int
            Some(Value::Float(f)) => Ok((f, false)),     // false = was float
            Some(other) => Err(format!("Expected number, got {:?}", other)),
            None => Err("Stack is empty!".to_string()),
        }
    }

    // Behaviour
    // 3 + 3 = 6
    // 3 + 3.0 = 6
    // 3 + 3.1 = 6.1
    // This checks for floats or ints and is used for mixed operations
    pub fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: Fn(f64, f64) -> f64,
    {
        // tos : Top of Stack
        // nos : Next on Stack
        let (tos, tos_is_int) = self.pop_number()?;
        let (nos, second_is_int) = self.pop_number()?;

        if second_is_int && tos_is_int {
            self.stack.push(Value::Int((op(nos, tos)) as i64));
        } else {
            self.stack.push(Value::Float(op(nos, tos)));
        }
        Ok(())
    }

    // Evaluate the tokens
    pub fn eval(&mut self, input: &str) -> Result<(), String> {
        let tokens = tokenize(input);

        for token in tokens {
            // DEBUG: println!("DEBUG: Processing token '{}'", token);

            // Check for string literal
            if token.starts_with("STR:") {
                let s = &token[4..];
                self.stack.push(Value::String(s.to_string()));
            }
            // Floats
            else if token.contains('.') && token.parse::<f64>().is_ok() {
                let f = token.parse::<f64>().unwrap();
                self.stack.push(Value::Float(f))
            }
            // Integers
            else if let Ok(n) = token.parse::<i64>() {
                self.stack.push(Value::Int(n))
            }
            // Dictionary lookup
            else if let Some(&func) = self.dictionary.get(&token) {
                func(self)?;
            } else {
                return Err(format!("Unknown token: {}", token));
            }
        }
        Ok(())
    }

    // Show what's on the stack
    pub fn show_stack(&self) {
        print!("Stack: [");
        for (i, val) in self.stack.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            match val {
                Value::Int(n) => print!("{}", n),
                Value::String(s) => print!("\"{}\"", s),
                Value::Float(f) => print!("{}", f),
                Value::Bool(b) => print!("{}", b),
            }
        }
        println!("]");
    }
}
