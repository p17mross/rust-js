use crate::{engine::program::ProgramLocation};

use super::*;

#[derive(Debug)]
pub struct ASTNodeLetExpression {
    pub location: ProgramLocation,

    pub lhs: ASTNodeDestructuringAssignmentTarget,
    pub rhs: ASTNodeExpression,
}


impl ASTNodeLetExpression {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Let expression at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}