mod block;
mod let_expression;

use std::{rc::Rc, cell::RefCell, collections::HashMap};

pub use block::*;
pub use let_expression::*;

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum ASTNodePatternType {
    Variable(String),
    ArrayDestructure {
        /// Rc is needed here to create a Weak for the parent of the sub-pattern
        items: Vec<Rc<RefCell<ASTNodePattern>>>, 
        spread: Option<Rc<RefCell<ASTNodePattern>>>,
    },
    /// Rc is needed here to create a Weak for the parent of the sub-pattern
    ObjectDestructure(HashMap<String, Rc<RefCell<ASTNodePattern>>>)
}

#[derive(Debug)]
pub struct ASTNodePattern {
    pub location: ProgramLocation,

    pub target: ASTNodePatternType,
}

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
}

impl ASTNodePattern {
    pub fn to_tree(&self) -> String {
        match &self.target {
            ASTNodePatternType::Variable(identifier) => format!("\"{identifier}\""),
            ASTNodePatternType::ArrayDestructure{items, spread} => {
                let mut s = format!("Array destructure at {}:{}\n|-Items: ", self.location.line, self.location.column);
                for (i, item) in items.iter().enumerate() {
                    s += &format!("|-{i}: {}", item.borrow().to_tree().indent_tree());
                }
                if let Some(spread) = spread {
                    s += &format!("|-Spread: {}", spread.borrow().to_tree());
                }
                s
            },
            ASTNodePatternType::ObjectDestructure(items) => {
                let mut s = format!("Object destructure at {}:{}\n", self.location.line, self.location.column);
                for key in items.keys() {
                    s += &format!("|-\"{key}\": {}", items[key].borrow().to_tree());
                }
                s
            }
        }
    }
}

impl ASTNodeStatement {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Block(n) => n.to_tree(),
            Self::LetExpression(n) => n.to_tree(),
            Self::Expression(e) => e.to_tree(),
        }
    }
}