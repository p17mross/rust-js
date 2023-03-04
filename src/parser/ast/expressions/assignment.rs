//! The [`Assignment`] and [`UpdateAssignment`] types

use crate::{
    engine::program::ProgramLocation,
    lexer::token::UpdateAssignmentOperator,
    parser::ast::{AssignmentTarget, DestructuringAssignmentTarget, StringExtTreeIndent, ToTree},
};

use super::Expression;

/// An assignment using the [`=`][crate::lexer::TokenType::OperatorAssignment] operator
#[derive(Debug)]
pub(crate) struct Assignment {
    /// The location of the `=` token
    pub location: ProgramLocation,

    /// The target of the assignment
    pub lhs: DestructuringAssignmentTarget,
    /// The expression which is being assigned
    pub rhs: Expression,
}

/// An assignment using an [`UpdateAssignmentOperator`]
#[derive(Debug)]
pub(crate) struct UpdateAssignment {
    /// The location of the assignment operator
    pub location: ProgramLocation,

    /// The type of assignment operator
    pub operator_type: UpdateAssignmentOperator,
    /// The target of the assignment
    pub lhs: AssignmentTarget,
    /// The expression which is being assigned
    pub rhs: Expression,
}

impl ToTree for Assignment {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Assignment at {}:{}\n",
            self.location.line, self.location.column
        );
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}

impl ToTree for UpdateAssignment {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Update Assignment at {}:{}\n",
            self.location.line, self.location.column
        );
        s += &format!("|-operator type: {:?}\n", self.operator_type);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}
