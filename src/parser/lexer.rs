use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;


pub struct Lexer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            expr: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.expr.next();
        match next_char {
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some('^') => Some(Token::Pow),
            Some('0'..='9') => {
                
                let mut number = next_char?.to_string();
                while let Some(ch) = self.expr.peek() {
                    if ch.is_numeric() {
                        number.push(self.expr.next()?);
                    } else if ch == &'.' {
                        number.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number.parse::<f64>().unwrap()))
            }   
            None => Some(Token::EOF),
            Some(_) => None
        }
    }
}
