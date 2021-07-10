use std::string::ToString;

pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Null => format!("{}", "Null"),
            Value::Bool(x) => format!("{}", if *x { "True" } else { "False" }),
            Value::Int(x) => format!("{}", x),
            Value::Float(x) => format!("{}", x),
            Value::String(x) => format!("{}", x),
        }
    }
}

#[cfg(test)]
mod tests;
