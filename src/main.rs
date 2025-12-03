use std::collections::hash_map::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

struct Interpreter {
    stack: Vec<Value>,
    dictionary: HashMap<String, fn(&mut Interpreter) -> Result<(), String>>,
}

impl Interpreter {
    fn new() -> Self {
        let mut interp = Interpreter {
            stack: Vec::new(),
            dictionary: HashMap::new(),
        };

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

        // Output and Consume the top-most value from the stack
        interp
            .dictionary
            .insert(".".to_string(), |interp| match interp.stack.pop() {
                Some(Value::Int(n)) => {
                    println!("{}", n);
                    Ok(())
                }
                Some(Value::String(s)) => {
                    println!("{}", s);
                    Ok(())
                }
                Some(Value::Float(f)) => {
                    println!("{}", f);
                    Ok(())
                }
                Some(Value::Bool(b)) => {
                    println!("{}", b);
                    Ok(())
                }
                None => Err("Stack is empty!".to_string()),
            });

        interp
    }

    // Helper functions
    fn pop_number(&mut self) -> Result<(f64, bool), String> {
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
    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
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
    fn eval(&mut self, input: &str) -> Result<(), String> {
        let tokens = self.tokenize(input);

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
    fn show_stack(&self) {
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

    // Tokenizer
    fn tokenize(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_string = false;

        for ch in input.chars() {
            if ch == '"' {
                if in_string {
                    // End of string - mark it with STR: prefix
                    tokens.push(format!("STR:{}", current));
                    current.clear();
                }
                in_string = !in_string;
            } else if in_string {
                // Inside quotes, collect everything (including spaces)
                current.push(ch);
            } else if ch.is_whitespace() {
                // Outside quotes, whitespace separates tokens
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            } else {
                // Regular character
                current.push(ch);
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }
}

// Entry Point
fn main() {
    let mut interp = Interpreter::new();

    println!("Smorth | Stack Based Language Interpreter");
    println!("Type 'bye' to exit.");

    loop {
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "bye" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        match interp.eval(input) {
            Ok(_) => interp.show_stack(),
            Err(e) => println!("Error: {}", e),
        }
    }
}
