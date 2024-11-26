use crate::models::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().enumerate().peekable();
    let mut current_term = String::new();
    while let Some((index, ch)) = chars.next() {
        let mut next_token = None;
        match ch {
            '(' => next_token = Some(Token::LParen(index + 1)),
            ')' => next_token = Some(Token::RParen(index + 1)),
            '\\' | 'λ' => next_token = Some(Token::Lambda(index + 1)),
            _ => {
                if !ch.is_whitespace() && ch != '.' {
                    current_term.push(ch);
                    continue;
                }
            }
        }

        // reach the end of a term
        if current_term.len() > 0 {
            tokens.push(Token::Term(index + 1 - current_term.len(), current_term));
            current_term = String::new();
        }

        match next_token {
            Some(token) => tokens.push(token),
            None => (),
        }
    }
    if current_term.len() > 0 {
        tokens.push(Token::Term(input.len() - current_term.len(), current_term));
    }
    tokens
}
