pub(super) enum Token {
    Unary(UnaryToken),   // Unary operations like sin, sqrt, !
    Binary(BinaryToken), // Binary operations like +, -, *, /
    Value(f64),          // Literal numbers
    LastResult,          // Reference to the last computed result
    Constant(Constant),  // Mathematical constants like pi, e
}

pub(super) enum Constant {
    Pi,
    E,
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

    pub(crate) fn parse(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
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
        // Handle top-level expressions starting with binary operators (use last result as left operand)
        if precedence == Precedence::Lowest {
            if let Some(token) = self.try_parse_binary_with_last_result()? {
                return Ok(token);
            }
        }

        let mut left = self.parse_primary()?;
        left = self.parse_postfix_operators(left)?;
        left = self.parse_binary_operators(left, precedence)?;

        Ok(left)
    }

    fn try_parse_binary_with_last_result(
        &mut self,
    ) -> Result<Option<Token>, Box<dyn std::error::Error>> {
        self.skip_whitespace();

        if let Some(&c) = self.chars.peek() {
            if let Some(operator) = self.try_parse_single_char_binary_operator(c) {
                self.chars.next();
                let right = self.parse_expression(Precedence::Unary)?;
                return Ok(Some(Token::Binary(BinaryToken {
                    left: Box::new(Token::LastResult),
                    operation: operator,
                    right: Box::new(right),
                })));
            }
        }

        Ok(None)
    }

    fn parse_binary_operators(
        &mut self,
        mut left: Token,
        precedence: Precedence,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        loop {
            self.skip_whitespace();

            let next_char = match self.chars.peek() {
                Some(&c) => c,
                None => break,
            };

            let op_precedence = self.get_precedence(next_char);

            if op_precedence > precedence
                && let Some(operator) = self.parse_binary_operator()
            {
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

        Ok(left)
    }

    fn parse_postfix_operators(
        &mut self,
        mut expr: Token,
    ) -> Result<Token, Box<dyn std::error::Error>> {
        loop {
            self.skip_whitespace();
            if self.chars.peek() == Some(&'!') {
                self.chars.next();
                expr = Token::Unary(UnaryToken {
                    operation: UnaryOperator::Factorial,
                    operand: Box::new(expr),
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(&c) if c.is_alphabetic() => self.parse_unary_operator(),
            Some(&'-') => self.parse_unary_minus(),
            Some(&'(') => self.parse_parenthesized_expression(),
            Some(&c) => Err(format!("Unexpected character: '{}'", c).into()),
            None => Err("Unexpected end of input".into()),
        }
    }

    fn parse_unary_minus(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        self.chars.next(); // consume '-'
        let operand = self.parse_primary()?;
        Ok(Token::Unary(UnaryToken {
            operation: UnaryOperator::Negate,
            operand: Box::new(operand),
        }))
    }

    fn parse_parenthesized_expression(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        self.chars.next(); // consume '('
        let expr = self.parse_expression(Precedence::Lowest)?;
        self.skip_whitespace();

        if self.chars.next() != Some(')') {
            return Err("Expected ')'".into());
        }

        Ok(expr)
    }

    fn parse_number(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        let mut num_str = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '.' {
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
        let op_str = self.consume_alphabetic_word();

        // Check if it's a constant first
        if let Some(constant) = self.parse_constant(&op_str) {
            return Ok(Token::Constant(constant));
        }

        let operation = self.parse_unary_operator_type(&op_str)?;

        self.skip_whitespace();

        let operand = if self.chars.peek().is_none() {
            Box::new(Token::LastResult)
        } else {
            Box::new(self.parse_primary()?)
        };

        Ok(Token::Unary(UnaryToken { operation, operand }))
    }

    fn consume_alphabetic_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
                word.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        word
    }

    fn parse_constant(&self, op_str: &str) -> Option<Constant> {
        match op_str {
            "pi" => Some(Constant::Pi),
            "e" => Some(Constant::E),
            _ => None,
        }
    }

    fn parse_unary_operator_type(
        &self,
        op_str: &str,
    ) -> Result<UnaryOperator, Box<dyn std::error::Error>> {
        match op_str {
            "sqrt" => Ok(UnaryOperator::SquareRoot),
            "sin" => Ok(UnaryOperator::Sin),
            "cos" => Ok(UnaryOperator::Cos),
            "tan" => Ok(UnaryOperator::Tan),
            "ln" => Ok(UnaryOperator::Ln),
            "floor" => Ok(UnaryOperator::Floor),
            "ceil" => Ok(UnaryOperator::Ceil),
            "abs" => Ok(UnaryOperator::Abs),
            "round" => Ok(UnaryOperator::Round),
            _ => Err(format!("Unknown unary operator: {}", op_str).into()),
        }
    }

    fn parse_binary_operator(&mut self) -> Option<BinaryOperator> {
        match self.chars.peek() {
            Some(&'+') => {
                self.chars.next();
                Some(BinaryOperator::Add)
            }
            Some(&'-') => {
                self.chars.next();
                Some(BinaryOperator::Subtract)
            }
            Some(&'*') => {
                self.chars.next();
                Some(BinaryOperator::Multiply)
            }
            Some(&'/') => {
                self.chars.next();
                Some(BinaryOperator::Divide)
            }
            Some(&'^') => {
                self.chars.next();
                Some(BinaryOperator::Power)
            }
            Some(&'%') => {
                self.chars.next();
                Some(BinaryOperator::Modulo)
            }
            Some(&'l') => self.try_parse_log_operator(),
            _ => None,
        }
    }

    fn try_parse_single_char_binary_operator(&self, c: char) -> Option<BinaryOperator> {
        match c {
            '+' => Some(BinaryOperator::Add),
            '-' => Some(BinaryOperator::Subtract),
            '*' => Some(BinaryOperator::Multiply),
            '/' => Some(BinaryOperator::Divide),
            '^' => Some(BinaryOperator::Power),
            '%' => Some(BinaryOperator::Modulo),
            _ => None,
        }
    }

    fn try_parse_log_operator(&mut self) -> Option<BinaryOperator> {
        let saved_position = self.chars.clone();
        let word = self.consume_alphabetic_word();

        if word == "log" {
            Some(BinaryOperator::Log)
        } else {
            self.chars = saved_position;
            None
        }
    }

    fn get_precedence(&self, c: char) -> Precedence {
        match c {
            '+' | '-' => Precedence::Addition,
            '*' | '/' | '%' => Precedence::Multiplication,
            '^' => Precedence::Exponentiation,
            'l' if self.peek_word_matches("log") => Precedence::Exponentiation,
            _ => Precedence::Lowest,
        }
    }

    fn peek_word_matches(&self, word: &str) -> bool {
        let mut peekable = self.chars.clone();
        for expected_char in word.chars() {
            match peekable.next() {
                Some(c) if c == expected_char => continue,
                _ => return false,
            }
        }
        true
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
