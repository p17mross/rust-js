mod block;
mod let_expression;

pub use block::*;
pub use let_expression::*;

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Block(Box<Block>),
    LetExpression(Box<LetExpression>),
}

impl Statement {
    #[must_use]
    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Statement::Expression(e) => e.get_location(),
            Statement::Block(b) => b.location.clone(),
            Statement::LetExpression(l) => l.location.clone(),
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