use core::panic;

use crate::lexer::Token;
use crate::ast::AstNode;

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

    fn consume_token(&mut self, expected: Token) {
        let token = self.tokens[self.current_token].clone();
        if token != expected {
            panic!("Expected token {:?}, but got {:?}", expected, token);
        }
        self.next_token();
    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens[self.current_token].clone();
        self.current_token += 1;
        token
    }

    fn parse_factor(&mut self) -> AstNode {
        match self.next_token() {
            Token::Number(val) => AstNode::Number(val),
            Token::LParen => {
                let expr = self.parse_expression();

                self.consume_token(Token::RParen);
                
                expr
            },
            Token::Minus => {
                let mut expr = self.parse_factor();

                match &mut expr {
                    AstNode::Number(value) => {
                        *value = -*value;
                    }
                    _ => {}
                }

                expr
            },
            Token::Identifier(identifier) => AstNode::ReadIdentifier(identifier),
            t => panic!("Unexpected token in parse_factor {:?}", t),
        }
    }

    fn parse_term(&mut self) -> AstNode {
        let mut node = self.parse_factor();

        while let Token::Star | Token::Slash = self.peek_token() {
            let op = self.next_token();
            let right = self.parse_factor();

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(right)
            }
        }

        node
    }

    pub fn parse_expression(&mut self) -> AstNode {
        let mut node = self.parse_term();

        while let Token::Plus | Token::Minus = self.peek_token() {
            let op = self.next_token();
            let rhs = self.parse_term();

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs)
            }
        }

        node
    }

    pub fn parse_input(&mut self) -> AstNode {
        let node: Option<AstNode> = None;

        if self.tokens.len() == 2 {
            return match self.next_token() {
                Token::Identifier(id) => AstNode::ReadIdentifier(id),
                _ => panic!("Incomplete statement")
            }
        }

        if let Token::Identifier(..) = self.peek_token() {
            return match self.next_token() {
                Token::Identifier(id) => {
                    match self.peek_token() {
                        Token::Eq => {
                            self.consume_token(Token::Eq);
                            let node = self.parse_expression();

                            AstNode::AssignIdentifier {
                                name: id,
                                node_value: Box::new(node)
                            }
                        },
                        Token::Plus | Token::Minus | Token::Slash | Token::Star => {
                            let op = self.next_token();
                            let node = self.parse_expression();

                            AstNode::BinaryOp {
                                op,
                                lhs: Box::new(AstNode::ReadIdentifier(id)),
                                rhs: Box::new(node)
                            }
                        },
                        _ => panic!("Ureachable"),
                    }
                }
                _ => panic!("Invalid")
            }
        }

        match node {
            Some(n) => n,
            None => self.parse_expression()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer};

    #[test]
    fn test_parse() {
        let operation = String::from("4 + a");

        let mut lexer = Lexer::new(&operation);
        let tokens = lexer.tokenize();
        println!("{:#?}", tokens);
    }
}