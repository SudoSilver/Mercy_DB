use crate::fields::{ Field, Entrie, Types, Value };

const TOKENS_PER_FIELD: usize = 2;

pub fn parser(tokens: Vec<String>, mut i: usize) -> Vec<Field> {
    let mut fields: Vec<Field> = Vec::new();

    while i + TOKENS_PER_FIELD <= tokens.len() {
            let field_name = tokens[i].clone();
            let field_type = tokens[i + 1].clone();

            let field = Field::from(field_name, &field_type);
            fields.push(field);

            i+=TOKENS_PER_FIELD;
    }

    return fields;
}

pub fn parse_db(
        tokens: Vec<String>,
        entrie_size: usize,
        fields_ty: Vec<Field>
    ) -> Vec<Entrie> {
    
    let mut entries: Vec<Entrie> = Vec::new();
    let mut i: usize = 0;

    while i < tokens.len() {
        let mut entrie = Entrie {
            fields: Vec::new(),
        };
        let mut j: usize = 0;
        while j < entrie_size {
            let value = match &fields_ty[j].ty {
                Types::Int => Value::Int(tokens[i+j].parse::<i64>().expect("[ERROR]: Invalid type")),
                Types::Uint => Value::Uint(tokens[i+j].parse::<u64>().expect("[ERROR]: Invalid type")),
                Types::Float => Value::Float(tokens[i+j].parse::<f64>().expect("[ERROR]: Invalid type")),
                Types::Bool => Value::Bool(tokens[i+j].parse::<bool>().expect("[ERROR]: Invalid type")),
                Types::StringInstance => Value::StringInstance(tokens[i + j].clone()),
                Types::Ind => Value::Ind(tokens[i+j].parse::<usize>().expect("[ERROR]: Invalid type")),
            };
            entrie.fields.push(value);
            j+=1;
        }
        entries.push(entrie);
        i += entrie_size;
    }

    return entries;
}
