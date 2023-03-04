//! Types representing [`Statement`]s which can be executed

mod block;
mod let_expression;

pub(crate) use block::*;
pub(crate) use let_expression::*;

use crate::engine::program::ProgramLocation;

use super::*;

/// A statement which can be run
#[derive(Debug)]
pub(crate) enum Statement {
    /// An expression to be evaluated
    Expression(Expression),
    /// A block of other statements
    Block(Box<Block>),
    /// A [`LetExpression`]
    LetExpression(Box<LetExpression>),
}

impl Statement {
    /// Get the location of a [`Statement`]
    #[must_use]
    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Self::Expression(e) => e.get_location(),
            Self::Block(b) => b.location.clone(),
            Self::LetExpression(l) => l.location.clone(),
        }
    }
}

impl ToTree for Statement {
    fn to_tree(&self) -> String {
        match self {
            Self::Block(n) => n.to_tree(),
            Self::LetExpression(n) => n.to_tree(),
            Self::Expression(e) => e.to_tree(),
        }
    }
}
