use std::f64::consts::PI;
use std::{collections::HashMap};

use crate::errors::CalculatorError;
use crate::output::Value;
use crate::{ast::AstNode, lexer::Token};

pub struct Evaluator {
    variables: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut variables: HashMap<String, Value> = HashMap::new();

        variables.insert(String::from("pi"), Value::Number(PI));

        Self { variables }
    }

    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, CalculatorError> {
        match node {
            AstNode::Number(val) => Ok(Value::Number(*val)),
            AstNode::BinaryOp { op, lhs, rhs } => {
                let left_val = self.evaluate(lhs)?;
                let right_val = self.evaluate(rhs)?;

                 let token = match op {
                    Token::Plus => Value::Number(left_val.as_number()? + right_val.as_number()?),
                    Token::Minus => Value::Number(left_val.as_number()? - right_val.as_number()?),
                    Token::Star => Value::Number(left_val.as_number()? * right_val.as_number()?),
                    Token::Slash => {
                        let r = right_val.as_number()?;
                        if r == 0.0 {
                            return Err(CalculatorError::DivisionByZero);
                        }
                        Value::Number(left_val.as_number()? / r)
                    }
                    Token::Greater => Value::Boolean(left_val.as_number()? > right_val.as_number()?),
                    Token::GreaterEq => Value::Boolean(left_val.as_number()? >= right_val.as_number()?),
                    Token::Less => Value::Boolean(left_val.as_number()? < right_val.as_number()?),
                    Token::LessEq => Value::Boolean(left_val.as_number()? <= right_val.as_number()?),
                    Token::LogicalAnd => Value::Boolean(left_val.as_bool()? && right_val.as_bool()?),
                    Token::EqComparison => Value::Boolean(left_val.as_bool()? == right_val.as_bool()?),
                    Token::Different => Value::Boolean(left_val.as_number()? != right_val.as_number()?),
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

            AstNode::UnaryExpr { op, node } => {
                let node_val = self.evaluate(node)?;

                match op {
                    &Token::Plus => Ok(node_val),
                    &Token::Minus => Ok(Value::Number(node_val.as_number()? * -1.0)),
                    _ => Err(CalculatorError::InvalidTokenUnary)
                }
            }
        }
    }
}
