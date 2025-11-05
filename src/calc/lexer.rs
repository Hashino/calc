use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub(super) enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Power,
    Modulo,
    Factorial,
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
    Pi,
    E,
}

pub(super) struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub(super) fn tokenize(input: &'a str) -> Result<Vec<Token>, String> {
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = lexer.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    pub(super) fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        match self.chars.next() {
            Some(ch) => match ch {
                // skip whitespace
                ' ' => self.next_token(),

                // operators and parentheses
                '+' => Ok(Some(Token::Plus)),
                '-' => Ok(Some(Token::Minus)),
                '*' => Ok(Some(Token::Multiply)),
                '/' => Ok(Some(Token::Divide)),
                '(' => Ok(Some(Token::LParen)),
                ')' => Ok(Some(Token::RParen)),
                '^' => Ok(Some(Token::Power)),
                '%' => Ok(Some(Token::Modulo)),
                '!' => Ok(Some(Token::Factorial)),

                // numbers
                '0'..='9' | '.' => {
                    let mut number_str = String::from(ch);

                    while let Some(ch) = self.chars.peek() {
                        match ch {
                            '0'..='9' | '.' => {
                                number_str.push(self.chars.next().unwrap());
                            }
                            _ => break,
                        }
                    }

                    let value = number_str
                        .parse::<f64>()
                        .map_err(|_| format!("Invalid number: {}", number_str))?;

                    Ok(Some(Token::Number(value)))
                }

                // identifiers (functions and constants)
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::from(ch);

                    while let Some(ch) = self.chars.peek()
                        && ch.is_alphanumeric()
                    {
                        identifier.push(self.chars.next().unwrap());
                    }

                    let token = match identifier.as_str() {
                        "sqrt" => Token::Sqrt,
                        "sin" => Token::Sin,
                        "cos" => Token::Cos,
                        "tan" => Token::Tan,
                        "ln" => Token::Ln,
                        "floor" => Token::Floor,
                        "ceil" => Token::Ceil,
                        "abs" => Token::Abs,
                        "round" => Token::Round,
                        "log" => Token::Log,
                        "pi" => Token::Pi,
                        "e" => Token::E,
                        _ => {
                            return Err(format!("Unknown identifier: {}", identifier));
                        }
                    };

                    Ok(Some(token))
                }

                _ => Err(format!("Unexpected character: '{}'", ch)),
            },

            // EOF
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operators() {
        let tokens = Lexer::tokenize("+ - * /").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Plus, Token::Minus, Token::Multiply, Token::Divide,]
        );
    }

    #[test]
    fn test_numbers() {
        let tokens = Lexer::tokenize("123 45.67 0.5 .25").unwrap();

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
        let tokens = Lexer::tokenize("sqrt sin cos tan ln").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Sqrt, Token::Sin, Token::Cos, Token::Tan, Token::Ln,]
        );
    }

    #[test]
    fn test_constants() {
        let tokens = Lexer::tokenize("pi e").unwrap();

        assert_eq!(tokens, vec![Token::Pi, Token::E]);
    }

    #[test]
    fn test_complex_expression() {
        let tokens = Lexer::tokenize("sin(pi / 2) + sqrt(4)").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Sin,
                Token::LParen,
                Token::Pi,
                Token::Divide,
                Token::Number(2.0),
                Token::RParen,
                Token::Plus,
                Token::Sqrt,
                Token::LParen,
                Token::Number(4.0),
                Token::RParen,
            ]
        );
    }
}
