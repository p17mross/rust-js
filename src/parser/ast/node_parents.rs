use std::{rc::Weak, cell::RefCell};

use super::nodes::*;

#[derive(Debug, Clone)]
pub enum ASTNodeStatementParent {
    Block(Weak<RefCell<ASTNodeBlock>>),

    Unset,
}

pub enum ASTNodeBlockParent {
    Block(Weak<RefCell<ASTNodeBlock>>),
    Program(Weak<RefCell<ASTNodeProgram>>),

    Unset,
}

impl From<ASTNodeStatementParent> for ASTNodeBlockParent {
    fn from(s: ASTNodeStatementParent) -> Self {
        match s {
            ASTNodeStatementParent::Block(b) => Self::Block(b),
            ASTNodeStatementParent::Unset => Self::Unset,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ASTNodePatternParent {
    LetExpression(Weak<RefCell<ASTNodeLetExpression>>),

    Unset,
}

#[derive(Debug, Clone)]
pub enum ASTNodeExpressionParent {
    LetExpression(Weak<RefCell<ASTNodeLetExpression>>),
    Block(Weak<RefCell<ASTNodeBlock>>),

    Unset,
}

impl From<ASTNodeStatementParent> for ASTNodeExpressionParent {
    fn from(s: ASTNodeStatementParent) -> Self {
        match s {
            ASTNodeStatementParent::Block(b) => Self::Block(b),

            ASTNodeStatementParent::Unset => Self::Unset,
        }
    }
}