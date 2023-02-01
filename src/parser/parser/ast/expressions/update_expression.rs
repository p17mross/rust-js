use crate::{engine::program::ProgramLocation, parser::{ast::StringExtTreeIndent}};

use super::{ASTNodePropertyLookup, ASTNodeVariable, ASTNodeExpression};

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
    Variable(Box<ASTNodeVariable>),
    Property(Box<ASTNodePropertyLookup>)
}

#[derive(Debug)]
pub struct ASTNodeUpdateExpression {
    pub location: ProgramLocation,

    pub target: UpdateExpressionTarget,
    pub operator_type: UpdateExpressionOperatorType,
    pub side: UpdateExpressionSide,
}

impl TryFrom<ASTNodeExpression> for UpdateExpressionTarget {
    type Error = ();

    fn try_from(value: ASTNodeExpression) -> Result<Self, Self::Error> {
        match value {
            ASTNodeExpression::PropertyLookup(p) => Ok(Self::Property(p)),
            ASTNodeExpression::Variable(v) => Ok(Self::Variable(v)),
            _ => Err(())
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

impl ASTNodeUpdateExpression {
    pub fn to_tree(&self) -> String {
        format!("{:?} {:?} at {}:{}\n|-property: {}", self.side, self.operator_type, self.location.line, self.location.column, self.target.to_tree().indent_tree())
    }
}

impl UpdateExpressionTarget {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Property(p) => p.to_tree(),
            Self::Variable(v) => v.to_tree(),
        }
    }
}