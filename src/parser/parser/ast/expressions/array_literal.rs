use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum ASTNodeArrayItem {
    Item(ASTNodeExpression),
    Spread(ASTNodeExpression),
}

#[derive(Debug)]
pub struct ASTNodeArrayLiteral {
    pub location: ProgramLocation,

    pub items: Vec<ASTNodeArrayItem>
}

impl ASTNodeArrayItem {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Item(e) => e.to_tree(),
            Self::Spread(e) => format!("Spread from {}", e.to_tree()),
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