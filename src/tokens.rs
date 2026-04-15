pub fn tokenize(characters: Vec<char>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer: String = String::new();

    for c in characters {
        if c.is_ascii_alphanumeric() {
            buffer.push(c);
        } else if !buffer.is_empty() {
            tokens.push(buffer);
            buffer = String::new();
        }
    }

    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    return tokens;
}

pub fn tokenize_db(characters: Vec<char>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer: String = String::new();

    for c in characters {
        if c.is_ascii_alphanumeric() {
            buffer.push(c);
        } else if c == "," {
            if !buffer.is_empty() {
                tokens.push(buffer);
                buffer = String::new();
            }
            tokens.push(c.to_string());
        }else if !buffer.is_empty() {
            tokens.push(buffer);
            buffer = String::new();
        }
    }

    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    return tokens;
}   
