use core::panic;

use crate::ast::AstNode;
use crate::errors::CalculatorError;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: 0,
        }
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.current_token]
    }

    fn consume_token(&mut self, expected: Token) -> Result<(), CalculatorError> {
        let token = self.tokens[self.current_token].clone();
        if token != expected {
            return Err(CalculatorError::UnexepctedToken {
                expected: Some(expected),
                got: token,
            });
        }
        self.next_token();
        Ok(())
    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens[self.current_token].clone();
        self.current_token += 1;
        token
    }

    fn parse_factor(&mut self) -> Result<AstNode, CalculatorError> {
        match self.next_token() {
            Token::Number(val) => Ok(AstNode::Number(val)),
            Token::LParen => {
                let expr = self.parse_expression()?;

                self.consume_token(Token::RParen);

                Ok(expr)
            }
            Token::Minus => {
                let mut expr = self.parse_factor()?;

                match &mut expr {
                    AstNode::Number(value) => {
                        *value = -*value;
                    }
                    _ => {}
                }

                Ok(expr)
            }
            Token::Identifier(identifier) => Ok(AstNode::ReadIdentifier(identifier)),
            t => Err(CalculatorError::UnexepctedToken {
                expected: None,
                got: t,
            }),
        }
    }

    fn parse_term(&mut self) -> Result<AstNode, CalculatorError> {
        let mut node = self.parse_factor()?;

        while let Token::Star | Token::Slash = self.peek_token() {
            let op = self.next_token();
            let right = self.parse_factor()?;

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(right),
            }
        }

        Ok(node)
    }

    pub fn parse_expression(&mut self) -> Result<AstNode, CalculatorError> {
        let mut node = self.parse_term()?;

        while let Token::Plus | Token::Minus = self.peek_token() {
            let op = self.next_token();
            let rhs = self.parse_term()?;

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs),
            }
        }

        Ok(node)
    }

    pub fn parse_input(&mut self) -> Result<AstNode, CalculatorError> {
        let node: Option<AstNode> = None;

        if self.tokens.len() == 2 {
            return match self.next_token() {
                Token::Identifier(id) => Ok(AstNode::ReadIdentifier(id)),
                _ => panic!("Incomplete statement"),
            };
        }

        if let Token::Identifier(..) = self.peek_token() {
            return match self.next_token() {
                Token::Identifier(id) => match self.peek_token() {
                    Token::Eq => {
                        self.consume_token(Token::Eq);
                        let node = self.parse_expression()?;

                        Ok(AstNode::AssignIdentifier {
                            name: id,
                            node_value: Box::new(node),
                        })
                    }
                    Token::Plus | Token::Minus | Token::Slash | Token::Star => {
                        let op = self.next_token();
                        let node = self.parse_expression()?;

                        Ok(AstNode::BinaryOp {
                            op,
                            lhs: Box::new(AstNode::ReadIdentifier(id)),
                            rhs: Box::new(node),
                        })
                    }
                    _ => panic!("Ureachable"),
                },
                _ => panic!("Invalid"),
            };
        }

        match node {
            Some(n) => Ok(n),
            None => self.parse_expression(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    #[test]
    fn test_parse() {
        let operation = String::from("4 + a");

        let mut lexer = Lexer::new(&operation);
        let tokens = lexer.tokenize();
        println!("{:#?}", tokens);
    }
}
