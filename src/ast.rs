use crate::lexer::Token;

#[derive(Debug)]
pub enum AstNode {
    Number(f64),

    UnaryExpr {
        op: Token,
        node: Box<AstNode>
    },

    BinaryOp {
        op: Token,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>
    },

    AssignIdentifier {
        name: String,
        node_value: Box<AstNode>,
    },

    ReadIdentifier(String),
}