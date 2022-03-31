use crate::parser;
use crate::parser::ast::{Expr, InfixOp, Literal, Prg, Stmt};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use value::Value;

mod value;

pub struct Runner {
    vars: HashMap<String, Value>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            vars: HashMap::new(),
        }
    }

    pub fn run_prg(&mut self, prg: &Prg) -> () {
        for stmt in &prg.stmts {
            self.run_stmt(stmt)
        }
    }

    pub fn print_vars(&self) {
        println!("vars ({}):", self.vars.len());
        for name in self.vars.keys().sorted() {
            println!("  {} = {}", name, self.vars[name]);
        }
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> () {
        match stmt {
            Stmt::Assign(name, expr) => {
                let val = self.eval_expr(expr);
                self.vars.insert(name.clone(), val);
                ()
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Infix(infix, lhs_expr, rhs_expr) => {
                let lhs = self.eval_expr(lhs_expr);
                let rhs = self.eval_expr(rhs_expr);
                self.eval_infix(infix, lhs, rhs)
            }
            Expr::Ident(var) => self.vars.get(var).unwrap().clone(),
            Expr::Parens(expr2) => self.eval_expr(expr2),
            Expr::Prefix(_prefix, _lhs) => Value::Str(String::from("TODO")),
            Expr::FctCall(name, param_expr) => match name.as_str() {
                "load_json" => {
                    let param = self.eval_expr(param_expr);
                    match param {
                        Value::Str(path) => {
                            println!("[DDA] mod::path {:?}", path);
                            let file_content = fs::read_to_string(path).expect("cannot read file");
                            let literal = parser::parse_json(file_content.as_str()).unwrap();
                            self.eval_literal(&literal)
                        }
                        unknown => panic!("Unexpected path: {:?}", unknown),
                    }
                }
                unknown => panic!("Unexpected function: {:?}", unknown),
            },
        }
    }

    fn eval_literal(&mut self, literal: &Literal) -> Value {
        match literal {
            Literal::Array(elts) => {
                Value::Array(elts.into_iter().map(|elt| self.eval_expr(elt)).collect())
            }
            Literal::Object(props) => Value::Object(
                props
                    .into_iter()
                    .map(|(name, val)| (name.clone(), self.eval_expr(val)))
                    .collect(),
            ),
            Literal::Str(s) => Value::Str(s.clone()),
            Literal::Num(n) => Value::Num(*n),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Null => Value::Null,
        }
    }

    fn eval_infix(&mut self, infix: &InfixOp, lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Num(v1), Value::Num(v2)) => Value::Num(self.eval_infix_num(infix, v1, v2)),
            (Value::Str(v1), Value::Str(v2)) => Value::Str(self.eval_infix_str(infix, v1, v2)),
            unknown => panic!("Unexpected infix: {:?}", unknown),
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

    fn eval_infix_str(&mut self, infix: &InfixOp, v1: String, v2: String) -> String {
        match infix {
            InfixOp::Add => format!("{}{}", v1, v2),
            unknown => panic!("Unexpected unfix: {:?}", unknown),
        }
    }
}
