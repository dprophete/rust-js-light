use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Object(Vec<(String, Value)>),
    Array(Vec<Value>),
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Object(props) => {
                let res: Vec<String> = props
                    .iter()
                    .map(|(name, value)| format!("\"{}\": {}", name, value))
                    .collect();
                write!(f, "{{{}}}", res.join(", "))
            }
            Value::Array(elts) => {
                let res: Vec<String> = elts.iter().map(|elt| format!("{}", elt)).collect();
                write!(f, "[{}]", res.join(", "))
            }
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Type {
    Object,
    Array,
    Str,
    Num,
    Bool,
    Null,
}

#[allow(dead_code)]
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Object => write!(f, "Object"),
            Type::Array => write!(f, "Array"),
            Type::Str => write!(f, "Str"),
            Type::Num => write!(f, "Num"),
            Type::Bool => write!(f, "Bool"),
            Type::Null => write!(f, "Null"),
        }
    }
}

pub fn as_string(val: &Value) -> &String {
    match val {
        Value::Str(str) => str,
        _ => panic!("invalid type for extract_str {}", val),
    }
}

#[allow(dead_code)]
pub fn as_f64(val: &Value) -> f64 {
    match val {
        Value::Num(num) => *num,
        _ => panic!("invalid type for extract_num {}", val),
    }
}

#[allow(dead_code)]
pub fn as_bool(val: &Value) -> bool {
    match val {
        Value::Bool(bool) => *bool,
        _ => panic!("invalid type for extract_bool {}", val),
    }
}

#[allow(dead_code)]
pub fn as_vec(val: &Value) -> &Vec<Value> {
    match val {
        Value::Array(arr) => arr,
        _ => panic!("invalid type for extract_array {}", val),
    }
}

#[allow(dead_code)]
pub fn as_hash(val: &Value) -> &Vec<(String, Value)> {
    match val {
        Value::Object(obj) => obj,
        _ => panic!("invalid type for extract_object {}", val),
    }
}
