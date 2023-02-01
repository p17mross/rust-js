use std::collections::HashMap;

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub struct ASTNodeObjectLiteral {
    pub location: ProgramLocation,

    pub properties: HashMap<String, ASTNodeExpression>
}

impl ASTNodeObjectLiteral {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Object Literal at {}:{}", self.location.line, self.location.column);

        for (key, expression) in &self.properties {
            s += &format!("|-\"{key}\": {}", expression.to_tree().indent_tree());
        }

        s
    }
}