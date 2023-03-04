//! The [`LetExpression`] type

use crate::engine::program::ProgramLocation;

use super::*;

/// A let expression scoped to the containing block
#[derive(Debug)]
pub(crate) struct LetExpression {
    /// The location of the let expression
    pub location: ProgramLocation,

    /// The target of the assignment
    pub lhs: DestructuringAssignmentTarget,
    /// The expression being assigned
    pub rhs: Expression,
}

impl ToTree for LetExpression {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Let expression at {}:{}\n",
            self.location.line, self.location.column
        );
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}
