use std::{error::Error, iter::Peekable, str::Chars};

pub(super) enum Expr {
    Operation(Operation), // Unary or binary operation
    Value(Value),         // Numeric value, constant, or last result
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

pub(super) fn parse(input: &str) -> Result<Expr, Box<dyn Error>> {
    // let mut parser = Parser::new(input);
    // parser.parse()
    Err("Parser not implemented".into())
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn parse(&mut self) -> Result<Expr, Box<dyn Error>> {
        todo!();
    }
}
