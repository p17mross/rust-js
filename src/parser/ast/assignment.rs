//! The [`AssignmentTarget`] and [`DestructuringAssignmentTarget`] types, which are needed by both [`Assignment`][super::expressions::Assignment] and [`LetExpression`][super::statements::LetExpression]

use std::collections::HashMap;

use super::{Expression, StringExtTreeIndent, ToTree};

/// An assignment target with no destructuring
#[derive(Debug)]
pub(crate) enum AssignmentTarget {
    /// Assigning to a variable
    Variable(String),
    /// Assigning to a property of an object
    PropertyLookup {
        /// The object
        expression: Expression,
        /// The property
        property: Expression,
    },
}

/// One item of a [`DestructuringAssignmentTarget`], e.g. the `a = 10` in `let [a = 10, b] = [10, 20]`
#[derive(Debug)]
pub(crate) struct DestructureBinding {
    /// The binding
    destructure: DestructuringAssignmentTarget,
    /// An optional default value for if the binding is not matched
    default_value: Option<Expression>,
}

/// An assignment target which can be destructuring
#[derive(Debug)]
pub(crate) enum DestructuringAssignmentTarget {
    /// The base case of the recursive data structure - just assigning to a variable
    Variable(String),
    /// Assigning to a property of an object
    PropertyLookup {
        /// The object
        expression: Expression,
        /// The property
        property: Expression,
    },
    /// An array destructure e.g. `let [a, b] = [10, 20]`
    ArrayDestructure {
        /// The items in the destructure
        items: Vec<Option<DestructureBinding>>,
        /// The optional 'rest' argument e.g. `let [a, b, ...c] = [1, 2, 3, 4, 5]`
        rest: Option<Box<DestructuringAssignmentTarget>>,
    },
    /// An object destructure e.g. `let {a, b} = {a: 10, b: 20}`
    ObjectDestructure(HashMap<String, DestructureBinding>),
}

impl ToTree for AssignmentTarget {
    fn to_tree(&self) -> String {
        match self {
            Self::Variable(v) => format!("Variable '{v}'"),
            Self::PropertyLookup {
                expression,
                property,
            } => format!(
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
        s += &format!(
            "|-destructure: {}\n",
            self.destructure.to_tree().indent_tree()
        );

        if let Some(d) = &self.default_value {
            s += &format!("|-default value: {}", d.to_tree().indent_tree());
        }

        s
    }
}

impl ToTree for DestructuringAssignmentTarget {
    fn to_tree(&self) -> String {
        match self {
            Self::Variable(v) => format!("Variable '{v}'"),
            Self::PropertyLookup {
                expression,
                property,
            } => format!(
                "Property Lookup\n|-expression: {}\n|-property: {}",
                expression.to_tree().indent_tree(),
                property.to_tree().indent_tree()
            ),
            Self::ArrayDestructure {
                items,
                rest: spread,
            } => {
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
