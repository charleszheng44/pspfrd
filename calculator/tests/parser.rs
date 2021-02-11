use calculator::parsemath::parser::Parser;
use calculator::parsemath::ast::Node::*;

#[test]
fn test_add() {
    let mut parser = Parser::new("1+2").unwrap();
    let expected = Add(
        Box::new(Number(1.0)), 
        Box::new(Number(2.0)));
    assert_eq!(parser.parse().unwrap(), expected);
}

#[test]
fn test_neg_add() {
    let mut parser = Parser::new("-2.2 + 3.4").unwrap();
    let expected = Add(
        Box::new(Negative(
                Box::new(Number(2.2)))), 
        Box::new(Number(3.4)));
    assert_eq!(parser.parse().unwrap(), expected);
}

#[test]
fn test_multiply_paren() {
    let mut parser = Parser::new("2*(2+3)").unwrap();
    let expected = Multiply(
        Box::new(Number(2.0)), 
        Box::new(Add(
                Box::new(Number(2.0)), 
                Box::new(Number(3.0)))));
    assert_eq!(parser.parse().unwrap(), expected);
}
