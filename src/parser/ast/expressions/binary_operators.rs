use std::fmt::Debug;

use crate::lexer::token::BinaryOperator;

use super::*;

#[derive(Debug)]
pub struct ASTNodeBinaryOperator {
    pub location: ProgramLocation,

    pub operator_type: BinaryOperator,
    pub lhs: Expression,
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
