use calculator::parsemath::token::*;
use calculator::parsemath::tokenizer::*;

#[test]
fn test_positive_integer() {
    let mut tokenizer = Tokenizer::new("34");
    assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
}

#[test]
fn test_decimal_number() {
    let mut tokenizer = Tokenizer::new("34.5");
    assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
}

#[test]
fn test_integer_add_float() {
    let mut tokenizer = Tokenizer::new("1+2.24");
    assert_eq!(tokenizer.next().unwrap(), Token::Num(1.0));
    assert_eq!(tokenizer.next().unwrap(), Token::Add);
    assert_eq!(tokenizer.next().unwrap(), Token::Num(2.24));
}

#[test]
fn test_parentheses() {
    let mut tokenizer = Tokenizer::new("(1+2.24)");
    assert_eq!(tokenizer.next().unwrap(), Token::LeftParen);
    assert_eq!(tokenizer.next().unwrap(), Token::Num(1.0));
    assert_eq!(tokenizer.next().unwrap(), Token::Add);
    assert_eq!(tokenizer.next().unwrap(), Token::Num(2.24));
    assert_eq!(tokenizer.next().unwrap(), Token::RightParen);
}

#[test]
fn test_whitespace() {
    let mut tokenizer = Tokenizer::new("1 +    2 ");
    assert_eq!(tokenizer.next().unwrap(), Token::Num(1.0));
    assert_eq!(tokenizer.next().unwrap(), Token::Add);
    assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
    assert_eq!(tokenizer.next().unwrap(), Token::EOF);
}
