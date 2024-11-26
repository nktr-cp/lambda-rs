pub enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Token {
    LParen(usize),
    RParen(usize),
    Lambda(usize),
    Term(usize, String),
}
