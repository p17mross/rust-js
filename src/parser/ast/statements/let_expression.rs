use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub(crate) struct LetExpression {
    pub location: ProgramLocation,

    pub lhs: DestructuringAssignmentTarget,
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
