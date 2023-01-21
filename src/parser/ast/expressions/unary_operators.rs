use crate::engine::program::ProgramLocation;

use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub enum UnaryOperator {
    /// A '+'
    Plus,
    /// A '-'
    Minus,
    /// A logical not ('!')
    LogicalNot,
    /// A bitwise not ('~')
    BitwiseNot,
    /// 'await'
    Await,
    /// 'typeof'
    TypeOf,
    /// 'void'
    Void,
    /// 'delete' - which can actually take any expression, not just a property lookup
    Delete,
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