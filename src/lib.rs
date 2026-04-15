mod fields;
mod tokens;
mod parse;

use crate::fields::{ Field, Entrie };
use crate::tokens::tokenize;
use crate::parse::{ parser, parse_db };

use std::fs;

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<Field>,
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

#[derive(Debug)]
pub struct DB {
    pub schema: Schema,
    pub entry_size: usize,
    pub values: Vec<Entrie>,
}

impl DB {
    pub fn from(schema: Schema, path: &str) -> Self {
        let entry_size = schema.fields.len();
        let fields = schema.fields.clone();

        let contents: String = fs::read_to_string(path)
            .expect(&format!("[ERROR]: Unable to read file {}", path));
        
        let chars: Vec<char> = contents.chars().collect(); 
        let tokens = tokenize(chars);
        let values: Vec<Entrie> = parse_db(tokens, entry_size, fields);

        return Self {
            schema,
            entry_size,
            values,
        }
    }
}
