use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum ASTNodeArrayItem {
    Item(ASTNodeExpression),
    Spread(ASTNodeExpression),
    Empty(ProgramLocation),
}

#[derive(Debug)]
pub struct ASTNodeArrayLiteral {
    pub location: ProgramLocation,

    pub items: Vec<ASTNodeArrayItem>
}

impl ASTNodeArrayItem {
    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Self::Item(e) | Self::Spread(e) => e.get_location(),
            Self::Empty(l) => l.clone(),
        }
    }
}

impl ASTNodeArrayItem {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Item(e) => e.to_tree(),
            Self::Spread(e) => format!("Spread from {}", e.to_tree()),
            Self::Empty(_) => "Empty Slot".to_string()
        }
    }
}

impl ASTNodeArrayLiteral {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Array Literal at {}:{}", self.location.line, self.location.column);

        for (i, expression) in self.items.iter().enumerate() {
            s += &format!("|-{i}: {}", expression.to_tree().indent_tree())
        }

        s
    }
}