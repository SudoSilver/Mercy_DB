use crate::fields::Field;

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
