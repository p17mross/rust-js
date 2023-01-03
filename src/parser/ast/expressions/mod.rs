mod array_literal;
mod binary_operators;
mod function_call;
mod object_literal;
mod property_lookup;
mod unary_operators;
mod value_literals;
mod pre_and_postfix;
mod assignment;

pub use array_literal::*;
pub use binary_operators::*;
pub use function_call::*;
pub use object_literal::*;
pub use property_lookup::*;
pub use unary_operators::*;
pub use value_literals::*;
pub use pre_and_postfix::*;
pub use assignment::*;

use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug, Clone)]
pub enum ASTNodeExpression {
    ArrayLiteral(Rc<RefCell<ASTNodeArrayLiteral>>),
    BinaryOperator(Rc<RefCell<ASTNodeBinaryOperator>>),
    FunctionCall(Rc<RefCell<ASTNodeFunctionCall>>),
    Grouping(Rc<RefCell<ASTNodeGrouping>>),
    ObjectLiteral(Rc<RefCell<ASTNodeObjectLiteral>>),
    PropertyLookup(Rc<RefCell<ASTNodePropertyLookup>>),
    UnaryOperator(Rc<RefCell<ASTNodeUnaryOperator>>),
    ValueLiteral(Rc<RefCell<ASTNodeValueLiteral>>),
    Variable(Rc<RefCell<ASTNodeVariable>>),
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
    ArrayLiteral(Weak<RefCell<ASTNodeArrayLiteral>>),
    BinaryOperator(Weak<RefCell<ASTNodeBinaryOperator>>),
    Block(Weak<RefCell<ASTNodeBlock>>),
    FunctionCall(Weak<RefCell<ASTNodeFunctionCall>>),
    FunctionCallArgs(Weak<RefCell<ASTNodeFunctionCallArgs>>),
    Grouping(Weak<RefCell<ASTNodeGrouping>>),
    LetExpression(Weak<RefCell<ASTNodeLetExpression>>),
    ObjectLiteral(Weak<RefCell<ASTNodeObjectLiteral>>),
    PropertyLookup(Weak<RefCell<ASTNodePropertyLookup>>),
    UnaryOperator(Weak<RefCell<ASTNodeUnaryOperator>>),

    Unset,
}

impl ASTNodeExpression {
    pub fn get_parent(&self) -> ASTNodeExpressionParent {
        match self {
            Self::ArrayLiteral(a) => a.borrow().parent.clone(),
            Self::BinaryOperator(b) => b.borrow().parent.clone(),
            Self::FunctionCall(f) => f.borrow().parent.clone(),
            Self::Grouping(g) => g.borrow().parent.clone(),
            Self::ObjectLiteral(o) => o.borrow().parent.clone(),
            Self::PropertyLookup(l) => l.borrow().parent.clone(),
            Self::UnaryOperator(u) => u.borrow().parent.clone(),
            Self::ValueLiteral(v) => v.borrow().parent.clone(),
            Self::Variable(v) => v.borrow().parent.clone(),
        }
    }

