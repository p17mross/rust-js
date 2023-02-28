use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub(crate) enum ObjectLiteralProperty {
    KeyValue(String, Expression),
    KeyOnly(String),
    Computed(Expression, Expression),
    Spread(Expression),
}

#[derive(Debug)]
pub(crate) struct ObjectLiteral {
    pub location: ProgramLocation,

    pub properties: Vec<ObjectLiteralProperty>,
    
    // TODO: getters and setters
}

impl ToTree for ObjectLiteralProperty {
    fn to_tree(&self) -> String {
        match self {
            Self::KeyOnly(k) => format!("\"{k}\""),
            Self::KeyValue(k, v) => format!("\"{k}\": {}", v.to_tree().indent_tree()),
            Self::Computed(k, v) => format!(
                "Computed property:\n|-lhs: {}\n|-rhs: {}",
                k.to_tree().indent_tree(),
                v.to_tree().indent_tree()
            ),
            Self::Spread(e) => format!("Spread from {}", e.to_tree()),
        }
    }
}

impl ToTree for ObjectLiteral {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "Object Literal at {}:{}",
            self.location.line, self.location.column
        );

        for property in &self.properties {
            s += &format!("\n|-{}", property.to_tree().indent_tree());
        }

        s
    }
}
