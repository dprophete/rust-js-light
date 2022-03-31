use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub struct Prg {
    pub stmts: Vec<Stmt>
}

impl fmt::Display for Prg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.stmts {
            write!(f, "{}\n", stmt)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Assign(String, Box<Expr>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Assign(name, val) => {
                write!(f, "var {} = {};", name, val)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    Infix(InfixOp, Box<Expr>, Box<Expr>),
    Prefix(PrefixOp, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Infix(infix_op, lhs, rhs) => write!(f, "{} {} {}", lhs, infix_op, rhs),
            Expr::Prefix(prefix_op, lhs) => write!(f, "{} {}", prefix_op, lhs),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Object(Vec<(String, Expr)>),
    Array(Vec<Expr>),
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Object(props) => {
                let res: Vec<String> = props.into_iter().map(|(name, value)| format!("\"{}\": {}", name, value)).collect();
                write!(f, "{{{}}}", res.join(", "))
            }
            Literal::Array(elts) => {
                let res: Vec<String> = elts.into_iter().map(|elt| format!("{}", elt)).collect();
                write!(f, "[{}]", res.join(", "))
            }
            Literal::Str(s) => write!(f, "\"{}\"", s),
            Literal::Num(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Null => write!(f, "null"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Modulo,
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfixOp::Add => write!(f, "+"),
            InfixOp::Sub => write!(f, "-"),
            InfixOp::Mul => write!(f, "*"),
            InfixOp::Div => write!(f, "/"),
            InfixOp::Pow => write!(f, "^"),
            InfixOp::Modulo => write!(f, "%"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PrefixOp {
    Plus,
    Minus,
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrefixOp::Plus => write!(f, "+"),
            PrefixOp::Minus => write!(f, "-"),
        }
    }
}
