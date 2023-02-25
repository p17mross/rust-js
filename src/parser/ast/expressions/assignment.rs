use crate::{
    engine::program::ProgramLocation,
    lexer::token::UpdateAssignmentOperator,
    parser::ast::{AssignmentTarget, DestructuringAssignmentTarget, StringExtTreeIndent, ToTree},
};

use super::Expression;

#[derive(Debug)]
pub(crate) struct Assignment {
    pub location: ProgramLocation,

    pub lhs: DestructuringAssignmentTarget,
    pub rhs: Expression,
}

#[derive(Debug)]
pub(crate) struct UpdateAssignment {
    pub location: ProgramLocation,

    pub operator_type: UpdateAssignmentOperator,
    pub lhs: AssignmentTarget,
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
