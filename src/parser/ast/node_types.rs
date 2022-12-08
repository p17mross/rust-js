use std::{rc::Rc, cell::RefCell};

use super::{nodes::*, node_parents::*};

#[derive(Debug)]
pub enum ASTNodeStatement {
    Expression(Rc<RefCell<ASTNodeExpression>>),

    Block(Rc<RefCell<ASTNodeBlock>>),
    LetExpression(Rc<RefCell<ASTNodeLetExpression>>),
}

impl ASTNodeStatement {
    pub fn set_parent(&mut self, parent: ASTNodeStatementParent) {
        match self {
            Self::Expression(e) => (*e).borrow_mut().set_parent(parent.into()),
            Self::Block(b) => (*b).borrow_mut().parent = parent.into(),
            Self::LetExpression(l) => (*l).borrow_mut().parent = parent,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ASTNodeExpression {
    Variable(Rc<RefCell<ASTNodeVariable>>),
    ObjectLiteral(Rc<RefCell<ASTNodeObjectLiteral>>),
    ArrayLiteral(Rc<RefCell<ASTNodeArrayLiteral>>,),
    StringLiteral(Rc<RefCell<ASTNodeStringLiteral>>),
    NumberLiteral(Rc<RefCell<ASTNodeNumberLiteral>>),
    BigIntLiteral(Rc<RefCell<ASTNodeBigIntLiteral>>),
    UnaryPlus(Rc<RefCell<ASTNodeUnaryPlus>>),
    UnaryMinus(Rc<RefCell<ASTNodeUnaryMinus>>),
}

impl ASTNodeExpression {
    pub fn set_parent(&mut self, parent: ASTNodeExpressionParent) {
        match self {
            Self::ObjectLiteral(o) => (*o).borrow_mut().parent = parent,
            Self::Variable(v) => (*v).borrow_mut().parent = parent,
            Self::ArrayLiteral(a) => (*a).borrow_mut().parent = parent,
            Self::StringLiteral(s) => (*s).borrow_mut().parent = parent,
            Self::NumberLiteral(n) => (*n).borrow_mut().parent = parent,
            Self::BigIntLiteral(n) => (*n).borrow_mut().parent = parent,
            Self::UnaryPlus(p) => (*p).borrow_mut().parent = parent,
            Self::UnaryMinus(m) => (*m).borrow_mut().parent = parent,
        }
    }
}