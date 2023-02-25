use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub(crate) enum ArrayItem {
    Item(Expression),
    Spread(Expression),
    Empty(ProgramLocation),
}

#[derive(Debug)]
pub(crate) struct ArrayLiteral {
    pub location: ProgramLocation,

    pub items: Vec<ArrayItem>,
}

impl ArrayItem {
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
