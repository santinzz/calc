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

                self.consume_token(Token::RParen)?;

                Ok(expr)
            }
            Token::Plus => {
                let expr = self.parse_factor()?;

                Ok(AstNode::UnaryExpr { op: Token::Plus, node: Box::new(expr) })
            }
            Token::Minus => {
                let expr = self.parse_factor()?;

                Ok(AstNode::UnaryExpr {
                    op: Token::Minus,
                    node: Box::new(expr),
                })
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

    pub fn parse_comparison(&mut self) -> Result<AstNode, CalculatorError> {
        let mut node = self.parse_expression()?;

        while let Token::Greater | Token::GreaterEq | Token::Less | Token::LessEq = self.peek_token() {
            let op = self.next_token();
            let rhs = self.parse_expression()?;

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs)
            }
        }

        Ok(node)
    }

    pub fn parse_equalities(&mut self) -> Result<AstNode, CalculatorError> {
        let mut node = self.parse_comparison()?;

        while let Token::EqComparison | Token::Different = self.peek_token() {
            let op = self.next_token();
            let rhs = self.parse_comparison()?;

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs)
            }
        }

        Ok(node)
    }

    pub fn parse_logical_and(&mut self) -> Result<AstNode, CalculatorError> {
        let mut node = self.parse_equalities()?;

        while let Token::LogicalAnd = self.peek_token() {
            let op = self.next_token();
            let rhs = self.parse_equalities()?;

            node = AstNode::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs)
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
                        self.consume_token(Token::Eq)?;
                        let node = self.parse_logical_and()?;

                        Ok(AstNode::AssignIdentifier {
                            name: id,
                            node_value: Box::new(node),
                        })
                    }
                    _ => {
                        let op = self.next_token();
                        let node = self.parse_logical_and()?;

                        Ok(AstNode::BinaryOp {
                            op,
                            lhs: Box::new(AstNode::ReadIdentifier(id)),
                            rhs: Box::new(node),
                        })
                    }
                },
                _ => panic!("Invalid"),
            };
        }

        match node {
            Some(n) => Ok(n),
            None => self.parse_logical_and(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_parse() {
        let operation = String::from("3 == 3");

        let mut lexer = Lexer::new(&operation);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        println!("Ast: {:#?}", parser.parse_input().unwrap())
        
    }
}
