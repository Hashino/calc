use super::lexer::{ConstantType, FunctionType, Lexer, Token};
use std::error::Error;

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
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    fn parse(&mut self) -> Result<Expr, Box<dyn Error>> {
        todo!("Parser implementation will be added later")
    }
}
