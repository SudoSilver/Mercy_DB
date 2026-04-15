mod fields;
mod tokens;
mod parse;

use crate::fields::Field;
use crate::tokens::tokenize;
use crate::parse::parser;

use std::fs;

#[derive(Debug)]
pub struct Schema {
    name: String,
    fields: Vec<Field>,
}

impl Schema {
    pub fn from(path: &str) -> Self {
        let mut schema = Schema {
            name: String::new(),
            fields: Vec::new(),
        };
        let contents: String = fs::read_to_string(path)
            .expect(&format!("[ERROR]: Unable to read file {}", path));

        let characters: Vec<char> = contents.chars().collect();
        let mut index: usize = 0;

        let tokens = tokenize(characters);

        if tokens.len() <= 1 {
            panic!("[ERROR]: Invalid Schema structure");
        }

        if ((tokens.len() - 1) % 2) != 0 {
            panic!("[ERROR]: Invalid Schema structure");
        }

        schema.name = tokens[index].clone();
        index += 1;

        let fields = parser(tokens, index);
        schema.fields = fields;

        println!("[MERCY]: Schema created successfully");

        return schema;
    }
}

pub struct DB {
    schema: Schema,
    entry_size: usize,
    values: Vec<Value>
}

impl DB {
    pub fn from(schema: Schema) -> Self {
        let entry_size = Schema.fields.len();
    
        let contents: String = fs::read_to_string(path)
            .expect(&format!("[ERROR]: Unable to read file {}", path));
        
        let chars: Vec<char> = contents.chars().collect(); 
        let mut tokens = tokenize(chars);
    }
}
