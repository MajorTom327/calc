use std::error;

use super::token::Node;

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    

    match expr {
        Node::Number(i) => Ok(i),
        Node::Add(left, right) => Ok(eval(*left)? + eval(*right)?),
        Node::Sub(left, right) => Ok(eval(*left)? - eval(*right)?),
        Node::Mul(left, right) => Ok(eval(*left)? * eval(*right)?),
        Node::Div(left, right) => Ok(eval(*left)? / eval(*right)?),
        Node::Pow(left, right) => Ok(eval(*left)?.powf(eval(*right)?)),
        Node::Neg(expr) => Ok(-eval(*expr)?),
    }
}