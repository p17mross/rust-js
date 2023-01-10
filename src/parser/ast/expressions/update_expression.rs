use crate::{engine::program::ProgramLocation, parser::ast::StringExtTreeIndent};

use super::{ASTNodePropertyLookup, ASTNodeVariable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateExpressionOperatorType {
    Addition,
    Subtraction,
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

impl std::fmt::Display for UpdateExpressionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Prefix => "prefix",
            Self::Postfix => "postfix",
        })
    }
}

#[derive(Debug)]
pub struct ASTNodeUpdateExpression {
    pub location: ProgramLocation,

    pub target: UpdateExpressionTarget,
    pub operator_type: UpdateExpressionOperatorType,
    pub side: UpdateExpressionSide,
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