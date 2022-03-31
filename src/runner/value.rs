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
                    .into_iter()
                    .map(|(name, value)| format!("\"{}\": {}", name, value))
                    .collect();
                write!(f, "{{{}}}", res.join(", "))
            }
            Value::Array(elts) => {
                let res: Vec<String> = elts.into_iter().map(|elt| format!("{}", elt)).collect();
                write!(f, "[{}]", res.join(", "))
            }
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Num(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}
