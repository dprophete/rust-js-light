use crate::pest::Parser;
use crate::LangParser;
use crate::Rule;
use ast::{Expr, InfixOp, Literal, PrefixOp, Prg, Stmt};

use pest::error::Error as PestError;
use pest::iterators::Pair;

pub mod ast;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid json {0}")]
    InvalidJson(Expr),
    #[error(transparent)]
    Pest(#[from] PestError<Rule>),
    #[error("invalid infix {0:?}")]
    Infix(Rule),
    #[error("invalid prefix {0:?}")]
    Prefix(Rule),
    #[error("invalid litteral {0:?}")]
    Litteral(Rule),
    #[error("invalid expression {0:?}")]
    Expr(Rule),
    #[error("unexpected unary {0}")]
    Unary(String),
    #[error("invalid statement {0:?}")]
    Stmt(Rule),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_json(str: &str) -> Result<Literal> {
    let pairs = LangParser::parse(Rule::literal, str);
    let pair = pairs.unwrap().into_iter().next().unwrap();
    match parse_expr(pair)? {
        Expr::Literal(literal) => Ok(literal),
        other => Err(Error::InvalidJson(other)),
    }
}

pub fn parse_prg(str: &str) -> Result<Prg> {
    let mut ast = vec![];
    let pairs = LangParser::parse(Rule::prg, str);
    //println!("[DDA] mod::pairs {:?}", pairs);

    for pair in pairs? {
        ast.push(parse_stmt(pair)?)
    }
    Ok(Prg { stmts: ast })
}

fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    match pair.as_rule() {
        Rule::assignment => {
            let mut inner_rules = pair.into_inner();
            let name = inner_rules.next().unwrap().as_str().to_string();
            let val = parse_expr(inner_rules.next().unwrap())?;
            Ok(Stmt::Assign(name, Box::new(val)))
        }
        unknown => Err(Error::Stmt(unknown)),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::fct_call => {
            let mut inner_rules = pair.into_inner();
            // node: we skip the function_start rule and go directly to the ident inside it
            let name = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .to_string();
            let mut params = vec![];
            for nx_pair in inner_rules {
                params.push(parse_expr(nx_pair)?)
            }
            Ok(Expr::FctCall(name, params))
        }
        Rule::sum | Rule::factor | Rule::power => {
            let mut inner_rules = pair.into_inner();
            let lhs_pair = inner_rules.next().unwrap();
            let mut lhs = parse_expr(lhs_pair)?;
            while let (Some(op_pair), Some(rhs_pair)) = (inner_rules.next(), inner_rules.next()) {
                let infix = parse_infix_op(op_pair)?;
                let rhs = parse_expr(rhs_pair)?;
                lhs = Expr::Infix(infix, Box::new(lhs), Box::new(rhs))
            }
            Ok(lhs)
        }
        Rule::unary => {
            let mut inner_rules = pair.into_inner();
            match (inner_rules.next(), inner_rules.next()) {
                (Some(op_pair), Some(rhs_pair)) => {
                    let prefix = parse_prefix_op(op_pair)?;
                    let rhs = parse_expr(rhs_pair)?;
                    Ok(Expr::Prefix(prefix, Box::new(rhs)))
                }
                (Some(lhs_pair), None) => parse_expr(lhs_pair),
                unkown => Err(Error::Unary(format!("{:?}", unkown))),
            }
        }
        Rule::ident => Ok(Expr::Ident(pair.as_str().to_string())),
        Rule::literal => Ok(Expr::Literal(parse_literal(
            pair.into_inner().next().unwrap(),
        )?)),
        Rule::inparens => Ok(Expr::Parens(Box::new(parse_expr(
            pair.into_inner().next().unwrap(),
        )?))),
        unknown => Err(Error::Expr(unknown)),
    }
}

fn parse_literal(pair: Pair<Rule>) -> Result<Literal> {
    match pair.as_rule() {
        Rule::object => {
            let mut fields = vec![];
            for pair in pair.into_inner() {
                let mut inner_rules = pair.into_inner();
                let name_pair = inner_rules.next().unwrap();
                let name = (match name_pair.as_rule() {
                    Rule::ident => Ok(name_pair.as_str().to_string()),
                    Rule::string => Ok(name_pair.into_inner().next().unwrap().as_str().to_string()),
                    unknown => Err(Error::Litteral(unknown)),
                })?;
                let value = parse_expr(inner_rules.next().unwrap())?;
                fields.push((name, value))
            }
            Ok(Literal::Object(fields))
        }
        Rule::array => {
            let mut fields = vec![];
            for pair in pair.into_inner() {
                fields.push(parse_expr(pair)?)
            }
            Ok(Literal::Array(fields))
        }
        Rule::string => Ok(Literal::Str(
            pair.into_inner().next().unwrap().as_str().to_string(),
        )),
        Rule::number => Ok(Literal::Num(pair.as_str().parse().unwrap())),
        Rule::boolean => Ok(Literal::Bool(pair.as_str().parse().unwrap())),
        Rule::null => Ok(Literal::Null),
        unknown => Err(Error::Litteral(unknown)),
    }
}

fn parse_infix_op(pair: Pair<Rule>) -> Result<InfixOp> {
    match pair.as_rule() {
        Rule::add => Ok(InfixOp::Add),
        Rule::sub => Ok(InfixOp::Sub),
        Rule::mul => Ok(InfixOp::Mul),
        Rule::div => Ok(InfixOp::Div),
        Rule::pow => Ok(InfixOp::Pow),
        Rule::modulo => Ok(InfixOp::Modulo),
        unknown => Err(Error::Infix(unknown)),
    }
}
fn parse_prefix_op(pair: Pair<Rule>) -> Result<PrefixOp> {
    match pair.as_rule() {
        Rule::add => Ok(PrefixOp::Plus),
        Rule::sub => Ok(PrefixOp::Minus),
        unknown => Err(Error::Prefix(unknown)),
    }
}
