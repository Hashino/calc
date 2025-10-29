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
        Parser::parse_expression(&mut tokens.iter().peekable())
    }

    fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParserError> {
        todo!()
    }
}
