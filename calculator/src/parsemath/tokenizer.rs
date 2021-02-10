use super::token::Token;
use std::str::Chars;
use std::iter::Peekable;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Self {
        Tokenizer{expr: expr.chars().peekable()}
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut current_chr = self.expr.next();
        
        // ignore the white space
        while let Some(chr) = current_chr {
            if !chr.is_whitespace() {
               break; 
            }
            current_chr = self.expr.next();
        }

        match current_chr {
            Some(chr @ '0'..='9') => {
                let mut num = chr.to_string();
                while let Some('0'..='9') | Some('.') = self.expr.peek() {
                    num.push(self.expr.next()?);
                }
                Some(Token::Num(num.parse::<f64>().unwrap()))
            },
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}
