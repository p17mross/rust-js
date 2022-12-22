mod array_literal;
mod object_literal;
mod unary_operators;
mod binary_operators;
mod value_literals;
mod property_lookup;

pub use array_literal::*;
pub use object_literal::*;
pub use unary_operators::*;
pub use binary_operators::*;
pub use value_literals::*;
pub use property_lookup::*;

use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug, Clone)]
pub enum ASTNodeExpression {
    Variable(Rc<RefCell<ASTNodeVariable>>),
    ObjectLiteral(Rc<RefCell<ASTNodeObjectLiteral>>),
    ArrayLiteral(Rc<RefCell<ASTNodeArrayLiteral>>,),
    ValueLiteral(Rc<RefCell<ASTNodeValueLiteral>>),
    UnaryOperator(Rc<RefCell<ASTNodeUnaryOperator>>),
    BinaryOperator(Rc<RefCell<ASTNodeBinaryOperator>>),
    PropertyLookup(Rc<RefCell<ASTNodePropertyLookup>>),
    Grouping(Rc<RefCell<ASTNodeGrouping>>),
}

pub struct ASTNodeGrouping{
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub expression: ASTNodeExpression,
}

pub struct ASTNodeVariable {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub identifier: String,
}

#[derive(Debug, Clone)]
pub enum ASTNodeExpressionParent {
    LetExpression(Weak<RefCell<ASTNodeLetExpression>>),
    Block(Weak<RefCell<ASTNodeBlock>>),
    UnaryOperator(Weak<RefCell<ASTNodeUnaryOperator>>),
    BinaryOperator(Weak<RefCell<ASTNodeBinaryOperator>>),
    ObjectLiteral(Weak<RefCell<ASTNodeObjectLiteral>>),
    ArrayLiteral(Weak<RefCell<ASTNodeArrayLiteral>>),
    PropertyLookup(Weak<RefCell<ASTNodePropertyLookup>>),
    Grouping(Weak<RefCell<ASTNodeGrouping>>),

    Unset,
}

impl ASTNodeExpression {
    pub fn get_parent(&self) -> ASTNodeExpressionParent {
        match self {
            Self::Variable(v) => v.borrow().parent.clone(),
            Self::ObjectLiteral(o) => o.borrow().parent.clone(),
            Self::ArrayLiteral(a) => a.borrow().parent.clone(),
            Self::ValueLiteral(v) => v.borrow().parent.clone(),
            Self::UnaryOperator(u) => u.borrow().parent.clone(),
            Self::BinaryOperator(b) => b.borrow().parent.clone(),
            Self::PropertyLookup(l) => l.borrow().parent.clone(),
            Self::Grouping(g) => g.borrow().parent.clone(),
        }
    }

    pub fn set_parent(&mut self, parent: ASTNodeExpressionParent) {
        match self {
            Self::Variable(v) => (*v).borrow_mut().parent = parent,
            Self::ObjectLiteral(o) => (*o).borrow_mut().parent = parent,
            Self::ArrayLiteral(a) => (*a).borrow_mut().parent = parent,
            Self::ValueLiteral(v) => (*v).borrow_mut().parent = parent,
            Self::UnaryOperator(u) => (*u).borrow_mut().parent = parent,
            Self::BinaryOperator(b) => (*b).borrow_mut().parent = parent,
            Self::PropertyLookup(l) => (*l).borrow_mut().parent = parent,
            Self::Grouping(g) => (*g).borrow_mut().parent = parent,
        }
    }

    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Self::Variable(v) => v.borrow().location.clone(),
            Self::ObjectLiteral(o) => o.borrow().location.clone(),
            Self::ArrayLiteral(a) => a.borrow().location.clone(),
            Self::ValueLiteral(v) => v.borrow().location.clone(),
            Self::UnaryOperator(u) => u.borrow().location.clone(),
            Self::BinaryOperator(b) => b.borrow().location.clone(),
            Self::PropertyLookup(l) => l.borrow().location.clone(),
            Self::Grouping(g) => g.borrow().location.clone(),
        }
    }
}

impl Debug for ASTNodeGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeGrouping at {}:{} {{{:?}}}",
            self.location.line,
            self.location.column,
            self.expression
        ))
    }
}

impl Debug for ASTNodeVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeVariable at {}:{}: \"{:?}\"", 
            self.location.line, 
            self.location.column, 
            self.identifier
        ))
    }
}

