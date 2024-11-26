mod lexer;
mod models;
use lexer::tokenize;
use models::Expr;
use std::fmt::{Display, Error, Formatter};

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

fn main() {
    let id = Expr::Abs(String::from("x"), Box::new(Expr::Var(String::from("x"))));
    println!("{}", id);

    let tokens = tokenize("λx.x");
    println!("{:?}", tokens);
}
