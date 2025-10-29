use std::cell::RefCell;
use std::f64::consts::{E, PI};

mod lexer;
mod parser;
use crate::calc::lexer::Lexer;
use crate::calc::parser::{
    BinaryOperator, Constant, Expr, Operation, Parser, UnaryOperator, Value,
};

#[cfg(test)]
mod tests;

// Thread-local storage for the last computed result for reuse in expressions
thread_local! {
    static LAST_RESULT: RefCell<Option<f64>> = RefCell::new(None);
}

fn print_ast(token: &Expr, depth: usize) {
    let indent = "  ".repeat(depth);
    match token {
        Expr::Operation(op) => match op {
            Operation::Unary { operation, operand } => {
                println!("{}Unary ({:?})", indent, operation);
                print_ast(operand, depth + 1);
            }
            Operation::Binary {
                left,
                operation,
                right,
            } => {
                println!("{}Binary ({:?})", indent, operation);
                print_ast(left, depth + 1);
                print_ast(right, depth + 1);
            }
        },
        Expr::Value(v) => match v {
            Value::Number(n) => println!("{}Number({})", indent, n),
            Value::Constant(c) => println!("{}Constant: {:?}", indent, c),
            Value::LastResult => println!("{}Last Result", indent),
        },
    }
}

pub struct Calculator;

impl Calculator {
    // Public function to evaluate a mathematical expression string
    // Parses the input, solves the expression tree, and stores the result
    pub fn evaluate(line: String) -> Result<f64, String> {
        let mut tokens = Lexer::tokenize(&line).map_err(|e| format!("Lexing Error: {}", e))?;
        let root = Parser::parse(&mut tokens).map_err(|e| format!("Parsing Error: {}", e))?;

        let result = Calculator::solve(root).map_err(|e| format!("Evaluation Error: {}", e))?;

        // Save the result for future use in subsequent expressions
        LAST_RESULT.set(Some(result));

        Ok(result)
    }

    pub fn evaluate_with_debug(line: String, debug: bool) -> Result<f64, String> {
        let mut tokens = Lexer::tokenize(&line).map_err(|e| format!("Lexing Error: {}", e))?;
        let root = Parser::parse(&mut tokens).map_err(|e| format!("Parsing Error: {}", e))?;

        if debug {
            print_ast(&root, 0);
        }

        let result = Calculator::solve(root).map_err(|e| format!("Evaluation Error: {}", e))?;

        // Save the result for future use in subsequent expressions
        LAST_RESULT.set(Some(result));

        Ok(result)
    }

    // Recursive function to solve/evaluate the expression tree represented by Token
    fn solve(token: Expr) -> Result<f64, String> {
        match token {
            Expr::Operation(op) => match op {
                Operation::Unary { operation, operand } => {
                    // Evaluate the operand first
                    let op = Calculator::solve(*operand)?;

                    // Apply the unary operation
                    match operation {
                        UnaryOperator::Factorial => {
                            // Factorial: n! = n * (n-1) * ... * 1, for non-negative integers
                            if op < 0.0 || op.fract() != 0.0 {
                                Err("Factorial of negative or non-integer number".into())
                            } else {
                                // Compute factorial using product of range
                                Ok((1..=op as u64).product::<u64>() as f64)
                            }
                        }
                        UnaryOperator::SquareRoot => {
                            // Square root: sqrt(x) = x^(1/2), for non-negative numbers
                            if op < 0.0 {
                                Err("Square root of negative number encountered".into())
                            } else {
                                Ok(op.sqrt())
                            }
                        }
                        UnaryOperator::Sin => Ok(op.sin()),
                        UnaryOperator::Cos => Ok(op.cos()),
                        UnaryOperator::Tan => Ok(op.tan()),
                        UnaryOperator::Ln => {
                            // Natural logarithm: ln(x), for positive numbers
                            if op <= 0.0 {
                                Err("Natural logarithm of non-positive number encountered".into())
                            } else {
                                Ok(op.ln())
                            }
                        }
                        UnaryOperator::Floor => Ok(op.floor()),
                        UnaryOperator::Ceil => Ok(op.ceil()),
                        UnaryOperator::Abs => Ok(op.abs()),
                        UnaryOperator::Round => Ok(op.round()),
                        UnaryOperator::Negate => Ok(-op),
                    }
                }
                Operation::Binary {
                    left,
                    operation,
                    right,
                } => {
                    let left = Calculator::solve(*left)?;
                    let right = Calculator::solve(*right)?;

                    match operation {
                        BinaryOperator::Add => Ok(left + right),
                        BinaryOperator::Subtract => Ok(left - right),
                        BinaryOperator::Multiply => Ok(left * right),
                        BinaryOperator::Divide => {
                            if right == 0.0 {
                                Err("Division by zero encountered".into())
                            } else {
                                Ok(left / right)
                            }
                        } //
                        BinaryOperator::Power => Ok(left.powf(right)),
                        BinaryOperator::Modulo => {
                            if right == 0.0 {
                                Err("Modulo by zero encountered".into())
                            } else {
                                Ok(left % right)
                            }
                        }
                        BinaryOperator::Log => {
                            // Logarithm: log_base(right) of left, with domain checks
                            if left <= 0.0 || right <= 0.0 || right == 1.0 {
                                Err("Invalid logarithm base or argument".into())
                            } else {
                                Ok(left.log(right))
                            }
                        }
                    }
                }
            },
            Expr::Value(v) => match v {
                Value::Number(n) => Ok(n), // Literal number value
                Value::Constant(c) => {
                    // Mathematical constants
                    match c {
                        Constant::Pi => Ok(PI),
                        Constant::E => Ok(E),
                    }
                }
                Value::LastResult => {
                    // Retrieve the last computed result from thread-local storage
                    LAST_RESULT
                        .with_borrow(|lr| lr.ok_or_else(|| "No last result available".into()))
                }
            },
        }
    }
}
