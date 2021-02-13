#![allow(dead_code, unused_variables)]

use super::tokenizer::Tokenizer;
use super::token::{Token, OperPrec};
use super::ast::Node;
use std::fmt;
use std::error::Error;

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
                    "invalid operator".into())),
        };
        Ok(Parser{
            tokenizer: tkzr,
            current_token: ct,
        })
    }
    
    // parse parses the expression holded by the tokenizer and 
    // generate an Abstract Syntax Tree(AST)
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
        // an valid AST must start with an number 
        let mut left_expr = self.parse_num()?;
        // stop generating an AST when the 
        while oper_prec < self.current_token.get_oper_prec() {
            let right_expr = self.convert_token_to_node(left_expr)?;
            left_expr = right_expr;
        }
        return Ok(left_expr);
    }

    // check_paren checks if the current token is the right parentheses
    fn check_paren(&mut self) -> Result<(), ParseError> {
        if let Token::RightParen = self.current_token {
            return self.get_next_token();
        }
        return Err(
            ParseError::InvalidOperator(
                "invalid character".into()))
    }
    
    // get_next_token moves the Parser's current token one step forward
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        self.current_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(
                ParseError::InvalidOperator(
                    "invalid character".into())),
        };
        Ok(())
    }
    
    // parse_num parses the numeric tokens and the clause enclosed in 
    // the parentheses
    fn parse_num(&mut self) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Num(f) => {
                self.get_next_token()?;
                return Ok(Node::Number(f));
            },

            Token::Subtract => {
                self.get_next_token()?;
                let ast = self.generate_ast(OperPrec::Negative)?;
                return Ok(Node::Negative(Box::new(ast)));
            },

            Token::LeftParen => {
                self.get_next_token()?;
                let ast = self.generate_ast(OperPrec::DefaultZero);
                self.check_paren()?;
                return ast;
            },

            _ => return Err(
                ParseError::UnableToParse(
                    format!("unable to parse token {}", self.current_token))),
                   
        }
    }
}

pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl Error for ParseError{}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnableToParse(e) => write!(f, "ParseError: {} ", e),

            Self::InvalidOperator(e) => write!(f, "ParseError: {}", e),
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnableToParse(e) => 
                write!(f, "{}:{}:{} ParseError: {} ", 
                    file!(), line!(), column!(), e),

            Self::InvalidOperator(e) => 
                write!(f, "{}:{}:{} ParseError: {}", 
                    file!(), line!(), column!(), e),
        }
    }
}
