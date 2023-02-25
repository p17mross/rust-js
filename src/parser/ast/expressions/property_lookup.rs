use super::*;
use crate::engine::program::ProgramLocation;

#[derive(Debug)]
pub(crate) struct PropertyLookup {
    pub location: ProgramLocation,

    pub lhs: Expression,
    pub rhs: Expression,

    /// Whether the property lookup is optionally chained e.g. 'a?.b'
    pub optional: bool,
}

impl ToTree for PropertyLookup {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Property lookup at {}:{}\n",
            self.location.line, self.location.column
        );
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}
