use crate::calc::lexer::{self, Lexer};

use super::lexer::Token;
use std::{error::Error, fmt, iter::Peekable, vec::IntoIter};

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

pub(super) struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub(super) fn parse(line: &str) -> Result<Expr, String> {
        let tokens = Lexer::tokenize(line)?;

        if tokens.is_empty() {
            return Err("Empty expression".into());
        }

        let mut parser = Parser::new(tokens.clone());

        let result = parser.parse_expression()?;

        if parser.tokens.peek().is_some() {
            return Err("Unexpected tokens after expression".into());
        }

        Ok(result)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        todo!()
    }
}
