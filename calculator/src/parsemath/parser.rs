#![allow(dead_code, unused_variables)]

use super::tokenizer::Tokenizer;
use super::token::{Token, OperPrec};
use super::ast::Node;
use std::fmt;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Public methods of the Parser
impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut tkzr = Tokenizer::new(expr);
        let ct = match tkzr.next() {
            Some(tk) => tk,
            // if encountering None, the `fn new` will be returned directly
            None => return Err(ParseError::InvalidOperator(
                String::from("TODO"))),
        };
        Ok(Parser{
            tokenizer: tkzr,
            current_token: ct,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.generate_ast(OperPrec::DefaultZero)
    }
}

// Private methods of the Parser
impl<'a> Parser<'a> {
    // convert_token_to_node converts the arithmetic operator into a AST node
    // with its left and right operands
    fn convert_token_to_node(
        &mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                // the token Add must be followed by a valid token
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                return Ok(Node::Add(
                        Box::new(left_expr), 
                        Box::new(right_expr)));
            },

            Token::Subtract => {
                // the token Subtract must be followed by a valid token
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                return Ok(Node::Subtract(
                        Box::new(left_expr), 
                        Box::new(right_expr)));
            },

            Token::Multiply => {
                // the token Multiply must be followed by a valid token
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                return Ok(Node::Multiply(
                        Box::new(left_expr), 
                        Box::new(right_expr)));
            },

            Token::Divide => {
                // the token Divide must be followed by a valid token
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                return Ok(Node::Divide(
                        Box::new(left_expr), 
                        Box::new(right_expr)));
            },
            
            Token::Caret => {
                // the token Caret must be followed by a valid token
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Power)?;
                return Ok(Node::Caret(
                        Box::new(left_expr), 
                        Box::new(right_expr)));
            },

            _ =>  return Err(
                    ParseError::InvalidOperator(
                        format!("invalid operator {:?}", self.current_token)
                    )),
        }
    }

    fn generate_ast(
        &mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        unreachable!()
    }

    // check_paren checks if the current token is the right parentheses
    fn check_paren(&mut self) -> Result<(), ParseError> {
        if let Token::RightParen = self.current_token {
            return self.get_next_token();
        }
        return Err(
            ParseError::InvalidOperator(
                String::from("invalid character")))
    }
    
    // get_next_token moves the Parser's current token one step forward
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        self.current_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(
                ParseError::InvalidOperator(
                    String::from("invalid character"))),
        };
        Ok(())
    }
    
    // parse_num parses the numeric tokens and the clause enclosed in 
    // the parentheses
    fn parse_num(&mut self) -> Result<Node, ParseError> {
        unreachable!()
    }
}

pub enum ParseError {
    UnableoParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnableoParse(e) => write!(f, "fail to parse: {} ", e),

            Self::InvalidOperator(e) => write!(f, "fail to parse: {}", e),
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnableoParse(e) => 
                write!(f, "{}:{}:{} fail to parse: {} ", 
                    file!(), line!(), column!(), e),

            Self::InvalidOperator(e) => 
                write!(f, "{}:{}:{} fail to parse: {}", 
                    file!(), line!(), column!(), e),
        }
    }
}
