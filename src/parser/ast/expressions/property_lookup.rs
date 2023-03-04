//! The [`PropertyLookup`] type

use super::*;
use crate::engine::program::ProgramLocation;

/// A property lookup. This type can be represented by many different syntaxes:
/// * `object.property`
/// * `object["property"]`
/// * `object?.property`
/// * `object?.["property"]`
#[derive(Debug)]
pub(crate) struct PropertyLookup {
    /// The location of the property lookup
    pub location: ProgramLocation,

    /// The object on which the lookup is occurring
    pub lhs: Expression,
    /// The property being looked up
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
