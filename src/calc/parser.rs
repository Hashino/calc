pub(super) enum Token {
    Unary(UnaryToken),   // Unary operations like sin, sqrt, !
    Binary(BinaryToken), // Binary operations like +, -, *, /
    Value(f64),          // Literal numbers
    LastResult,          // Reference to the last computed result
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
    Exponentiation, // ^ log
    Unary,          // ! sqrt sin cos tan ln (highest precedence)
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
        // Ensure no extra characters remain after parsing
        if self.chars.peek().is_some() {
            return Err("Unexpected characters at end of input".into());
        }
        Ok(expression)
    }

    // Parses an expression with precedence handling (recursive descent parser)
    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        // Handle expressions starting with binary operators (e.g., "- 5" uses last result as left)
        self.skip_whitespace();
        if let Some(&c) = self.chars.peek() {
            if self.is_binary_operator(c) {
                // Binary operator at start means use last result as implicit left operand
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

        // Parse the leftmost part of the expression
        let mut left = self.parse_primary()?;

        // Parse binary operators in precedence order
        loop {
            self.skip_whitespace();
            let next_char = match self.chars.peek() {
                Some(c) => *c,
                None => break, // End of input
            };

            let op_precedence = self.get_precedence(next_char);
            if op_precedence <= precedence {
                break; // Lower precedence, stop here
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
                break; // Not a binary operator
            }
        }

        // Handle postfix factorial operator '!'
        loop {
            self.skip_whitespace();
            if self.chars.peek() == Some(&'!') {
                self.chars.next(); // consume '!'
                left = Token::Unary(UnaryToken {
                    operation: UnaryOperator::Factorial,
                    operand: Box::new(left),
                });
            } else {
                break; // No more factorials
            }
        }

        Ok(left)
    }

    // Parses primary expressions: numbers, unary operators, or parenthesized expressions
    fn parse_primary(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&c) if c.is_digit(10) || c == '.' => self.parse_number(), // Numbers
            Some(&c) if c.is_alphabetic() => self.parse_unary_operator(), // Unary functions like sin, sqrt
            Some(&'(') => {
                self.chars.next(); // consume '('
                let expr = self.parse_expression(Precedence::Lowest)?; // Parse sub-expression
                self.skip_whitespace();
                if self.chars.next() != Some(')') {
                    return Err("Expected ')'".into());
                }
                Ok(expr)
            }
            _ => Err("Expected number, unary operator, or '(".into()),
        }
    }

    // Parses a numeric literal (integer or floating point)
    fn parse_number(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let mut num_str = String::new();

        // Collect digits and decimal point
        while let Some(&c) = self.chars.peek() {
            if c.is_digit(10) || c == '.' {
                num_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        // Parse the string into a float
        let value = num_str.parse::<f64>()?;
        Ok(Token::Value(value))
    }

    // Parses unary operators (functions like sin, sqrt, etc.)
    fn parse_unary_operator(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let mut op_str = String::new();

        // Collect alphabetic characters for the operator name
        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
                op_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        // Map string to operator enum
        let operation = match op_str.as_str() {
            "sqrt" => UnaryOperator::SquareRoot,
            "sin" => UnaryOperator::Sin,
            "cos" => UnaryOperator::Cos,
            "tan" => UnaryOperator::Tan,
            "ln" => UnaryOperator::Ln,
            _ => return Err(format!("Unknown unary operator: {}", op_str).into()),
        };

        self.skip_whitespace();

        // Handle case where no operand is provided (use last result)
        if self.chars.peek().is_none() {
            return Ok(Token::Unary(UnaryToken {
                operation,
                operand: Box::new(Token::LastResult),
            }));
        }

        // Parse the operand
        let operand = self.parse_primary()?;

        Ok(Token::Unary(UnaryToken {
            operation,
            operand: Box::new(operand),
        }))
    }

    // Parses a binary operator from the current character
    fn parse_binary_operator(&mut self, c: char) -> Option<BinaryOperator> {
        match c {
            '+' => Some(BinaryOperator::Add),
            '-' => Some(BinaryOperator::Subtract),
            '*' => Some(BinaryOperator::Multiply),
            '/' => Some(BinaryOperator::Divide),
            '^' => Some(BinaryOperator::Power),
            '%' => Some(BinaryOperator::Modulo),
            _ => {
                // Special handling for "log" which is 3 characters
                if c == 'l' {
                    let mut log_str = String::new();
                    let mut peekable = self.chars.clone();
                    for _ in 0..3 {
                        if let Some(ch) = peekable.next() {
                            log_str.push(ch);
                        }
                    }
                    if log_str == "log" {
                        // Consume the "log" characters
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

    // Checks if the given character starts a binary operator
    fn is_binary_operator(&self, c: char) -> bool {
        return matches!(c, '+' | '-' | '*' | '/' | '^' | '%') || {
            if c == 'l' {
                // Check if it's "log"
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

    // Returns the precedence level of the operator starting with the given character
    fn get_precedence(&self, c: char) -> Precedence {
        match c {
            '+' | '-' => Precedence::Addition,
            '*' | '/' | '%' => Precedence::Multiplication,
            '^' | 'l' => Precedence::Exponentiation, // 'l' for log or ln
            _ => Precedence::Lowest,
        }
    }

    // Skips over whitespace characters in the input
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
