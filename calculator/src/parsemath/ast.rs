use std::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

pub fn eval(n: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match n {
        Add(left_node, right_node) => 
            Ok(eval(*left_node)? + eval(*right_node)?),

        Subtract(left_node, right_node)=> 
            Ok(eval(*left_node)? - eval(*right_node)?),

        Multiply(left_node, right_node)=> 
            Ok(eval(*left_node)? * eval(*right_node)?),

        Divide(left_node, right_node)=> 
            Ok(eval(*left_node)?/eval(*right_node)?),

        Caret(left_node, right_node)=> 
            Ok(eval(*left_node)?.powf(eval(*right_node)?)),

        Negative(node)=> 
            Ok(-eval(*node)?),

        Number(n) => Ok(n),
    }
}
