// Tokenizer
pub fn tokenize(input: &str) -> Vec<String> {
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
