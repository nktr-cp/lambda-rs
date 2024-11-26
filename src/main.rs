mod lexer;
mod models;
mod parser;
mod substitute;
use lexer::tokenize;
use models::Expr;
use parser::parse;
use std::fmt::{Display, Error, Formatter};
use std::io;
use substitute::substitute;

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Expr::Var(s) => write!(f, "{}", s),
            Expr::Abs(s, e) => write!(f, "λ{}.{}", s, e),
            Expr::App(u, v) => {
                match u.as_ref() {
                    Expr::Abs(_, _) => write!(f, "({}) ", u),
                    _ => write!(f, "{} ", u),
                }?;
                match v.as_ref() {
                    Expr::Abs(_, _) => write!(f, "({})", v),
                    Expr::App(_, _) => write!(f, "({})", v),
                    _ => write!(f, "{}", v),
                }
            }
        }
    }
}

fn test_apply(exp: Expr) -> Expr {
    match exp {
        Expr::App(l, r) => match *l {
            Expr::Abs(v, b) => substitute(*b, v, &r),
            _ => panic!("unimplemented"),
        },
        _ => panic!("also unimplemented"),
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let tokens = tokenize(&buf);
    let expr = parse(&tokens).unwrap();
    let result = test_apply(expr);
    println!("{}", result);
}
