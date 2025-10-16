use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::calc::parser::{BinaryOperator, Parser, Token, UnaryOperator};

// Global static to store the last computed result for reuse in expressions
lazy_static! {
    static ref LAST_RESULT: Mutex<Option<f64>> = Mutex::new(None);
}

// Public function to evaluate a mathematical expression string
// Parses the input, solves the expression tree, and stores the result
pub fn evaluate(line: String) -> Result<f64, String> {
    let mut parser = Parser::new(&line);
    let root = match parser.parse() {
        Ok(token) => token,
        Err(e) => return Err(format!("Syntax Error: {}", e)),
    };
    let result = solve(root);

    // Save the result for future use in subsequent expressions
    {
        let mut last_result = LAST_RESULT.lock().unwrap();
        *last_result = Some(result);
    }

    Ok(result)
}

// Recursive function to solve/evaluate the expression tree represented by Token
fn solve(token: Token) -> f64 {
    match token {
        Token::Unary(t) => {
            // Evaluate the operand first
            let operand = solve(*t.operand);

            // Apply the unary operation
            let result = match t.operation {
                UnaryOperator::Factorial => {
                    // Factorial: n! = n * (n-1) * ... * 1, for non-negative integers
                    if operand < 0.0 || operand.fract() != 0.0 {
                        eprintln!("Warning: Factorial of negative or non-integer number");
                        f64::NAN
                    } else {
                        // Compute factorial using product of range
                        (1..=operand as u64).product::<u64>() as f64
                    }
                }
                UnaryOperator::SquareRoot => {
                    // Square root: sqrt(x) = x^(1/2), for non-negative numbers
                    if operand < 0.0 {
                        eprintln!("Warning: Square root of negative number encountered");
                        f64::NAN
                    } else {
                        operand.sqrt()
                    }
                }
                UnaryOperator::Sin => operand.sin(), // Sine function in radians
                UnaryOperator::Cos => operand.cos(), // Cosine function in radians
                UnaryOperator::Tan => operand.tan(), // Tangent function in radians
                UnaryOperator::Ln => {
                    // Natural logarithm: ln(x), for positive numbers
                    if operand <= 0.0 {
                        eprintln!("Warning: Natural logarithm of non-positive number encountered");
                        f64::NAN
                    } else {
                        operand.ln()
                    }
                }
            };

            result
        }
        Token::Binary(t) => {
            let left = solve(*t.left);
            let right = solve(*t.right);

            let result = match t.operation {
                BinaryOperator::Add => left + right,
                BinaryOperator::Subtract => left - right,
                BinaryOperator::Multiply => left * right,
                BinaryOperator::Divide => match right {
                    0.0 => {
                        eprintln!("Warning: Division by zero encountered");
                        f64::NAN
                    }
                    _ => left / right,
                },
                BinaryOperator::Power => left.powf(right),
                BinaryOperator::Modulo => left % right,
                BinaryOperator::Log => {
                    // Logarithm: log_base(right) of left, with domain checks
                    if left <= 0.0 || right <= 0.0 || right == 1.0 {
                        eprintln!("Warning: Invalid logarithm base or argument");
                        f64::NAN
                    } else {
                        left.log(right)
                    }
                }
            };

            result
        }
        Token::Value(n) => return n, // Literal number value
        Token::LastResult => {
            // Retrieve the last computed result from global storage
            let last_result = LAST_RESULT.lock().unwrap();
            match *last_result {
                Some(value) => value,
                None => {
                    eprintln!("Warning: No last result available");
                    f64::NAN
                }
            }
        }
    }
}
