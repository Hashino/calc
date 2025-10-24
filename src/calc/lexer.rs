use std::{error::Error, fmt, iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
    Factorial,
    LParen,
    RParen,
    Function(FunctionType),
    Constant(ConstantType),
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum FunctionType {
    Sqrt,
    Sin,
    Cos,
    Tan,
    Ln,
    Floor,
    Ceil,
    Abs,
    Round,
    Log,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum ConstantType {
    Pi,
    E,
}

#[derive(Debug)]
pub(super) struct LexError {
    message: String,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexer error: {}", self.message)
    }
}

impl Error for LexError {}

pub(super) struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub(super) fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexError> {
        match self.chars.next() {
            Some(ch) => match ch {
                // skip whitespace
                ' ' => self.next_token(),

                // operators and parentheses
                '+' => Ok(Some(Token::Plus)),
                '-' => Ok(Some(Token::Minus)),
                '*' => Ok(Some(Token::Multiply)),
                '/' => Ok(Some(Token::Divide)),
                '^' => Ok(Some(Token::Power)),
                '%' => Ok(Some(Token::Modulo)),
                '!' => Ok(Some(Token::Factorial)),
                '(' => Ok(Some(Token::LParen)),
                ')' => Ok(Some(Token::RParen)),

                // numbers
                '0'..='9' | '.' => {
                    let mut number_str = String::from(ch);

                    while let Some(ch) = self.chars.peek() {
                        match ch {
                            '0'..='9' | '.' => {
                                number_str.push(ch.to_owned());
                                self.chars.next();
                            }
                            _ => break,
                        }
                    }

                    let value = number_str.parse::<f64>().map_err(|_| LexError {
                        message: format!("Invalid number: {}", number_str),
                    })?;

                    Ok(Some(Token::Number(value)))
                }

                // identifiers (functions and constants)
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::from(ch);

                    while let Some(ch) = self.chars.peek()
                        && ch.is_alphanumeric()
                    {
                        identifier.push(ch.to_owned());
                        self.chars.next();
                    }

                    let token = match identifier.as_str() {
                        "sqrt" => Token::Function(FunctionType::Sqrt),
                        "sin" => Token::Function(FunctionType::Sin),
                        "cos" => Token::Function(FunctionType::Cos),
                        "tan" => Token::Function(FunctionType::Tan),
                        "ln" => Token::Function(FunctionType::Ln),
                        "floor" => Token::Function(FunctionType::Floor),
                        "ceil" => Token::Function(FunctionType::Ceil),
                        "abs" => Token::Function(FunctionType::Abs),
                        "round" => Token::Function(FunctionType::Round),
                        "log" => Token::Function(FunctionType::Log),
                        "pi" => Token::Constant(ConstantType::Pi),
                        "e" => Token::Constant(ConstantType::E),
                        _ => {
                            return Err(LexError {
                                message: format!("Unknown identifier: {}", identifier),
                            });
                        }
                    };

                    Ok(Some(token))
                }

                _ => Err(LexError {
                    message: format!("Unexpected character: '{}'", ch),
                }),
            },
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operators() {
        let mut lexer = Lexer::new("+ - * / ^ % !");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Multiply,
                Token::Divide,
                Token::Power,
                Token::Modulo,
                Token::Factorial,
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("123 45.67 0.5 .25");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(123.0),
                Token::Number(45.67),
                Token::Number(0.5),
                Token::Number(0.25),
            ]
        );
    }

    #[test]
    fn test_functions() {
        let mut lexer = Lexer::new("sqrt sin cos tan ln");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Function(FunctionType::Sqrt),
                Token::Function(FunctionType::Sin),
                Token::Function(FunctionType::Cos),
                Token::Function(FunctionType::Tan),
                Token::Function(FunctionType::Ln),
            ]
        );
    }

    #[test]
    fn test_constants() {
        let mut lexer = Lexer::new("pi e");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Constant(ConstantType::Pi),
                Token::Constant(ConstantType::E),
            ]
        );
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("sin(pi / 2) + sqrt(4)");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Function(FunctionType::Sin),
                Token::LParen,
                Token::Constant(ConstantType::Pi),
                Token::Divide,
                Token::Number(2.0),
                Token::RParen,
                Token::Plus,
                Token::Function(FunctionType::Sqrt),
                Token::LParen,
                Token::Number(4.0),
                Token::RParen,
            ]
        );
    }
}