    pub fn set_parent(&self, parent: ASTNodeExpressionParent) {
        match self {
            Self::ArrayLiteral(a) => (*a).borrow_mut().parent = parent,
            Self::BinaryOperator(b) => (*b).borrow_mut().parent = parent,
            Self::FunctionCall(f) => (*f).borrow_mut().parent = parent,
            Self::Grouping(g) => (*g).borrow_mut().parent = parent,
            Self::ObjectLiteral(o) => (*o).borrow_mut().parent = parent,
            Self::PropertyLookup(l) => (*l).borrow_mut().parent = parent,
            Self::UnaryOperator(u) => (*u).borrow_mut().parent = parent,
            Self::ValueLiteral(v) => (*v).borrow_mut().parent = parent,
            Self::Variable(v) => (*v).borrow_mut().parent = parent,
        }
    }

    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Self::ArrayLiteral(a) => a.borrow().location.clone(),
            Self::BinaryOperator(b) => b.borrow().location.clone(),
            Self::FunctionCall(f) => f.borrow().location.clone(),
            Self::Grouping(g) => g.borrow().location.clone(),
            Self::ObjectLiteral(o) => o.borrow().location.clone(),
            Self::PropertyLookup(l) => l.borrow().location.clone(),
            Self::UnaryOperator(u) => u.borrow().location.clone(),
            Self::ValueLiteral(v) => v.borrow().location.clone(),
            Self::Variable(v) => v.borrow().location.clone(),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Self::ArrayLiteral(_) => "Array literal".to_string(),
            Self::BinaryOperator(b) => format!("Binary operator {:?}", b.borrow().operator_type),
            Self::FunctionCall(_) => "Function call".to_string(),
            Self::Grouping(_) => "Grouping".to_string(),
            Self::ObjectLiteral(_) => "Object literal".to_string(),
            Self::PropertyLookup(_) => "Property lookup".to_string(),
            Self::UnaryOperator(u) => format!("Unary operator {:?}", u.borrow().operator_type),
            Self::ValueLiteral(_) => "Value literal".to_string(),
            Self::Variable(_) => "Variable".to_string(),
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
            (ASTNodeExpressionParent::ArrayLiteral(a), ASTNodeExpressionParent::ArrayLiteral(p)) => a.ptr_eq(p),
            (ASTNodeExpressionParent::ArrayLiteral(_), _) => false,
            (ASTNodeExpressionParent::BinaryOperator(u), ASTNodeExpressionParent::BinaryOperator(p)) => u.ptr_eq(p),
            (ASTNodeExpressionParent::BinaryOperator(_), _) => false,
            (ASTNodeExpressionParent::Block(b), ASTNodeExpressionParent::Block(p)) => b.ptr_eq(p),
            (ASTNodeExpressionParent::Block(_), _) => false,
            (ASTNodeExpressionParent::FunctionCall(f), ASTNodeExpressionParent::FunctionCall(p)) => f.ptr_eq(p),
            (ASTNodeExpressionParent::FunctionCall(_), _) => false,
            (ASTNodeExpressionParent::FunctionCallArgs(f), ASTNodeExpressionParent::FunctionCallArgs(p)) => f.ptr_eq(p),
            (ASTNodeExpressionParent::FunctionCallArgs(_), _) => false,
            (ASTNodeExpressionParent::Grouping(g), ASTNodeExpressionParent::Grouping(p)) => g.ptr_eq(p),
            (ASTNodeExpressionParent::Grouping(_), _) => false,
            (ASTNodeExpressionParent::LetExpression(b), ASTNodeExpressionParent::LetExpression(p)) => b.ptr_eq(p),
            (ASTNodeExpressionParent::LetExpression(_), _) => false,
            (ASTNodeExpressionParent::ObjectLiteral(o), ASTNodeExpressionParent::ObjectLiteral(p)) => o.ptr_eq(p),
            (ASTNodeExpressionParent::ObjectLiteral(_), _) => false,
            (ASTNodeExpressionParent::PropertyLookup(l), ASTNodeExpressionParent::PropertyLookup(p)) => l.ptr_eq(p),
            (ASTNodeExpressionParent::PropertyLookup(_), _) => false,
            (ASTNodeExpressionParent::UnaryOperator(u), ASTNodeExpressionParent::UnaryOperator(p)) => u.ptr_eq(p),
            (ASTNodeExpressionParent::UnaryOperator(_), _) => false,

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
            Self::ArrayLiteral(a) => a.borrow().to_tree(),
            Self::BinaryOperator(b) => b.borrow().to_tree(),
            Self::FunctionCall(f) => f.borrow().to_tree(),
            Self::Grouping(g) => g.borrow().to_tree(),
            Self::ObjectLiteral(o) => o.borrow().to_tree(),
            Self::PropertyLookup(l) => l.borrow().to_tree(),
            Self::UnaryOperator(u) => u.borrow().to_tree(),
            Self::ValueLiteral(v) => v.borrow().to_tree(),
            Self::Variable(v) => v.borrow().to_tree(),
        }
    }
}

impl CheckParent for ASTNodeExpression {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        match self {
            Self::ArrayLiteral(a) => a.check_parent(p),
            Self::BinaryOperator(b) => b.check_parent(p),
            Self::FunctionCall(f) => f.check_parent(p),
            Self::Grouping(g) => g.check_parent(p),
            Self::ObjectLiteral(o) => o.check_parent(p),
            Self::PropertyLookup(l) => l.check_parent(p),
            Self::UnaryOperator(u) => u.check_parent(p),
            Self::ValueLiteral(v) => v.check_parent(p),
            Self::Variable(v) => v.check_parent(p),
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