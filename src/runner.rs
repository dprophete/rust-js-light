use std::collections::HashMap;
use itertools::Itertools;
use crate::ast::{InfixOp, Literal, Expr, Stmt, Prg};

pub struct Runner {
    vars: HashMap<String, Literal>
}

impl Runner {
    pub fn new() -> Self {
        Runner { 
            vars: HashMap::new()
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

    fn eval_expr(&mut self, expr: &Expr) -> Literal {
        match expr {
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Infix(infix, lhs_expr, rhs_expr) => {
                let lhs = self.eval_expr(lhs_expr);
                let rhs = self.eval_expr(rhs_expr);
                self.eval_infix(infix, lhs, rhs)
            },
            Expr::Ident(var) => self.vars.get(var).unwrap().clone(),
            _ => Literal::Null
        }
    }

    fn eval_literal(&mut self, literal: &Literal) -> Literal {
        match literal {
            Literal::Array(elts) => Literal::Array(
                elts
                    .into_iter()
                    .map(|elt| Expr::Literal(self.eval_expr(elt)))
                    .collect()),
            Literal::Object(props) => Literal::Object(
                props
                    .into_iter()
                    .map(|(name, val)| (name.clone(), Expr::Literal(self.eval_expr(val))))
                    .collect()),
            _ => literal.clone()
        }
    }

    fn eval_infix(&mut self, infix: &InfixOp, lhs: Literal, rhs: Literal) -> Literal {
        match (lhs, rhs) {
            (Literal::Num(v1), Literal::Num(v2)) => Literal::Num(self.eval_infix_num(infix, v1, v2)),
            (Literal::Str(v1), Literal::Str(v2)) => Literal::Str(self.eval_infix_str(infix, v1, v2)),
            unknown => panic!("Unexpected infix: {:?}", unknown),
        }
    }

    fn eval_infix_num(&mut self, infix: &InfixOp, v1: f64, v2: f64) -> f64 {
        match infix {
            InfixOp::Add => v1 + v2,
            InfixOp::Sub => v1 - v2,
            InfixOp::Mul => v1 * v2,
            InfixOp::Div => v1 / v2,
            unknown => panic!("Unexpected unfix: {:?}", unknown),
        }
    }

    fn eval_infix_str(&mut self, infix: &InfixOp, v1: String, v2: String) -> String {
        match infix {
            InfixOp::Add => format!("{}{}", v1, v2),
            unknown => panic!("Unexpected unfix: {:?}", unknown),
        }
    }
}
