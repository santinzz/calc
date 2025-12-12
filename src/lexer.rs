use std::path::MAIN_SEPARATOR;

use crate::errors::CalculatorError;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Eof,
    RParen,
    LParen,
    Identifier(String),
    Eq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    EqComparison,
    Different,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BinaryAnd,
    BinaryOr,
    BinaryNot,
}

pub struct Lexer<'a> {
    input: &'a String,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char();
        if c.is_some() {
            self.current_pos += 1;
        }
        c
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek_char().map_or(false, |c| c.is_whitespace()) {
            self.next_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, CalculatorError> {
        self.skip_whitespace();

        let c = match self.next_char() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        let token = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '=' => {
                if self.peek_char().is_some() && self.peek_char().unwrap() == '=' {
                    self.next_char();
                    Token::EqComparison
                } else {
                    Token::Eq
                }
            },
            '>' => {
                if self.peek_char().is_some() && self.peek_char().unwrap() == '=' {
                    self.next_char();
                    Token::GreaterEq
                } else {
                    Token::Greater
                }
            },

            '<' => {
                if self.peek_char().is_some() && self.peek_char().unwrap() == '=' {
                    self.next_char();
                    Token::LessEq
                } else {
                    Token::Less
                }
            }

            '!' => {
                if self.peek_char().is_some() && self.peek_char().unwrap() == '=' {
                    self.next_char();
                    Token::Different
                } else {
                    Token::LogicalNot
                }
            }

            '&' => {
                if self.peek_char().is_some() && self.peek_char().unwrap() == '&' {
                    self.next_char();
                    Token::LogicalAnd
                } else {
                    Token::BinaryAnd
                }
            }

            '|' => {
                if self.peek_char().is_some() && self.peek_char().unwrap() == '|' {
                    self.next_char();
                    Token::LogicalOr
                } else {
                    Token::BinaryOr
                }
            }
            '0'..='9' | '.' => self.lex_number(c)?,
            c => self.identifier(c)?,
        };

        Ok(token)
    }

    pub fn identifier(&mut self, initial_char: char) -> Result<Token, CalculatorError> {
        if !initial_char.is_alphanumeric() && initial_char != '_' {
            return Err(CalculatorError::InvalidCharacter(initial_char));
        }

        let mut identifier_str = String::from(initial_char);

        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier_str.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        Ok(Token::Identifier(identifier_str))
    }

    pub fn lex_number(&mut self, inital_char: char) -> Result<Token, CalculatorError> {
        let mut num_str = String::from(inital_char);

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        match num_str.parse::<f64>() {
            Ok(val) => Ok(Token::Number(val)),
            Err(_) => Err(CalculatorError::InvalidNumberFormat),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CalculatorError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;

            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    #[test]
    fn test_boolean_operators_tokenizing() {
        let operation = String::from("3 == 3");

        let mut lexer = Lexer::new(&operation);

        println!("Tokens genrated: {:#?}", lexer.tokenize().unwrap());
    }
}
