use crate::models::{Expr, ParseError, Token};

fn find_matching_paren(tokens: &[Token], start: usize) -> Option<usize> {
    let mut depth = 0;
    for j in start + 1..tokens.len() {
        match tokens[j] {
            Token::LParen(_) => depth += 1,
            Token::RParen(_) => {
                if depth == 0 {
                    return Some(j);
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    None
}

fn combine_exprs(exprs: Vec<Expr>) -> Result<Expr, ParseError> {
    exprs
        .into_iter()
        .reduce(|acc, item| Expr::App(Box::new(acc), Box::new(item)))
        .ok_or_else(|| ParseError::EmptyExprList(0))
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut i = 0;
    let mut exprs = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::LParen(paren_index) => match find_matching_paren(tokens, i) {
                Some(j) => {
                    let inside_expr = parse(&tokens[i + 1..j])?;
                    exprs.push(inside_expr);
                    i = j;
                }
                None => return Err(ParseError::UnclosedParen(*paren_index)),
            },
            Token::RParen(paren_index) => {
                return Err(ParseError::UnopenedParen(*paren_index));
            }
            Token::Lambda(lambda_index) => {
                if let Some(Token::Term(_, var)) = tokens.get(i + 1) {
                    if tokens.len() > i + 2 {
                        let body = parse(&tokens[i + 2..])?;
                        exprs.push(Expr::Abs(var.clone(), Box::new(body)));
                        break;
                    } else {
                        return Err(ParseError::MissingLambdaBody(*lambda_index));
                    }
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

    combine_exprs(exprs)
}
