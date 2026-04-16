mod tokens;
mod parse;

use crate::tokens::tokenize;
use crate::parse::parser;

use std::fs;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::io::Write;

pub struct DB {
    entries: HashMap<String, Vec<String>>,
    entrie_size: usize,
    path: String,
}

impl DB {
    // note that es (entry size) is an unsigned integer
    // it should be the size of each row (Vec<String>)
    pub fn from(path: &str, es: usize) -> Option<Self> {
        let raw = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return None,
        };

        let chars: Vec<char> = raw.chars().collect();
        let tokens: Vec<String> = tokenize(chars);

        let entries = parser(tokens, es);

        return Some( Self {
            entries,
            entrie_size: es,
            path: path.to_string(),
        });
    }

    pub fn push(&mut self, name: String, row: Vec<String>) -> bool {
        if row.len() != self.entrie_size {
            return false;
        }

        if self.entries.contains_key(&name) {
            return false;
        }

        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path) {
                Ok(file) => file,
                Err(_) => return false,
            };

        let original_len = file.metadata().map(|m| m.len()).unwrap_or(0);

        if let Err(_) = write!(file, "\n{}", name) {
            let _ = file.set_len(original_len);
            return false;
        }

        if row.iter().try_for_each(|item| write!(file, "\n{}", item)).is_err() {
            let _ = file.set_len(original_len);
            return false;
        }


        self.entries.insert(name, row);

        return true;
    }

    pub fn read(&self, name: String) -> Option<&Vec<String>> {
        if let Some(data) = self.entries.get(&name) {
            return Some(data);
        }
        return None;
    }
}
