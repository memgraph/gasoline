use super::*;

#[test]
fn null_to_string() {
    assert_eq!(Value::Null.to_string(), "Null");
}

#[test]
fn bool_to_string() {
    assert_eq!(Value::Bool(true).to_string(), "True");
    assert_eq!(Value::Bool(false).to_string(), "False");
}

#[test]
fn int_to_string() {
    assert_eq!(Value::Int(1).to_string(), "1");
}

#[test]
fn float_to_string() {
    assert_eq!(Value::Float(1.1).to_string(), "1.1");
}

#[test]
fn string_to_string() {
    assert_eq!(Value::String("abc".to_string()).to_string(), "abc");
}
