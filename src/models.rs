#[derive(Clone)]
pub enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    LParen(usize),
    RParen(usize),
    Lambda(usize),
    Term(usize, String),
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedParen(usize),
    UnopenedParen(usize),
    MissingLambdaVar(usize),
    MissingLambdaBody(usize),
    EmptyExprList(usize),
}
