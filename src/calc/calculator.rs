// Calculator module: handles evaluation of parsed expressions

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::calc::parser::{BinaryOperator, Parser, Token, UnaryOperator};

// Global static to store the last computed result for reuse in expressions
lazy_static! {
    static ref LAST_RESULT: Mutex<Option<f64>> = Mutex::new(None);
}

// Public function to evaluate a mathematical expression string
// Parses the input, solves the expression tree, and stores the result
pub fn evaluate(line: String) -> Result<f64, Box<dyn std::error::Error>> {
    let mut parser = Parser::new(&line);
    let root = parser.parse()?;
    let result = solve(root)?;

    // Save the result for future use in subsequent expressions
    {
        let mut last_result = LAST_RESULT.lock().unwrap();
        *last_result = Some(result);
    }

    Ok(result)
}

// Recursive function to solve/evaluate the expression tree represented by Token
fn solve(token: Token) -> Result<f64, Box<dyn std::error::Error>> {
    match token {
        Token::Unary(t) => {
            // Evaluate the operand first
            let operand = solve(*t.operand)?;

            // Apply the unary operation
            let result = match t.operation {
                UnaryOperator::Factorial => {
                    // Factorial: n! = n * (n-1) * ... * 1, for non-negative integers
                    if operand < 0.0 || operand.fract() != 0.0 {
                        return Err("Factorial is only defined for non-negative integers".into());
                    }
                    (1..=operand as u64).product::<u64>() as f64
                }
                UnaryOperator::SquareRoot => {
                    // Square root: sqrt(x) = x^(1/2), for non-negative numbers
                    if operand < 0.0 {
                        return Err("Square root is only defined for non-negative integers".into());
                    }
                    operand.sqrt()
                }
                UnaryOperator::Sin => operand.sin(), // Sine function in radians
                UnaryOperator::Cos => operand.cos(), // Cosine function in radians
                UnaryOperator::Tan => operand.tan(), // Tangent function in radians
                UnaryOperator::Ln => {
                    // Natural logarithm: ln(x), for positive numbers
                    if operand <= 0.0 {
                        return Err("Natural logarithm is only defined for positive numbers".into());
                    }
                    operand.ln()
                }
            };

            return Ok(result);
        }
        Token::Binary(t) => {
            // Evaluate left and right operands
            let left = solve(*t.left)?;
            let right = solve(*t.right)?;

            // Apply the binary operation
            let result = match t.operation {
                BinaryOperator::Add => left + right, // Addition
                BinaryOperator::Subtract => left - right, // Subtraction
                BinaryOperator::Multiply => left * right, // Multiplication
                BinaryOperator::Divide => match right { // Division with zero check
                    0.0 => return Err("Division by zero".into()),
                    _ => left / right,
                },
                BinaryOperator::Power => left.powf(right), // Exponentiation: left^right
                BinaryOperator::Modulo => left % right, // Modulo operation
                BinaryOperator::Log => {
                    // Logarithm: log_base(right) of left, with domain checks
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
        Token::Value(n) => Ok(n), // Literal number value
        Token::LastResult => {
            // Retrieve the last computed result from global storage
            let last_result = LAST_RESULT.lock().unwrap();
            match *last_result {
                Some(value) => Ok(value),
                None => Err("No previous result available".into()),
            }
        }
    }
}
