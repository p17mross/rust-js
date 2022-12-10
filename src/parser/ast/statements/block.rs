use std::{rc::Weak, cell::RefCell, fmt::Debug};

use crate::{engine::program::ProgramLocation, parser::ast::ASTNodeProgram};

use super::*;

pub struct ASTNodeBlock {
    pub location: ProgramLocation,
    pub parent: ASTNodeBlockParent,

    pub statements: Vec<ASTNodeStatement>,
}

#[derive(Debug, Clone)]
pub enum ASTNodeBlockParent {
    Block(Weak<RefCell<ASTNodeBlock>>),
    Program(Weak<RefCell<ASTNodeProgram>>),

    Unset,
}

impl Debug for ASTNodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("ASTNodeProgram at {}:{} {{", self.location.line, self.location.column);
        for (i, statement) in self.statements.iter().enumerate() {
            s += &format!("{statement:?}");
            if i != self.statements.len() - 1 {
                s += ", ";
            }
        }
        s += "}";

        f.write_str(&s)    
    }
}

impl From<ASTNodeStatementParent> for ASTNodeBlockParent {
    fn from(s: ASTNodeStatementParent) -> Self {
        match s {
            ASTNodeStatementParent::Block(b) => Self::Block(b),
            ASTNodeStatementParent::Unset => Self::Unset,
        }
    }
}

impl PartialEq for ASTNodeBlockParent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Match arms are laid out like this to avoid 'a _ => false' branch
            // This means that if new enum variants are added, this code will not compile
            (ASTNodeBlockParent::Block(b), ASTNodeBlockParent::Block(parent)) => b.ptr_eq(parent),
            (ASTNodeBlockParent::Block(_), _) => false,
            (ASTNodeBlockParent::Program(p), ASTNodeBlockParent::Program(parent)) => p.ptr_eq(parent),
            (ASTNodeBlockParent::Program(_), _) => false,

            (ASTNodeBlockParent::Unset, _) => false
        }
    }
}

impl ASTNodeBlock {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Block at {}:{}", self.location.line, self.location.column);
        for statement in self.statements.iter() {
            s += "\n|-";
            s += &statement.to_tree().indent_tree();
        }
        s
    }
}

impl CheckParent for Rc<RefCell<ASTNodeBlock>> {
    type Parent = ASTNodeBlockParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();

        if s_ref.parent != p {
            panic!("Incorrect parent on block at {}:{}", s_ref.location.line, s_ref.location.column);
        }

        for statement in &s_ref.statements {
            statement.check_parent(ASTNodeStatementParent::Block(Rc::downgrade(&self)));
        }   
    }
}