use super::lexer::Token;
use std::{error::Error, fmt, iter::Peekable, slice::Iter};

#[derive(Debug)]
pub(super) enum Expr {
    Operation(Operation),
    Value(Value),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(super) enum Value {
    Number(f64),
    Constant(Constant),
    LastResult,
}

#[derive(Debug)]
pub(super) enum Constant {
    Pi,
    E,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub(super) fn parse(tokens: &mut Vec<Token>) -> Result<Expr, ParserError> {
        let mut token_iter = tokens.iter().peekable();
        let expr = Parser::parse_expression(&mut token_iter)?;

        if let Some(remaining) = token_iter.peek() {
            return Err(ParserError {
                message: format!("Unexpected token after expression: {:?}", remaining),
            });
        }

        Ok(expr)
    }

    fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
        if tokens.peek().is_none() {
            return Err(ParserError {
                message: String::from("Empty expression"),
            });
        }
        Parser::parse_first(tokens)
    }

    fn parse_first(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
        let left = Parser::parse_second(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::Plus => {
                    tokens.next();
                    let right = Parser::parse_second(tokens)?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Add,
                        right: Box::new(right),
                    }));
                }
                Token::Minus => {
                    tokens.next();
                    let right = Parser::parse_second(tokens)?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Subtract,
                        right: Box::new(right),
                    }));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_second(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
        let left = Parser::parse_third(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::Multiply => {
                    tokens.next();
                    let right = Parser::parse_third(tokens)?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Multiply,
                        right: Box::new(right),
                    }));
                }
                Token::Divide => {
                    tokens.next();
                    let right = Parser::parse_third(tokens)?;

                    return Ok(Expr::Operation(Operation::Binary {
                        left: Box::new(left),
                        operation: BinaryOperator::Divide,
                        right: Box::new(right),
                    }));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_third(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
        match tokens.peek() {
            Some(Token::Minus) => {
                tokens.next();
                let operand = Parser::parse_final(tokens)?;

                Ok(Expr::Operation(Operation::Unary {
                    operation: UnaryOperator::Negate,
                    operand: Box::new(operand),
                }))
            }
            _ => Parser::parse_final(tokens),
        }
    }

    fn parse_final(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
        match tokens.next() {
            Some(Token::Number(n)) => Ok(Expr::Value(Value::Number(*n))),
            Some(Token::LParen) => {
                let expr = Parser::parse_expression(tokens)?;

                match tokens.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(ParserError {
                        message: String::from("Expected closing parenthesis"),
                    }),
                }
            }
            Some(Token::Pi) => Ok(Expr::Value(Value::Constant(Constant::Pi))),
            Some(Token::E) => Ok(Expr::Value(Value::Constant(Constant::E))),
            Some(token) => Err(ParserError {
                message: format!("Unexpected token: {:?}", token),
            }),
            None => Err(ParserError {
                message: String::from("Unexpected end of input"),
            }),
        }
    }
}
