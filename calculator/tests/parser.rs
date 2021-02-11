use calculator::parsemath::parser::Parser;
use calculator::parsemath::ast::Node::*;

macro_rules! add {
    ($left_node:expr, $right_node:expr) => {
        Add(Box::new($left_node), Box::new($right_node)) 
    }
}

macro_rules! negative {
    ($node:expr) => {
        Negative(Box::new($node)) 
    }
}

macro_rules! multiply {
    ($left_node:expr, $right_node:expr) => {
        Multiply(Box::new($left_node), Box::new($right_node)) 
    }
}

#[test]
fn test_add() {
    let mut parser = Parser::new("1+2").unwrap();
    let expected = add!(Number(1.0), Number(2.0));
    assert_eq!(parser.parse().unwrap(), expected);
}

#[test]
fn test_neg_add() {
    let mut parser = Parser::new("-2.2 + 3.4").unwrap();
    let expected = add! (negative!(Number(2.2)), Number(3.4));
    assert_eq!(parser.parse().unwrap(), expected);
}

#[test]
fn test_multiply_paren() {
    let mut parser = Parser::new("2*(2+3)").unwrap();
    let expected = multiply!(Number(2.0), add!(Number(2.0), Number(3.0)));
    assert_eq!(parser.parse().unwrap(), expected);
}
