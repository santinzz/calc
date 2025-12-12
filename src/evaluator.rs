use core::panic;
use std::{collections::HashMap, f64::consts};

use crate::errors::CalculatorError;
use crate::{ast::AstNode, lexer::Token};

pub struct Evaluator {
    variables: HashMap<String, f64>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut variables = HashMap::new();

        variables.insert(String::from("pi"), consts::PI);

        Self { variables }
    }

    pub fn evaluate(&mut self, node: &AstNode) -> Result<f64, CalculatorError> {
        match node {
            AstNode::Number(val) => Ok(*val),
            AstNode::BinaryOp { op, lhs, rhs } => {
                let left_val = self.evaluate(lhs)?;
                let right_val = self.evaluate(rhs)?;

                let token = match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => {
                        if right_val == 0.0 {
                            return Err(CalculatorError::DivisionByZero);
                        }
                        left_val / right_val
                    }
                    _ => return Err(CalculatorError::EvaluationUnknownOperator),
                };

                Ok(token)
            }

            AstNode::AssignIdentifier { name, node_value } => {
                let value = self.evaluate(&node_value)?;
                self.variables.insert(name.clone(), value);

                Ok(value)
            }

            AstNode::ReadIdentifier(identifier) => {
                let value = self.variables.get(identifier);

                match value {
                    Some(val) => Ok(*val),
                    None => Err(CalculatorError::VariableNotDefined(identifier.clone())),
                }
            }
        }
    }
}
