use crate::parser::{
    self,
    ast::{Expr, InfixOp, Literal, Prg, Stmt},
};
use itertools::Itertools;
use std::collections::HashMap;
use value::Value;

mod builtins;
mod value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid infix {0} {1}")]
    Infix(Value, Value),
    #[error("invalid string infix {0}")]
    InfixStr(InfixOp),
    #[error("invalid number of params for {0}. Expected {1}, got {2}")]
    FunctionParams(String, usize, usize),
    #[error("invalid string infix {0}")]
    Function(String),
    #[error("parse error {0:?}")]
    ParseError(#[from] parser::Error),
    #[error("invalid string type {0}")]
    InvalidStr(Value),
    #[error("invalid float type {0}")]
    InvalidNum(Value),
    #[error("invalid bool type {0}")]
    InvalidBool(Value),
    #[error("invalid array type {0}")]
    InvalidArray(Value),
    #[error("invalid hash type {0}")]
    InvalidHash(Value),
    #[error("invalid file path {0}")]
    InvalidFile(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Runner {
    vars: HashMap<String, Value>,
    builtins: HashMap<String, builtins::Builtin>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            vars: HashMap::new(),
            builtins: builtins::new(),
        }
    }

    pub fn run_prg(&mut self, prg: &Prg) -> Result<()> {
        for stmt in &prg.stmts {
            self.run_stmt(stmt)?
        }
        Ok(())
    }

    pub fn print_vars(&self) {
        println!("vars ({}):", self.vars.len());
        for name in self.vars.keys().sorted() {
            println!("  {} = {}", name, self.vars[name]);
        }
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Assign(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.vars.insert(name.clone(), val);
            }
        }
        Ok(())
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Infix(infix, lhs_expr, rhs_expr) => {
                let lhs = self.eval_expr(lhs_expr)?;
                let rhs = self.eval_expr(rhs_expr)?;
                self.eval_infix(infix, lhs, rhs)
            }
            Expr::Ident(var) => Ok(self.vars.get(var).unwrap().clone()),
            Expr::Parens(expr2) => self.eval_expr(expr2),
            Expr::Prefix(_prefix, _lhs) => Ok(Value::Str(String::from("TODO"))),
            Expr::FctCall(name, params_expr) => {
                let mut params = vec![];
                for param_expr in params_expr.iter() {
                    params.push(self.eval_expr(param_expr)?)
                }
                match self.builtins.get(name) {
                    Some(builtin) => {
                        let nb_args = builtin.nb_args;
                        let func = builtin.func;
                        if params.len() == nb_args {
                            func(self, &params)
                        } else {
                            Err(Error::FunctionParams(
                                name.to_string(),
                                nb_args,
                                params.len(),
                            ))
                        }
                    }
                    _ => Err(Error::Function(name.to_string())),
                }
            }
        }
    }

    fn eval_literal(&mut self, literal: &Literal) -> Result<Value> {
        match literal {
            Literal::Array(elts) => {
                let mut parts = vec![];
                for elt in elts.iter() {
                    parts.push(self.eval_expr(elt)?);
                }
                Ok(Value::Array(parts))
            }
            Literal::Object(props) => {
                let mut parts = vec![];
                for (name, val) in props.iter() {
                    parts.push((name.clone(), self.eval_expr(val)?))
                }
                Ok(Value::Object(parts))
            }
            Literal::Str(s) => Ok(Value::Str(s.clone())),
            Literal::Num(n) => Ok(Value::Num(*n)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Null => Ok(Value::Null),
        }
    }

    fn eval_infix(&mut self, infix: &InfixOp, lhs: Value, rhs: Value) -> Result<Value> {
        match (lhs, rhs) {
            (Value::Num(v1), Value::Num(v2)) => Ok(Value::Num(self.eval_infix_num(infix, v1, v2))),
            (Value::Str(v1), Value::Str(v2)) => Ok(Value::Str(self.eval_infix_str(infix, v1, v2)?)),
            (lhs, rhs) => Err(Error::Infix(lhs, rhs)),
        }
    }

    fn eval_infix_num(&mut self, infix: &InfixOp, v1: f64, v2: f64) -> f64 {
        match infix {
            InfixOp::Add => v1 + v2,
            InfixOp::Sub => v1 - v2,
            InfixOp::Mul => v1 * v2,
            InfixOp::Div => v1 / v2,
            InfixOp::Pow => f32::powi(v1 as f32, v2 as i32) as f64,
            InfixOp::Modulo => v1 % v2,
        }
    }

    fn eval_infix_str(&mut self, infix: &InfixOp, v1: String, v2: String) -> Result<String> {
        match infix {
            InfixOp::Add => Ok(format!("{}{}", v1, v2)),
            unknown => Err(Error::InfixStr(unknown.clone())),
        }
    }
}
