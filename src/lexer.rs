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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = match self.next_char() {
            Some(c) => c,
            None => return Token::Eof,
        };

        match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '=' => Token::Eq,
            '0'..='9' | '.' => self.lex_number(c),
            c => self.identifier(c),
        }
    }

    pub fn identifier(&mut self, initial_char: char) -> Token {
        if !initial_char.is_alphanumeric() && initial_char != '_' {
            panic!("Invalid character encountered");
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

        Token::Identifier(identifier_str)
    }

    pub fn lex_number(&mut self, inital_char: char) -> Token {
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
            Ok(val) => Token::Number(val),
            Err(_) => panic!("Invalid number format: {}", num_str),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();

            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod test {
    use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

    #[test]
    fn test_identifier_tokenizing() {
        let mut eval = Evaluator::new();

        {
            let input = String::from("ast = 5");
            let mut lexer = Lexer::new(&input);
            let mut parser = Parser::new(lexer.tokenize());

            let result = eval.evaluate(&parser.parse_input());

            println!("{}", result)
        }

        {
            let input = String::from("5 + ast");
            let mut lexer = Lexer::new(&input);
            let mut parser = Parser::new(lexer.tokenize());

            let result = eval.evaluate(&parser.parse_input());

            println!("{}", result)
        }
    }
}
