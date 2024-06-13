use anyhow::{bail, Result};
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

pub fn as_string(val: &Value) -> Result<&String> {
    match val {
        Value::Str(str) => Ok(str),
        _ => bail!("invalid string type {}", val),
    }
}

#[allow(dead_code)]
pub fn as_f64(val: &Value) -> Result<f64> {
    match val {
        Value::Num(num) => Ok(*num),
        _ => bail!("invalid number type {}", val),
    }
}

#[allow(dead_code)]
pub fn as_bool(val: &Value) -> Result<bool> {
    match val {
        Value::Bool(bool) => Ok(*bool),
        _ => bail!("invalid bool type {}", val),
    }
}

#[allow(dead_code)]
pub fn as_vec(val: &Value) -> Result<&Vec<Value>> {
    match val {
        Value::Array(arr) => Ok(arr),
        _ => bail!("invalid array type {}", val),
    }
}

#[allow(dead_code)]
pub fn as_hash(val: &Value) -> Result<&Vec<(String, Value)>> {
    match val {
        Value::Object(obj) => Ok(obj),
        _ => bail!("invalid object type {}", val),
    }
}
