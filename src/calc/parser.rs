use super::lexer::Token;
use std::{error::Error, fmt};

pub(super) enum Expr {
    Operation(Operation),
    Value(Value),
}

pub(super) enum Operation {
    Unary {
        operation: UnaryOperator,
        operand: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operation: BinaryOperator,
        right: Box<Expr>,
    },
}

pub(super) enum Value {
    Number(f64),
    Constant(Constant),
    LastResult,
}

pub(super) enum Constant {
    Pi,
    E,
}

pub(super) enum UnaryOperator {
    Factorial,  // !
    SquareRoot, // sqrt
    Sin,        // sin
    Cos,        // cos
    Tan,        // tan
    Ln,         // ln
    Floor,      // floor
    Ceil,       // ceil
    Abs,        // abs
    Round,      // round
    Negate,     // unary minus
}

pub(super) enum BinaryOperator {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Power,    // ^
    Modulo,   // %
    Log,      // log
}

#[derive(Debug)]
pub(super) struct ParserError {
    message: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParserError {}

pub(super) struct Parser {}

impl Parser {
    pub(super) fn parse(tokens: Vec<Token>) -> Result<Expr, ParserError> {
        Parser::parse_expression(tokens)
    }

    fn parse_expression(mut tokens: Vec<Token>) -> Result<Expr, ParserError> {
        for i in 0..tokens.len() {
            match &tokens[i] {
                Token::Plus => {
                    let left_tokens: Vec<Token> = tokens.drain(0..i).collect();
                    let right_tokens: Vec<Token> = tokens.drain(1..).collect();

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(Parser::parse_expression(left_tokens)?),
                        operation: BinaryOperator::Add,
                        right: Box::new(Parser::parse_expression(right_tokens)?),
                    }));
                }
                Token::Minus => {
                    let left = Parser::parse_expression(tokens.drain(0..i).collect())?;
                    let right = Parser::parse_expression(tokens.drain(1..).collect())?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Subtract,
                        right: Box::new(right),
                    }));
                }
                _ => continue,
            }
        }

        for i in 0..tokens.len() {
            match &tokens[i] {
                Token::Multiply => {
                    let left = Parser::parse_expression(tokens.drain(0..i).collect())?;
                    let right = Parser::parse_expression(tokens.drain(1..).collect())?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Multiply,
                        right: Box::new(right),
                    }));
                }
                Token::Divide => {
                    let left = Parser::parse_expression(tokens.drain(0..i).collect())?;
                    let right = Parser::parse_expression(tokens.drain(1..).collect())?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Divide,
                        right: Box::new(right),
                    }));
                }
                _ => continue,
            }
        }

        for i in 0..tokens.len() {
            match &tokens[i] {
                Token::Number(value) => return Ok(Expr::Value(Value::Number(*value))),
                _ => continue,
            }
        }

        Err(ParserError {
            message: "Parsing not fully implemented".into(),
        })
    }
}
