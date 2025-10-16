use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::calc::parser::{BinaryOperator, Parser, Token, UnaryOperator};

lazy_static! {
    static ref LAST_RESULT: Mutex<Option<f64>> = Mutex::new(None);
}

pub fn evaluate(line: String) -> Result<f64, Box<dyn std::error::Error>> {
    let mut parser = Parser::new(&line);
    let root = parser.parse()?;
    let result = solve(root)?;

    // Save the result for future use
    {
        let mut last_result = LAST_RESULT.lock().unwrap();
        *last_result = Some(result);
    }

    Ok(result)
}

fn solve(token: Token) -> Result<f64, Box<dyn std::error::Error>> {
    match token {
        Token::Unary(t) => {
            let operand = solve(*t.operand)?;

            let result = match t.operation {
                UnaryOperator::Factorial => {
                    if operand < 0.0 || operand.fract() != 0.0 {
                        return Err("Factorial is only defined for non-negative integers".into());
                    }
                    (1..=operand as u64).product::<u64>() as f64
                }
                UnaryOperator::SquareRoot => {
                    if operand < 0.0 {
                        return Err("Square root is only defined for non-negative integers".into());
                    }
                    operand.sqrt()
                }
                UnaryOperator::Sin => operand.sin(),
                UnaryOperator::Cos => operand.cos(),
                UnaryOperator::Tan => operand.tan(),
                UnaryOperator::Ln => {
                    if operand <= 0.0 {
                        return Err("Natural logarithm is only defined for positive numbers".into());
                    }
                    operand.ln()
                }
            };

            return Ok(result);
        }
        Token::Binary(t) => {
            let left = solve(*t.left)?;
            let right = solve(*t.right)?;

            let result = match t.operation {
                BinaryOperator::Add => left + right,
                BinaryOperator::Subtract => left - right,
                BinaryOperator::Multiply => left * right,
                BinaryOperator::Divide => match right {
                    0.0 => return Err("Division by zero".into()),
                    _ => left / right,
                },
                BinaryOperator::Power => left.powf(right),
                BinaryOperator::Modulo => left % right,
                BinaryOperator::Log => {
                    if left <= 0.0 || right <= 0.0 || right == 1.0 {
                        return Err(
                            "Logarithm is only defined for positive numbers and base != 1".into(),
                        );
                    }
                    left.log(right)
                }
            };

            return Ok(result);
        }
        Token::Value(n) => Ok(n),
        Token::LastResult => {
            let last_result = LAST_RESULT.lock().unwrap();
            match *last_result {
                Some(value) => Ok(value),
                None => Err("No previous result available".into()),
            }
        }
    }
}
