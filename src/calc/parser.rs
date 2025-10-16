pub(super) enum Token {
    Unary(UnaryToken),
    Binary(BinaryToken),
    Value(f64),
    LastResult, // New token to represent the last result
}

pub(super) struct UnaryToken {
    pub(super) operation: UnaryOperator,
    pub(super) operand: Box<Token>,
}

pub(super) struct BinaryToken {
    pub(super) left: Box<Token>,
    pub(super) operation: BinaryOperator,
    pub(super) right: Box<Token>,
}

pub(super) enum UnaryOperator {
    Factorial,  // !
    SquareRoot, // sqrt
    Sin,        // sin
    Cos,        // cos
    Tan,        // tan
    Ln,         // ln
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

#[derive(PartialEq, PartialOrd)]
pub(super) enum Precedence {
    Lowest,
    Addition,       // + -
    Multiplication, // * / %
    Exponentiation, // ^
    Unary,          // ! sqrt sin cos tan ln
}

pub(super) struct Parser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.skip_whitespace();
        if self.chars.peek().is_some() {
            return Err("Unexpected characters at end of input".into());
        }
        Ok(expression)
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        // Check if this expression starts with a binary operator (missing left operand)
        self.skip_whitespace();
        if let Some(&c) = self.chars.peek() {
            if self.is_binary_operator(c) {
                // Binary operator at the beginning - use last result as left operand
                let operator = self.parse_binary_operator(c).unwrap();
                self.chars.next(); // consume the operator
                let right = self.parse_expression(Precedence::Unary)?;

                return Ok(Token::Binary(BinaryToken {
                    left: Box::new(Token::LastResult),
                    operation: operator,
                    right: Box::new(right),
                }));
            }
        }

        let mut left = self.parse_primary()?;

        loop {
            self.skip_whitespace();
            let next_char = match self.chars.peek() {
                Some(c) => *c,
                None => break,
            };

            let op_precedence = self.get_precedence(next_char);
            if op_precedence <= precedence {
                break;
            }

            if let Some(operator) = self.parse_binary_operator(next_char) {
                self.chars.next(); // consume the operator
                let right = self.parse_expression(op_precedence)?;
                left = Token::Binary(BinaryToken {
                    left: Box::new(left),
                    operation: operator,
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        // Handle postfix factorial
        loop {
            self.skip_whitespace();
            if self.chars.peek() == Some(&'!') {
                self.chars.next();
                left = Token::Unary(UnaryToken {
                    operation: UnaryOperator::Factorial,
                    operand: Box::new(left),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&c) if c.is_digit(10) || c == '.' => self.parse_number(),
            Some(&c) if c.is_alphabetic() => self.parse_unary_operator(),
            Some(&'(') => {
                self.chars.next(); // consume '('
                let expr = self.parse_expression(Precedence::Lowest)?;
                self.skip_whitespace();
                if self.chars.next() != Some(')') {
                    return Err("Expected ')'".into());
                }
                Ok(expr)
            }
            _ => Err("Expected number, unary operator, or '(".into()),
        }
    }

    fn parse_number(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let mut num_str = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_digit(10) || c == '.' {
                num_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let value = num_str.parse::<f64>()?;
        Ok(Token::Value(value))
    }

    fn parse_unary_operator(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let mut op_str = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
                op_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let operation = match op_str.as_str() {
            "sqrt" => UnaryOperator::SquareRoot,
            "sin" => UnaryOperator::Sin,
            "cos" => UnaryOperator::Cos,
            "tan" => UnaryOperator::Tan,
            "ln" => UnaryOperator::Ln,
            _ => return Err(format!("Unknown unary operator: {}", op_str).into()),
        };

        self.skip_whitespace();

        // Check if there's an operand after the unary operator
        // If the next character is the end of input, use last result
        if self.chars.peek().is_none() {
            // No operand found - use last result
            return Ok(Token::Unary(UnaryToken {
                operation,
                operand: Box::new(Token::LastResult),
            }));
        }

        // Parse the operand normally
        let operand = self.parse_primary()?;

        Ok(Token::Unary(UnaryToken {
            operation,
            operand: Box::new(operand),
        }))
    }

    fn parse_binary_operator(&mut self, c: char) -> Option<BinaryOperator> {
        match c {
            '+' => Some(BinaryOperator::Add),
            '-' => Some(BinaryOperator::Subtract),
            '*' => Some(BinaryOperator::Multiply),
            '/' => Some(BinaryOperator::Divide),
            '^' => Some(BinaryOperator::Power),
            '%' => Some(BinaryOperator::Modulo),
            _ => {
                // Check for "log" operator
                if c == 'l' {
                    let mut log_str = String::new();
                    let mut peekable = self.chars.clone();
                    for _ in 0..3 {
                        if let Some(ch) = peekable.next() {
                            log_str.push(ch);
                        }
                    }
                    if log_str == "log" {
                        // Advance the iterator and return the operator
                        for _ in 0..3 {
                            self.chars.next();
                        }
                        return Some(BinaryOperator::Log);
                    }
                }
                None
            }
        }
    }

    fn is_binary_operator(&self, c: char) -> bool {
        return matches!(c, '+' | '-' | '*' | '/' | '^' | '%') || {
            if c == 'l' {
                let mut log_str = String::new();
                let mut peekable = self.chars.clone();
                for _ in 0..3 {
                    if let Some(ch) = peekable.next() {
                        log_str.push(ch);
                    }
                }
                return log_str == "log";
            }
            return false;
        };
    }

    fn get_precedence(&self, c: char) -> Precedence {
        match c {
            '+' | '-' => Precedence::Addition,
            '*' | '/' | '%' => Precedence::Multiplication,
            '^' | 'l' => Precedence::Exponentiation, // 'l' for log or ln
            _ => Precedence::Lowest,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }
}
