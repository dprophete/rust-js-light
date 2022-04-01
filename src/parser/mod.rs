use crate::pest::Parser;
use crate::LangParser;
use crate::Rule;
use ast::{Expr, InfixOp, Literal, PrefixOp, Prg, Stmt};

use pest::error::Error;
use pest::iterators::Pair;

pub mod ast;

pub fn parse_json(str: &str) -> Result<Literal, Error<Rule>> {
    let pairs = LangParser::parse(Rule::literal, &str);
    let pair = pairs.unwrap().into_iter().next().unwrap();
    match parse_expr(pair) {
        Expr::Literal(literal) => Ok(literal),
        unknown => panic!("Unexpected json expr: {:?}", unknown),
    }
}

pub fn parse_prg(str: &str) -> Result<Prg, Error<Rule>> {
    let mut ast = vec![];
    let pairs = LangParser::parse(Rule::prg, &str);
    //println!("[DDA] mod::pairs {:?}", pairs);

    for pair in pairs? {
        ast.push(parse_stmt(pair))
    }
    Ok(Prg { stmts: ast })
}

fn parse_stmt(pair: Pair<Rule>) -> Stmt {
    match pair.as_rule() {
        Rule::assignment => {
            let mut inner_rules = pair.into_inner();
            let name = inner_rules.next().unwrap().as_str().to_string();
            let val = parse_expr(inner_rules.next().unwrap());
            Stmt::Assign(name, Box::new(val))
        }
        unknown => panic!("Unexpected statement: {:?}", unknown),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::fct_call => {
            let mut inner_rules = pair.into_inner();
            // node: we skip the function_start rule and go directly to the ident inside it
            let name = inner_rules.next().unwrap().into_inner().next().unwrap().as_str().to_string();
            let mut params = vec![];
            while let Some(nx_pair) = inner_rules.next() {
                params.push(parse_expr(nx_pair))
            }
            Expr::FctCall(name, params)
        }
        Rule::sum | Rule::factor | Rule::power => {
            let mut inner_rules = pair.into_inner();
            let lhs_pair = inner_rules.next().unwrap();
            let mut lhs = parse_expr(lhs_pair);
            loop {
                match (inner_rules.next(), inner_rules.next()) {
                    (Some(op_pair), Some(rhs_pair)) => {
                        let infix = parse_infix_op(op_pair);
                        let rhs = parse_expr(rhs_pair);
                        lhs = Expr::Infix(infix, Box::new(lhs), Box::new(rhs))
                    }
                    _ => break,
                }
            }
            lhs
        }
        Rule::unary => {
            let mut inner_rules = pair.into_inner();
            match (inner_rules.next(), inner_rules.next()) {
                (Some(op_pair), Some(rhs_pair)) => {
                    let prefix = parse_prefix_op(op_pair);
                    let rhs = parse_expr(rhs_pair);
                    Expr::Prefix(prefix, Box::new(rhs))
                }
                (Some(lhs_pair), None) => parse_expr(lhs_pair),
                unknown => panic!("Unexpected unary: {:?}", unknown),
            }
        }
        Rule::ident => Expr::Ident(pair.as_str().to_string()),
        Rule::literal => Expr::Literal(parse_literal(pair.into_inner().next().unwrap())),
        Rule::inparens => Expr::Parens(Box::new(parse_expr(pair.into_inner().next().unwrap()))),
        unknown => panic!("Unexpected expression: {:?}", unknown),
    }
}

fn parse_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::object => Literal::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();
                    let name_pair = inner_rules.next().unwrap();
                    let name = match name_pair.as_rule() {
                        Rule::ident => name_pair.as_str().to_string(),
                        Rule::string => name_pair.into_inner().next().unwrap().as_str().to_string(),
                        unknown => panic!("Unexpected literal: {:?}", unknown),
                    };
                    let value = parse_expr(inner_rules.next().unwrap());
                    (name, value)
                })
                .collect(),
        ),
        Rule::array => Literal::Array(pair.into_inner().map(parse_expr).collect()),
        Rule::string => Literal::Str(pair.into_inner().next().unwrap().as_str().to_string()),
        Rule::number => Literal::Num(pair.as_str().parse().unwrap()),
        Rule::boolean => Literal::Bool(pair.as_str().parse().unwrap()),
        Rule::null => Literal::Null,
        unknown => panic!("Unexpected literal: {:?}", unknown),
    }
}

fn parse_infix_op(pair: Pair<Rule>) -> InfixOp {
    match pair.as_rule() {
        Rule::add => InfixOp::Add,
        Rule::sub => InfixOp::Sub,
        Rule::mul => InfixOp::Mul,
        Rule::div => InfixOp::Div,
        Rule::pow => InfixOp::Pow,
        Rule::modulo => InfixOp::Modulo,
        unknown => panic!("Unexpected infix: {:?}", unknown),
    }
}
fn parse_prefix_op(pair: Pair<Rule>) -> PrefixOp {
    match pair.as_rule() {
        Rule::add => PrefixOp::Plus,
        Rule::sub => PrefixOp::Minus,
        unknown => panic!("Unexpected prefix: {:?}", unknown),
    }
}
