use std::{rc::Rc, cell::RefCell};

use crate::{engine::program::ProgramLocation};

use super::*;

pub struct ASTNodeLetExpression {
    pub location: ProgramLocation,
    pub parent: ASTNodeStatementParent,

    pub lhs: Rc<RefCell<ASTNodePattern>>,
    pub rhs: ASTNodeExpression,
}

impl Debug for ASTNodeLetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ASTNodeLetExpression {{lhs: {:?}, rhs: {:?}}}", self.lhs, self.rhs))
    }
}

impl ASTNodeLetExpression {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Let expression at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.borrow().to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}

impl CheckParent for Rc<RefCell<ASTNodeLetExpression>> {
    type Parent = ASTNodeStatementParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on let expression at {}:{}", s_ref.location.line, s_ref.location.column)
        }
        s_ref.lhs.check_parent(ASTNodePatternParent::LetExpression(Rc::downgrade(self)));
        s_ref.rhs.check_parent(ASTNodeExpressionParent::LetExpression(Rc::downgrade(self)));
    }
}