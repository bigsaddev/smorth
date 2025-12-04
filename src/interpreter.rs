use crate::tokenizer::tokenize;
use crate::types::Type;
use crate::words;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Word {
    Native(fn(&mut Interpreter) -> Result<(), String>),
    UserDefined(Vec<String>),
}

pub struct Interpreter {
    pub stack: Vec<Type>,
    pub dictionary: HashMap<String, Word>,
    pub variables: HashMap<String, Type>,

    compiling: bool,
    current_word_name: String,
    current_definition: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interp = Interpreter {
            stack: Vec::new(),
            dictionary: HashMap::new(),
            variables: HashMap::new(),
            compiling: false,
            current_word_name: String::new(),
            current_definition: Vec::new(),
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
            Some(Type::Int(n)) => Ok((n as f64, true)), // true = was int
            Some(Type::Float(f)) => Ok((f, false)),     // false = was float
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
            self.stack.push(Type::Int((op(nos, tos)) as i64));
        } else {
            self.stack.push(Type::Float(op(nos, tos)));
        }
        Ok(())
    }

    // Evaluate the tokens
    pub fn eval(&mut self, input: &str) -> Result<(), String> {
        let tokens = tokenize(input);

        for token in tokens {
            println!("DEBUG: Processing token '{}'", token);

            // Handle word definition start
            if token == ":" {
                if self.compiling {
                    return Err("Already defining a word".to_string());
                }
                self.compiling = true;
                self.current_definition.clear();
                self.current_word_name.clear();
                continue;
            }

            // Handle word definition end
            if token == ";" {
                if !self.compiling {
                    return Err("Not defining a word".to_string());
                }
                if self.current_word_name.is_empty() {
                    return Err("No word name specified".to_string());
                }

                let name = self.current_word_name.clone();
                let def = self.current_definition.clone();
                self.dictionary.insert(name, Word::UserDefined(def));

                self.compiling = false;
                self.current_word_name.clear();
                self.current_definition.clear();
                continue;
            }

            // If we're compiling, collect tokens
            if self.compiling {
                if self.current_word_name.is_empty() {
                    // First token after : is the word name
                    self.current_word_name = token.clone();
                } else {
                    // Rest are the definition
                    self.current_definition.push(token.clone());
                }
                continue;
            }

            // Not compiling - execute the token normally
            self.eval_token(&token)?;
        }

        Ok(())
    }

    // Helper method to evaluate a single token
    fn eval_token(&mut self, token: &str) -> Result<(), String> {
        // Check for string literal
        if token.starts_with("STR:") {
            let s = &token[4..];
            self.stack.push(Type::String(s.to_string()));
            return Ok(());
        }

        // Variable storage
        if token.ends_with("!") && token.len() > 1 {
            let var_name = &token[..token.len() - 1];
            let value = self
                .stack
                .pop()
                .ok_or("Stack underflow! Need a value to store")?;
            self.variables.insert(var_name.to_string(), value);
            return Ok(());
        }

        // Variable retrieval
        if token.ends_with("@") && token.len() > 1 {
            let var_name = &token[..token.len() - 1];
            let value = self
                .variables
                .get(var_name)
                .ok_or_else(|| format!("Variable '{}' not found", var_name))?
                .clone();
            self.stack.push(value);
            return Ok(());
        }

        // Floats
        if token.contains('.') && token.parse::<f64>().is_ok() {
            let f = token.parse::<f64>().unwrap();
            self.stack.push(Type::Float(f));
            return Ok(());
        }

        // Integers
        if let Ok(n) = token.parse::<i64>() {
            self.stack.push(Type::Int(n));
            return Ok(());
        }

        // Dictionary lookup
        if let Some(word) = self.dictionary.get(token).cloned() {
            match word {
                Word::Native(func) => return func(self),
                Word::UserDefined(tokens) => {
                    // Execute each token in the definition
                    for t in tokens {
                        self.eval_token(&t)?;
                    }
                    return Ok(());
                }
            }
        }

        Err(format!("Unknown token: {}", token))
    }

    // Show what's on the stack
    pub fn show_stack(&self) {
        print!("Stack: [");
        for (i, val) in self.stack.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            match val {
                Type::Int(n) => print!("{}", n),
                Type::String(s) => print!("\"{}\"", s),
                Type::Float(f) => print!("{}", f),
                Type::Bool(b) => print!("{}", b),
            }
        }
        println!("]");
    }
}
