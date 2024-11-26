use crate::models::{Expr, ParseError, Token};

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut i = 0;
    let mut exprs = Vec::new();
    while i < tokens.len() {
        match &tokens[i] {
            Token::LParen(paren_index) => {
                let mut depth = 0;
                let mut pushed_expr = false;
                for j in i + 1..tokens.len() {
                    match tokens[j] {
                        Token::LParen(_) => {
                            depth += 1;
                        }
                        Token::RParen(_) => {
                            if depth == 0 {
                                let inside_expr = parse(&tokens[i + 1..j])?;
                                exprs.push(inside_expr);
                                pushed_expr = true;
                                i = j;
                                break;
                            }
                            depth -= 1;
                        }
                        _ => (),
                    }
                }
                if !pushed_expr {
                    return Err(ParseError::UnclosedParen(*paren_index));
                }
            }
            Token::RParen(paren_index) => {
                return Err(ParseError::UnopenedParen(*paren_index));
            }
            Token::Lambda(lambda_index) => {
                if tokens.len() <= i + 2 {
                    return Err(ParseError::MissingLambdaBody(*lambda_index));
                }
                if let Some(Token::Term(_, var)) = tokens.get(i + 1) {
                    let body = parse(&tokens[i + 2..])?;
                    exprs.push(Expr::Abs(var.clone(), Box::new(body)));
                    i = tokens.len();
                } else {
                    return Err(ParseError::MissingLambdaVar(*lambda_index));
                }
            }
            Token::Term(_, var) => {
                exprs.push(Expr::Var(var.clone()));
            }
        }
        i += 1;
    }

    match exprs
        .into_iter()
        .reduce(|acc, item| Expr::App(Box::new(acc), Box::new(item)))
    {
        Some(expr) => Ok(expr),
        None => Err(ParseError::EmptyExprList(0)),
    }
}
