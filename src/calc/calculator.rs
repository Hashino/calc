use std::cell::RefCell;

use crate::calc::parser::{BinaryOperator, Parser, Token, UnaryOperator};
use crate::log::{Level, log};

// Thread-local storage for the last computed result for reuse in expressions
thread_local! {
    static LAST_RESULT: RefCell<Option<f64>> = RefCell::new(None);
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

    // Check if result is NaN (indicates an error occurred during calculation)
    if result.is_nan() {
        return Err("Mathematical error occurred".to_string());
    }

    // Save the result for future use in subsequent expressions
    LAST_RESULT.with(|last_result| {
        *last_result.borrow_mut() = Some(result);
    });

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
                        log(
                            Level::Warning,
                            "Factorial of negative or non-integer number",
                        );
                        f64::NAN
                    } else {
                        // Compute factorial using product of range
                        (1..=operand as u64).product::<u64>() as f64
                    }
                }
                UnaryOperator::SquareRoot => {
                    // Square root: sqrt(x) = x^(1/2), for non-negative numbers
                    if operand < 0.0 {
                        log(Level::Warning, "Square root of negative number encountered");
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
                        log(
                            Level::Warning,
                            "Natural logarithm of non-positive number encountered",
                        );
                        f64::NAN
                    } else {
                        operand.ln()
                    }
                }
                UnaryOperator::Floor => operand.floor(), // Floor: largest integer <= x
                UnaryOperator::Ceil => operand.ceil(),   // Ceiling: smallest integer >= x
                UnaryOperator::Abs => operand.abs(),     // Absolute value: |x|
                UnaryOperator::Round => operand.round(), // Round to nearest integer
                UnaryOperator::Negate => -operand,       // Unary minus: -x
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
                        log(Level::Warning, "Division by zero encountered");
                        f64::NAN
                    }
                    _ => left / right,
                },
                BinaryOperator::Power => left.powf(right),
                BinaryOperator::Modulo => left % right,
                BinaryOperator::Log => {
                    // Logarithm: log_base(right) of left, with domain checks
                    if left <= 0.0 || right <= 0.0 || right == 1.0 {
                        log(Level::Warning, "Invalid logarithm base or argument");
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
            // Retrieve the last computed result from thread-local storage
            LAST_RESULT.with(|last_result| match *last_result.borrow() {
                Some(value) => value,
                None => {
                    log(Level::Warning, "No last result available");
                    f64::NAN
                }
            })
        }
    }
}
