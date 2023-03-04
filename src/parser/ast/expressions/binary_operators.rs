//! The [`ASTNodeBinaryOperator`] type

use std::fmt::Debug;

use crate::lexer::token::BinaryOperator;

use super::*;

/// A binary operator
#[derive(Debug)]
pub(crate) struct ASTNodeBinaryOperator {
    /// The location of the operator
    pub location: ProgramLocation,

    /// Which binary operator is being applied
    pub operator_type: BinaryOperator,
    /// The left hand side of the operator
    pub lhs: Expression,
    /// The right hand side of the operator
    pub rhs: Expression,
}

impl ToTree for ASTNodeBinaryOperator {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Binary operator {:?} at {}:{}\n",
            self.operator_type, self.location.line, self.location.column
        );
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}
