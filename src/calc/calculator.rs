use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::calc::parser::{BinaryOperator, Parser, Token, UnaryOperator};
use crate::log::{Level, log};

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

// Public function to evaluate with debug output
pub fn evaluate_with_debug(line: String, debug: bool) -> Result<f64, String> {
    let mut parser = Parser::new(&line);
    let root = match parser.parse() {
        Ok(token) => token,
        Err(e) => return Err(format!("Syntax Error: {}", e)),
    };

    if debug {
        println!("{}", format_ast(&root, 0));
    }

    let result = solve(root);

    // Save the result for future use in subsequent expressions
    {
        let mut last_result = LAST_RESULT.lock().unwrap();
        *last_result = Some(result);
    }

    Ok(result)
}

// Function to format the AST for debug output
fn format_ast(token: &Token, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match token {
        Token::Value(n) => format!("{}Value({})", indent_str, n),
        Token::LastResult => format!("{}LastResult", indent_str),
        Token::Unary(t) => {
            let op_str = match t.operation {
                UnaryOperator::Factorial => "Factorial",
                UnaryOperator::SquareRoot => "SquareRoot",
                UnaryOperator::Sin => "Sin",
                UnaryOperator::Cos => "Cos",
                UnaryOperator::Tan => "Tan",
                UnaryOperator::Ln => "Ln",
            };
            format!(
                "{}Unary({})\n{}",
                indent_str,
                op_str,
                format_ast(&t.operand, indent + 1)
            )
        }
        Token::Binary(t) => {
            let op_str = match t.operation {
                BinaryOperator::Add => "Add",
                BinaryOperator::Subtract => "Subtract",
                BinaryOperator::Multiply => "Multiply",
                BinaryOperator::Divide => "Divide",
                BinaryOperator::Power => "Power",
                BinaryOperator::Modulo => "Modulo",
                BinaryOperator::Log => "Log",
            };
            format!(
                "{}Binary({})\n{}\n{}",
                indent_str,
                op_str,
                format_ast(&t.left, indent + 1),
                format_ast(&t.right, indent + 1)
            )
        }
    }
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
                            "Factorial is undefined for negative numbers or non-integers",
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
                        log(
                            Level::Warning,
                            "Square root is undefined for negative numbers",
                        );
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
                            "Natural logarithm is undefined for non-positive numbers",
                        );
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
                BinaryOperator::Divide => {
                    if right == 0.0 {
                        log(Level::Warning, "Division by zero is undefined");
                        f64::NAN
                    } else {
                        left / right
                    }
                }
                BinaryOperator::Power => left.powf(right),
                BinaryOperator::Modulo => left % right,
                BinaryOperator::Log => {
                    // Logarithm: log_base(right) of left, with domain checks
                    if left <= 0.0 || right <= 0.0 || right == 1.0 {
                        log(
                            Level::Warning,
                            "Logarithm is undefined for non-positive arguments or base equal to 1",
                        );
                        f64::NAN
                    } else {
                        left.log(right)
                    }
                }
            };

            result
        }
        Token::Value(n) => n,
        Token::LastResult => {
            // Retrieve the last computed result from global storage
            let last_result = LAST_RESULT.lock().unwrap();
            match *last_result {
                Some(value) => value,
                None => {
                    log(Level::Warning, "No previous result available");
                    f64::NAN // If no last result, return NaN
                }
            }
        }
    }
}
