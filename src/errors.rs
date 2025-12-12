use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum CalculatorError {
    InvalidCharacter(char),
    InvalidNumberFormat,

    UnexepctedToken { expected: Option<Token>, got: Token },
    MismatchedParenthesis,

    DivisionByZero,
    VariableNotDefined(String),
    EvaluationUnknownOperator,
    InvalidTokenUnary,
    TypeError(String)
}
