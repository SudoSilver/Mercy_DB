use std::collections::HashMap;

pub fn parser(tokens: Vec<String>, es: usize) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut row_name: String = String::new();
    let mut index: usize = 0;

    for token in tokens {
        if index == 0 {
            row_name = token;
        } else {
            buffer.push(token);
        }

        index += 1;

        if index == es {
            map.insert(row_name, buffer);
            buffer = Vec::new();
            row_name = String::new();
            index = 0;
        }
    }

    return map;
}
