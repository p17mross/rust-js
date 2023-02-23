use crate::{engine::program::ProgramLocation, parser::ast::{DestructuringAssignmentTarget, AssignmentTarget, StringExtTreeIndent}, lexer::token::UpdateAssignmentOperator};

use super::Expression;

#[derive(Debug)]
pub struct Assignment {
    pub location: ProgramLocation,

    pub lhs: DestructuringAssignmentTarget,
    pub rhs: Expression,
}

#[derive(Debug)]
pub struct UpdateAssignment {
    pub location: ProgramLocation,

    pub(crate) operator_type: UpdateAssignmentOperator,
    pub lhs: AssignmentTarget,
    pub rhs: Expression,
}

impl Assignment {
    #[must_use]
    pub fn to_tree(&self) -> String {
        let mut s = format!("Assignment at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}

impl UpdateAssignment {
    #[must_use]
    pub fn to_tree(&self) -> String {
        let mut s = format!("Update Assignment at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-operator type: {:?}\n", self.operator_type);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}