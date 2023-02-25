use crate::{
    engine::program::ProgramLocation,
    parser::ast::{StringExtTreeIndent, ToTree},
};

use super::{Expression, PropertyLookup, Variable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateExpressionOperatorType {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateExpressionSide {
    Prefix,
    Postfix,
}

#[derive(Debug)]
pub enum UpdateExpressionTarget {
    Variable(Box<Variable>),
    Property(Box<PropertyLookup>),
}

#[derive(Debug)]
pub struct UpdateExpression {
    pub location: ProgramLocation,

    pub target: UpdateExpressionTarget,
    pub operator_type: UpdateExpressionOperatorType,
    pub side: UpdateExpressionSide,
}

impl TryFrom<Expression> for UpdateExpressionTarget {
    type Error = ();

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::PropertyLookup(p) => Ok(Self::Property(p)),
            Expression::Variable(v) => Ok(Self::Variable(v)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for UpdateExpressionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Prefix => "prefix",
            Self::Postfix => "postfix",
        })
    }
}

impl ToTree for UpdateExpression {
    fn to_tree(&self) -> String {
        format!(
            "{:?} {:?} at {}:{}\n|-property: {}",
            self.side,
            self.operator_type,
            self.location.line,
            self.location.column,
            self.target.to_tree().indent_tree()
        )
    }
}

impl ToTree for UpdateExpressionTarget {
    fn to_tree(&self) -> String {
        match self {
            Self::Property(p) => p.to_tree(),
            Self::Variable(v) => v.to_tree(),
        }
    }
}