impl From<ASTNodeStatementParent> for ASTNodeExpressionParent {
    fn from(s: ASTNodeStatementParent) -> Self {
        match s {
            ASTNodeStatementParent::Block(b) => Self::Block(b),

            ASTNodeStatementParent::Unset => Self::Unset,
        }
    }
}

impl PartialEq for ASTNodeExpressionParent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Match arms are laid out like this to avoid a '_ => false' branch
            // This means that if new enum variants are added, this code will not compile
            (ASTNodeExpressionParent::Block(b), ASTNodeExpressionParent::Block(p)) => b.ptr_eq(p),
            (ASTNodeExpressionParent::Block(_), _) => false,
            (ASTNodeExpressionParent::LetExpression(b), ASTNodeExpressionParent::LetExpression(p)) => b.ptr_eq(p),
            (ASTNodeExpressionParent::LetExpression(_), _) => false,
            (ASTNodeExpressionParent::UnaryOperator(u), ASTNodeExpressionParent::UnaryOperator(p)) => u.ptr_eq(p),
            (ASTNodeExpressionParent::UnaryOperator(_), _) => false,
            (ASTNodeExpressionParent::BinaryOperator(u), ASTNodeExpressionParent::BinaryOperator(p)) => u.ptr_eq(p),
            (ASTNodeExpressionParent::BinaryOperator(_), _) => false,
            (ASTNodeExpressionParent::ObjectLiteral(o), ASTNodeExpressionParent::ObjectLiteral(p)) => o.ptr_eq(p),
            (ASTNodeExpressionParent::ObjectLiteral(_), _) => false,
            (ASTNodeExpressionParent::ArrayLiteral(a), ASTNodeExpressionParent::ArrayLiteral(p)) => a.ptr_eq(p),
            (ASTNodeExpressionParent::ArrayLiteral(_), _) => false,
            (ASTNodeExpressionParent::PropertyLookup(l), ASTNodeExpressionParent::PropertyLookup(p)) => l.ptr_eq(p),
            (ASTNodeExpressionParent::PropertyLookup(_), _) => false,
            (ASTNodeExpressionParent::Grouping(g), ASTNodeExpressionParent::Grouping(p)) => g.ptr_eq(p),
            (ASTNodeExpressionParent::Grouping(_), _) => false,

            (ASTNodeExpressionParent::Unset, _) => false,
            
        }
    }
}

impl ASTNodeGrouping {
    pub fn to_tree(&self) -> String {
        format!("Grouping at {}:{}: {}", self.location.line, self.location.column, self.expression.to_tree())
    }
}

impl ASTNodeVariable {
    pub fn to_tree(&self) -> String {
        format!("Variable at {}:{}: \"{}\"", self.location.line, self.location.column, self.identifier)
    }
}

impl ASTNodeExpression {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Variable(v) => v.borrow().to_tree(),
            Self::ObjectLiteral(o) => o.borrow().to_tree(),
            Self::ArrayLiteral(a) => a.borrow().to_tree(),
            Self::ValueLiteral(v) => v.borrow().to_tree(),
            Self::UnaryOperator(u) => u.borrow().to_tree(),
            Self::BinaryOperator(b) => b.borrow().to_tree(),
            Self::PropertyLookup(l) => l.borrow().to_tree(),
            Self::Grouping(g) => g.borrow().to_tree(),
        }
    }
}

impl CheckParent for ASTNodeExpression {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        match self {
            Self::Variable(v) => v.check_parent(p.into()),
            Self::ObjectLiteral(o) => o.check_parent(p.into()),
            Self::ArrayLiteral(a) => a.check_parent(p.into()),
            Self::ValueLiteral(v) => v.check_parent(p.into()),
            Self::UnaryOperator(u) => u.check_parent(p.into()),
            Self::BinaryOperator(b) => b.check_parent(p.into()),
            Self::PropertyLookup(l) => l.check_parent(p.into()),
            Self::Grouping(g) => g.check_parent(p.into()),
        }
    }
}

impl CheckParent for Rc<RefCell<ASTNodeGrouping>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on grouping at {}:{}", s_ref.location.line, s_ref.location.column);
        }
        s_ref.expression.check_parent(ASTNodeExpressionParent::Grouping(Rc::downgrade(self)));
    }
}

impl CheckParent for Rc<RefCell<ASTNodeVariable>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on variable at {}:{}", s_ref.location.line, s_ref.location.column);
        }
    }
}