mod block;
mod let_expression;

use std::{rc::{Rc, Weak}, cell::RefCell, collections::HashMap};

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

pub struct ASTNodePattern {
    pub location: ProgramLocation,
    pub parent: ASTNodePatternParent,

    pub target: ASTNodePatternType,
}

#[derive(Debug)]
pub enum ASTNodeStatement {
    Expression(ASTNodeExpression),
    Block(Rc<RefCell<ASTNodeBlock>>),
    LetExpression(Rc<RefCell<ASTNodeLetExpression>>),
}

#[derive(Debug, Clone)]
pub enum ASTNodePatternParent {
    LetExpression(Weak<RefCell<ASTNodeLetExpression>>),
    Pattern(Weak<RefCell<ASTNodePattern>>),

    Unset,
}

#[derive(Debug, Clone)]
pub enum ASTNodeStatementParent {
    Block(Weak<RefCell<ASTNodeBlock>>),

    Unset,
}

impl ASTNodeStatement {
    pub fn get_parent(&self) -> ASTNodeStatementParent {
        match self {
            ASTNodeStatement::Expression(e) => e.get_parent().into(),
            ASTNodeStatement::Block(b) => b.borrow().parent.clone().into(),
            ASTNodeStatement::LetExpression(l) => l.borrow().parent.clone(),
        }
    }

    pub fn set_parent(&mut self, parent: ASTNodeStatementParent) {
        match self {
            Self::Expression(e) => (*e).set_parent(parent.into()),
            Self::Block(b) => (*b).borrow_mut().parent = parent.into(),
            Self::LetExpression(l) => (*l).borrow_mut().parent = parent,
        }
    }

    pub fn get_location(&self) -> ProgramLocation {
        match self {
            ASTNodeStatement::Expression(e) => e.get_location(),
            ASTNodeStatement::Block(b) => b.borrow().location.clone(),
            ASTNodeStatement::LetExpression(l) => l.borrow().location.clone(),
        }
    }
}

impl Debug for ASTNodePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeAssignmentLHS at {}:{} {{target: {:?}}}", 
            self.location.line, 
            self.location.column, 
            self.target
        ))
    }
}

impl From<ASTNodeExpressionParent> for ASTNodeStatementParent {
    fn from(e: ASTNodeExpressionParent) -> Self {
        match e {
            ASTNodeExpressionParent::Block(b) => Self::Block(b),
            ASTNodeExpressionParent::Unset => Self::Unset,

            ASTNodeExpressionParent::LetExpression(_) => panic!("Statement cannot be a child of a let expression"),
            ASTNodeExpressionParent::UnaryOperator(_) => panic!("Statement cannot be a child of a unary operator"),
            ASTNodeExpressionParent::BinaryOperator(_) => panic!("Statement cannot be a child of a binary operator"),
            ASTNodeExpressionParent::ObjectLiteral(_) => panic!("Statement cannot be a child of an object literal"),
            ASTNodeExpressionParent::ArrayLiteral(_) => panic!("Statement cannot be a child of an array literal"),
        }
    }
}

impl From<ASTNodeBlockParent> for ASTNodeStatementParent {
    fn from(b: ASTNodeBlockParent) -> Self {
        match b {
            ASTNodeBlockParent::Block(b) => Self::Block(b),
            ASTNodeBlockParent::Unset => Self::Unset,

            ASTNodeBlockParent::Program(_) => panic!("Statement cannot be a child of a program")
        }
    }
}

impl PartialEq for ASTNodePatternParent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Match arms are laid out like this to avoid a '_ => false' branch
            // This means that if new enum variants are added, this code will not compile
            (ASTNodePatternParent::LetExpression(l), ASTNodePatternParent::LetExpression(p)) => l.ptr_eq(p),
            (ASTNodePatternParent::LetExpression(_), _) => false,
            (ASTNodePatternParent::Pattern(pat), ASTNodePatternParent::Pattern(par)) => pat.ptr_eq(par),
            (ASTNodePatternParent::Pattern(_), _) => false,

            (ASTNodePatternParent::Unset, _) => false,
        }
    }
}

impl PartialEq for ASTNodeStatementParent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Match arms are laid out like this to avoid a '_ => false' branch
            // This means that if new enum variants are added, this code will not compile
            (ASTNodeStatementParent::Block(b), ASTNodeStatementParent::Block(p)) => b.ptr_eq(p),
            (ASTNodeStatementParent::Block(_), _) => false,

            (ASTNodeStatementParent::Unset, _) => false,

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
            Self::Block(n) => n.borrow().to_tree(),
            Self::LetExpression(n) => n.borrow().to_tree(),
            Self::Expression(e) => e.to_tree(),
        }
    }
}

impl CheckParent for ASTNodeStatement {
    type Parent = ASTNodeStatementParent;
    fn check_parent(&self, p: Self::Parent) {
        match self {
            ASTNodeStatement::Block(b) => b.check_parent(p.into()),
            ASTNodeStatement::Expression(e) => e.check_parent(p.into()),
            ASTNodeStatement::LetExpression(l) => l.check_parent(p.into())
        }
    }
}

impl CheckParent for Rc<RefCell<ASTNodePattern>> {
    type Parent = ASTNodePatternParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on pattern at {}:{}", s_ref.location.line, s_ref.location.column)
        }

        match &s_ref.target {
            ASTNodePatternType::ArrayDestructure { items, spread } => {
                for item in items {
                    item.check_parent(ASTNodePatternParent::Pattern(Rc::downgrade(&self)));
                }
                if let Some(spread) = spread {
                    spread.check_parent(ASTNodePatternParent::Pattern(Rc::downgrade(&self)));
                }
            }
            ASTNodePatternType::ObjectDestructure(keys) => {
                for (_, pattern) in keys {
                    pattern.check_parent(ASTNodePatternParent::Pattern(Rc::downgrade(&self)));
                }
            }
            ASTNodePatternType::Variable(_) => ()
        }
    }
}