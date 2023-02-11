use crate::{engine::program::ProgramLocation, parser::ast::{ASTNodeDestructuringAssignmentTarget, ASTNodeAssignmentTarget, StringExtTreeIndent}, lexer::token::UpdateAssignmentOperator};

use super::ASTNodeExpression;

#[derive(Debug)]
pub struct ASTNodeAssignment {
    pub location: ProgramLocation,

    pub lhs: ASTNodeDestructuringAssignmentTarget,
    pub rhs: ASTNodeExpression,
}

#[derive(Debug)]
pub struct ASTNodeUpdateAssignment {
    pub location: ProgramLocation,

    pub(crate) operator_type: UpdateAssignmentOperator,
    pub lhs: ASTNodeAssignmentTarget,
    pub rhs: ASTNodeExpression,
}

impl ASTNodeAssignment {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Assignment at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}

impl ASTNodeUpdateAssignment {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Update Assignment at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-operator type: {:?}\n", self.operator_type);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}