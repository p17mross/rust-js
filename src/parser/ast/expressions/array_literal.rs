//! The [`ArrayLiteral`] and related types

use crate::engine::program::ProgramLocation;

use super::*;

/// An item in an array literal
#[derive(Debug)]
pub(crate) enum ArrayItem {
    /// A normal item e.g. `[10]`
    Item(Expression),
    /// A spread from some other array e.g. `[...[10, 20]]`
    Spread(Expression),
    /// An empty slot e.g. `[,]`
    Empty(ProgramLocation),
}

/// An array literal
#[derive(Debug)]
pub(crate) struct ArrayLiteral {
    /// The location of the start of the literal
    pub location: ProgramLocation,
    /// The items contained in the literal
    pub items: Vec<ArrayItem>,
}

impl ArrayItem {
    /// Get the location of an array item
    #[must_use]
    pub(crate) fn get_location(&self) -> ProgramLocation {
        match self {
            Self::Item(e) | Self::Spread(e) => e.get_location(),
            Self::Empty(l) => l.clone(),
        }
    }
}

impl ToTree for ArrayItem {
    fn to_tree(&self) -> String {
        match self {
            Self::Item(e) => e.to_tree(),
            Self::Spread(e) => format!("Spread from {}", e.to_tree()),
            Self::Empty(_) => "Empty Slot".to_string(),
        }
    }
}

impl ToTree for ArrayLiteral {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Array Literal at {}:{}",
            self.location.line, self.location.column
        );

        for (i, expression) in self.items.iter().enumerate() {
            s += &format!("\n|-{i}: {}", expression.to_tree().indent_tree());
        }

        s
    }
}
