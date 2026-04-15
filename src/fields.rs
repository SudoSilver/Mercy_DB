#[derive(Debug, Clone)]
pub enum Types {
    StringInstance,
    Int,
    Uint,
    Bool,
    Float,
    Ind,
}

#[derive(Debug)]
pub enum Value {
    StringInstance(String),
    Int(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    Ind(usize),
}

#[derive(Debug)]
pub struct Entrie {
    pub fields: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Types,
}

impl Field {
    pub fn from(name: String, ty_temp: &str) -> Self {
        let ty = match ty_temp {
            "int" => Types::Int,
            "string" => Types::StringInstance,
            "uint" => Types::Uint,
            "bool" => Types::Bool,
            "float" => Types::Float,
            "ind" => Types::Ind,
            _ => panic!("[ERROR]: Unknown type {}", ty_temp),
        };
        return Self {
            name,
            ty,
        };
    }
}
