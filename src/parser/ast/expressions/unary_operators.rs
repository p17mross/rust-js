use crate::engine::program::ProgramLocation;

use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Await,
    TypeOf,
    Void,
}

#[derive(Debug)]
pub struct ASTNodeUnaryOperator {
    pub location: ProgramLocation,

    pub operator_type: UnaryOperator,
    pub expression: ASTNodeExpression,
}


impl ASTNodeUnaryOperator {
    pub fn to_tree(&self) -> String {
        format!("Unary operator ({:?}) at {}:{}: {}", self.operator_type, self.location.line, self.location.column, self.expression.to_tree())
    }
}