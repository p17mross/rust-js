//! The [`ASTNodeUnaryOperator`] and related types

use crate::engine::program::ProgramLocation;

use std::fmt::Debug;

use super::*;

/// A type of unary operator
#[derive(Debug)]
pub(crate) enum UnaryOperator {
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

/// A unary operator being applied to a value
#[derive(Debug)]
pub(crate) struct ASTNodeUnaryOperator {
    /// The location of the unary operator
    pub location: ProgramLocation,
    /// The type of operator
    pub operator_type: UnaryOperator,
    /// The expression to which the operator is being applied
    pub expression: Expression,
}

impl ToTree for ASTNodeUnaryOperator {
    fn to_tree(&self) -> String {
        format!(
            "Unary operator ({:?}) at {}:{}: {}",
            self.operator_type,
            self.location.line,
            self.location.column,
            self.expression.to_tree()
        )
    }
}
