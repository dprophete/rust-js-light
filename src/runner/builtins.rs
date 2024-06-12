use crate::parser;
use crate::runner::value;
use crate::runner::Runner;
use crate::runner::Value;
use std::collections::HashMap;
use std::fs;

use crate::runner::Result;

pub type BuiltinFuncSign = fn(&mut Runner, &[Value]) -> Result<Value>;

pub struct Builtin {
    pub nb_args: usize,
    pub func: BuiltinFuncSign,
}

pub fn new() -> HashMap<String, Builtin> {
    let mut builtins = HashMap::<String, Builtin>::new();
    builtins.insert(
        String::from("min"),
        Builtin {
            nb_args: 2,
            func: builtin_min,
        },
    );
    builtins.insert(
        String::from("max"),
        Builtin {
            nb_args: 2,
            func: builtin_max,
        },
    );
    builtins.insert(
        String::from("load_json"),
        Builtin {
            nb_args: 1,
            func: builtin_load_json,
        },
    );
    builtins
}

fn builtin_load_json(runner: &mut Runner, params: &[Value]) -> Result<Value> {
    let path = value::as_string(params.get(0).unwrap())?;
    let file_content = fs::read_to_string(path)
        .map_err(|_| crate::runner::Error::InvalidFile(path.to_string()))?;
    let literal = parser::parse_json(file_content.as_str())?;
    runner.eval_literal(&literal)
}

fn builtin_min(_runner: &mut Runner, params: &[Value]) -> Result<Value> {
    let v1 = value::as_f64(params.get(0).unwrap())?;
    let v2 = value::as_f64(params.get(1).unwrap())?;
    Ok(Value::Num(v1.min(v2)))
}

fn builtin_max(_runner: &mut Runner, params: &[Value]) -> Result<Value> {
    let v1 = value::as_f64(params.get(0).unwrap())?;
    let v2 = value::as_f64(params.get(1).unwrap())?;
    Ok(Value::Num(v1.max(v2)))
}
