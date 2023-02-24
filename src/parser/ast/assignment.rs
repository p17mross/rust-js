use std::collections::HashMap;

use super::{Expression, StringExtTreeIndent, ToTree};

#[derive(Debug)]
pub enum AssignmentTarget {
    Variable(String),
    PropertyLookup{
        expression: Expression,
        property: Expression
    }
}

#[derive(Debug)]
pub struct DestructureBinding {
    destructure: DestructuringAssignmentTarget,
    default_value: Option<Expression>,
}

#[derive(Debug)]
pub enum DestructuringAssignmentTarget {
    Variable(String),
    PropertyLookup {
        expression: Expression,
        property: Expression
    },
    ArrayDestructure {
        items: Vec<Option<DestructureBinding>>, 
        rest: Option<Box<DestructuringAssignmentTarget>>,
    },
    ObjectDestructure (HashMap<String, DestructureBinding>)
}

impl ToTree for AssignmentTarget {
    fn to_tree(&self) -> String {
        match self {
            Self::Variable(v) => format!("Variable '{v}'"),
            Self::PropertyLookup { expression, property } => format!(
                "Property Lookup\n|-expression: {}\n|-property: {}", 
                expression.to_tree().indent_tree(), 
                property.to_tree().indent_tree()
            ),
        }
    }
}

impl ToTree for DestructureBinding {
    fn to_tree(&self) -> String {
        let mut s = "Destructuring binding\n".to_string();
        s += &format!("|-destructure: {}\n", self.destructure.to_tree().indent_tree());
        
        if let Some(d) = &self.default_value {
            s += &format!("|-default value: {}", d.to_tree().indent_tree());
        }
        
        s
    }
}

impl ToTree for DestructuringAssignmentTarget{
   fn to_tree(&self) -> String {
        match self {
            Self::Variable(v) => format!("Variable '{v}'"),
            Self::PropertyLookup { expression, property } => format!(
                "Property Lookup\n|-expression: {}\n|-property: {}", 
                expression.to_tree().indent_tree(), 
                property.to_tree().indent_tree()
            ),
            Self::ArrayDestructure { items, rest: spread } => {
                let mut s = "Array Destructure\n".to_string();
                for (i, item) in items.iter().enumerate() {
                    if let Some(item) = item {
                        s += &format!("|-{i}: {}", item.to_tree().indent_tree());
                    } else {
                        s += &format!("|-{i}: Empty Slot");
                    }
                }

                if let Some(spread) = spread {
                    s += &format!("|-Rest: {}", spread.to_tree().indent_tree());
                }

                s
            }
            Self::ObjectDestructure(o) => {
                let mut s = "Object Destructure\n".to_string();

                for (k, v) in o {
                    s += &format!("|-'{k}': {}", v.to_tree().indent_tree());
                }

                s
            }
        }
    }
}