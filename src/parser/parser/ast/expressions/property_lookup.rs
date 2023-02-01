use crate::engine::program::ProgramLocation;
use super::*;

#[derive(Debug)]
pub struct ASTNodePropertyLookup {
    pub location: ProgramLocation,

    pub lhs: ASTNodeExpression,
    pub rhs: ASTNodeExpression,

    /// Whether the property lookup is optionally chained e.g. 'a?.b'
    pub optional: bool,
}

impl ASTNodePropertyLookup {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Property lookup at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}
