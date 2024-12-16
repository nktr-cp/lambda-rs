use crate::models::Token;
use regex::Regex;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let token_pattern =
        Regex::new(r"(?P<lambda>[λ\\])|(?P<lparen>\()|(?P<rparen>\))|(?P<term>[A-Za-z]+)").unwrap();

    for cap in token_pattern.captures_iter(input) {
        if let Some(_) = cap.name("lambda") {
            tokens.push(Token::Lambda(cap.get(0).unwrap().start()));
        } else if let Some(_) = cap.name("lparen") {
            tokens.push(Token::LParen(cap.get(0).unwrap().start()));
        } else if let Some(_) = cap.name("rparen") {
            tokens.push(Token::RParen(cap.get(0).unwrap().start()));
        } else if let Some(term) = cap.name("term") {
            tokens.push(Token::Term(
                cap.get(0).unwrap().start(),
                term.as_str().to_string(),
            ));
        }
    }

    tokens
}
