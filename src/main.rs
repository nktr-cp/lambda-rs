mod models;
mod parse;
mod reduce;
mod tokenize;
use crate::models::{Expr, ParseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

fn main() {
    let mut buf = String::new();
    if let Err(e) = io::stdin().read_line(&mut buf) {
        eprintln!("Error reading input: {}", e);
        return;
    }

    let tokens = tokenize::tokenize(&buf);

    let expr = match parse::parse(&tokens) {
        Ok(e) => e,
        Err(e) => {
            match e {
                ParseError::UnclosedParen(index) => {
                    eprintln!("Error: Unclosed parenthesis at index {}", index)
                }
                ParseError::UnopenedParen(index) => {
                    eprintln!("Error: Unopened parenthesis at index {}", index)
                }
                ParseError::MissingLambdaVar(index) => {
                    eprintln!("Error: Missing lambda variable at index {}", index)
                }
                ParseError::MissingLambdaBody(index) => {
                    eprintln!("Error: Missing lambda body at index {}", index)
                }
                ParseError::EmptyExprList(index) => {
                    eprintln!("Error: Empty expression list at index {}", index)
                }
            }
            return;
        }
    };

    let result = reduce::reduce_expression(expr);
    println!("-> {}", result);
}
