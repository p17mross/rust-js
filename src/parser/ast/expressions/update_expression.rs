//! The [`UpdateExpression`] and related types

use crate::{
    engine::program::ProgramLocation,
    parser::ast::{StringExtTreeIndent, ToTree},
};

use super::{Expression, PropertyLookup, Variable};

/// The type of an [`UpdateExpression`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UpdateExpressionOperatorType {
    /// An increment `++a` or `a++`
    Increment,
    /// A decrement `--a` or `a--`
    Decrement,
}

/// Which side of the value the [`UpdateExpression`] is on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateExpressionSide {
    /// A prefix `++a` or `--a`
    Prefix,
    /// A postfix `a++` or `a--` 
    Postfix,
}

/// The target of an [`UpdateExpression`]
#[derive(Debug)]
pub(crate) enum UpdateExpressionTarget {
    /// A [`Variable`]
    Variable(Box<Variable>),
    /// A [`PropertyLookup`]
    Property(Box<PropertyLookup>),
}

/// An update expression (`++` or `--` operator)
#[derive(Debug)]
pub(crate) struct UpdateExpression {
    /// The location of the update expression
    pub location: ProgramLocation,

    /// The target of the update expression
    pub target: UpdateExpressionTarget,
    /// The type of operator
    pub operator_type: UpdateExpressionOperatorType,
    /// Which side of the [target][UpdateExpression::target] the operator is
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
