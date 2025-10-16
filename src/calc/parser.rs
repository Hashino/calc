use std::error::Error;

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

pub(super) struct Parser {
    chars: std::iter::Peekable<std::vec::IntoIter<char>>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let cleaned_chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
        Self {
            chars: cleaned_chars.into_iter().peekable(),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Token, Box<dyn Error>> {
        if self.is_empty() {
            return Err("Empty expression".into());
        }

        let result: Token;

        // Handle expressions starting with operators (using last result)
        if let Some(op) = self.chars.peek().and_then(|&ch| match ch {
            '+' => Some(BinaryOperator::Add),
            '-' => Some(BinaryOperator::Subtract),
            '*' => Some(BinaryOperator::Multiply),
            '/' => Some(BinaryOperator::Divide),
            '%' => Some(BinaryOperator::Modulo),
            '^' => Some(BinaryOperator::Power),
            _ => None,
        }) {
            self.consume_char();
            let right = self.parse_expression(0)?;
            result = Token::Binary(BinaryToken {
                left: Box::new(Token::LastResult),
                operation: op,
                right: Box::new(right),
            });
        } else if self.current_char() == Some('!') {
            // Handle factorial at start (e.g., "!" means "last_result!")
            self.consume_char();
            result = Token::Unary(UnaryToken {
                operation: UnaryOperator::Factorial,
                operand: Box::new(Token::LastResult),
            });
        } else {
            // Parse normal expression
            let root = self.parse_expression(0)?;
            result = root;
        }

        if !self.is_empty() {
            Err("Unexpected characters after expression".into())
        } else {
            Ok(result)
        }
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<Token, Box<dyn Error>> {
        let left = {
            // Handle unary functions (sin, cos, sqrt, etc.)
            if let Some(word) = self.peek_word() {
                if let Some(unary_op) = match word.as_str() {
                    "sqrt" => Some(UnaryOperator::SquareRoot),
                    "sin" => Some(UnaryOperator::Sin),
                    "cos" => Some(UnaryOperator::Cos),
                    "tan" => Some(UnaryOperator::Tan),
                    "ln" => Some(UnaryOperator::Ln),
                    _ => None,
                } {
                    self.consume_word(&word);
                    let operand = if self.is_empty() {
                        Token::LastResult
                    } else if self.current_char() == Some('-') {
                        // Handle negative operands
                        self.consume_char();

                        if let Some(ch) = self.current_char() {
                            if ch.is_ascii_digit() || ch == '.' {
                                let num = self.parse_number()?;
                                if let Token::Value(n) = num {
                                    Token::Value(-n)
                                } else {
                                    return Err("Invalid negative number".into());
                                }
                            } else {
                                return Err("Invalid negative operand".into());
                            }
                        } else {
                            return Err("Invalid negative operand".into());
                        }
                    } else {
                        self.parse_value()?
                    };
                    Token::Unary(UnaryToken {
                        operation: unary_op,
                        operand: Box::new(operand),
                    })
                } else {
                    // Parse basic value and handle postfix factorial
                    let value = self.parse_value()?;

                    // Handle factorial (postfix operator)
                    self.parse_factorial_chain(value)
                }
            } else if self.current_char() == Some('-') {
                // Handle negative numbers and subtraction with last result
                self.consume_char(); // consume '-'

                // Check if it's a negative number
                if let Some(ch) = self.current_char() {
                    if ch.is_ascii_digit() || ch == '.' {
                        let num = self.parse_number()?;
                        if let Token::Value(n) = num {
                            Token::Value(-n)
                        } else {
                            return Err("Invalid negative number".into());
                        }
                    } else {
                        // It's subtraction with last result
                        let right = self.parse_value()?;
                        Token::Binary(BinaryToken {
                            left: Box::new(Token::LastResult),
                            operation: BinaryOperator::Subtract,
                            right: Box::new(right),
                        })
                    }
                } else {
                    return Err("Unexpected end after minus sign".into());
                }
            } else {
                // Parse basic value and handle postfix factorial
                let value = self.parse_value()?;

                // Handle factorial (postfix operator)
                self.parse_factorial_chain(value)
            }
        };

        self.parse_binary_chain(left, min_precedence)
    }

    fn parse_binary_chain(
        &mut self,
        left: Token,
        min_precedence: u8,
    ) -> Result<Token, Box<dyn Error>> {
        if let Some((precedence, op)) = self.peek_binary_op() {
            if precedence >= min_precedence {
                match op {
                    BinaryOperator::Log => self.consume_word("log"),
                    _ => {
                        self.consume_char();
                    }
                }

                let right_precedence = if matches!(op, BinaryOperator::Power | BinaryOperator::Log) {
                    precedence
                } else {
                    precedence + 1
                };

                let right = self.parse_expression(right_precedence)?;

                let binary_token = Token::Binary(BinaryToken {
                    left: Box::new(left),
                    operation: op,
                    right: Box::new(right),
                });

                return self.parse_binary_chain(binary_token, min_precedence);
            }
        }

        Ok(left)
    }

    fn parse_factorial_chain(&mut self, value: Token) -> Token {
        if self.current_char() == Some('!') {
            self.consume_char();
            let factorial_token = Token::Unary(UnaryToken {
                operation: UnaryOperator::Factorial,
                operand: Box::new(value),
            });
            self.parse_factorial_chain(factorial_token)
        } else {
            value
        }
    }

    fn parse_value(&mut self) -> Result<Token, Box<dyn Error>> {
        match self.current_char() {
            Some('(') => self.parse_parentheses(),
            Some(ch) if ch.is_ascii_digit() || ch == '.' => self.parse_number(),
            Some(ch) => Err(format!("Unexpected character: {}", ch).into()),
            None => Err("Unexpected end of expression".into()),
        }
    }

    fn parse_parentheses(&mut self) -> Result<Token, Box<dyn Error>> {
        self.consume_char(); // consume '('

        let expr = self.parse_expression(0)?;
        if self.current_char() != Some(')') {
            return Err("Missing closing parenthesis".into());
        }
        self.consume_char(); // consume ')'

        Ok(expr)
    }

    fn parse_number(&mut self) -> Result<Token, Box<dyn Error>> {
        let number = self.collect_number_chars(String::new(), false)?;
        number
            .parse::<f64>()
            .map(Token::Value)
            .map_err(|_| "Invalid number format".into())
    }

    fn collect_number_chars(
        &mut self,
        mut number: String,
        has_decimal: bool,
    ) -> Result<String, Box<dyn Error>> {
        match self.current_char() {
            Some(ch) if ch.is_ascii_digit() => {
                number.push(ch);
                self.consume_char();
                self.collect_number_chars(number, has_decimal)
            }
            Some('.') if !has_decimal => {
                number.push('.');
                self.consume_char();
                self.collect_number_chars(number, true)
            }
            _ => Ok(number),
        }
    }



    fn peek_binary_op(&mut self) -> Option<(u8, BinaryOperator)> {
        // Check for word operators first
        if let Some(word) = self.peek_word() {
            if word == "log" {
                return Some((3, BinaryOperator::Log));
            }
        }

        // Check for character operators
        self.chars.peek().and_then(|&ch| match ch {
            '+' => Some((0, BinaryOperator::Add)),
            '-' => Some((0, BinaryOperator::Subtract)),
            '*' => Some((1, BinaryOperator::Multiply)),
            '/' => Some((1, BinaryOperator::Divide)),
            '%' => Some((1, BinaryOperator::Modulo)),
            '^' => Some((2, BinaryOperator::Power)),
            _ => None,
        })
    }











    // Simple helper functions
    fn current_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn consume_char(&mut self) {
        self.chars.next();
    }

    fn is_empty(&mut self) -> bool {
        self.chars.peek().is_none()
    }





    fn peek_word(&mut self) -> Option<String> {
        let word = self.collect_word_chars(String::new(), self.chars.clone());
        if word.is_empty() { None } else { Some(word) }
    }

    fn collect_word_chars(
        &self,
        mut word: String,
        mut chars_iter: std::iter::Peekable<std::vec::IntoIter<char>>,
    ) -> String {
        match chars_iter.peek() {
            Some(&ch) if ch.is_alphabetic() => {
                word.push(ch);
                chars_iter.next();
                self.collect_word_chars(word, chars_iter)
            }
            _ => word,
        }
    }

    fn consume_word(&mut self, word: &str) {
        self.consume_word_recursive(word.len());
    }

    fn consume_word_recursive(&mut self, remaining: usize) {
        if remaining > 0 {
            self.consume_char();
            self.consume_word_recursive(remaining - 1);
        }
    }
}
