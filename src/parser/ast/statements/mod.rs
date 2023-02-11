mod block;
mod let_expression;

pub use block::*;
pub use let_expression::*;

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum ASTNodeStatement {
    Expression(ASTNodeExpression),
    Block(Box<ASTNodeBlock>),
    LetExpression(Box<ASTNodeLetExpression>),
}

impl ASTNodeStatement {
    pub fn get_location(&self) -> ProgramLocation {
        match self {
            ASTNodeStatement::Expression(e) => e.get_location(),
            ASTNodeStatement::Block(b) => b.location.clone(),
            ASTNodeStatement::LetExpression(l) => l.location.clone(),
        }
    }

    pub fn to_tree(&self) -> String {
        match self {
            Self::Block(n) => n.to_tree(),
            Self::LetExpression(n) => n.to_tree(),
            Self::Expression(e) => e.to_tree(),
        }
    }
}