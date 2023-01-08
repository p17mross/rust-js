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

use std::{rc::Rc, cell::RefCell};

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug, Clone)]
pub enum ASTNodeExpression {
    ArrayLiteral(Rc<RefCell<ASTNodeArrayLiteral>>),
    BinaryOperator(Rc<RefCell<ASTNodeBinaryOperator>>),
    FunctionCall(Rc<RefCell<ASTNodeFunctionCall>>),
    Grouping(Rc<RefCell<ASTNodeGrouping>>),
    New(Rc<RefCell<ASTNodeNew>>),
    ObjectLiteral(Rc<RefCell<ASTNodeObjectLiteral>>),
    PropertyLookup(Rc<RefCell<ASTNodePropertyLookup>>),
    UnaryOperator(Rc<RefCell<ASTNodeUnaryOperator>>),
    ValueLiteral(Rc<RefCell<ASTNodeValueLiteral>>),
    Variable(Rc<RefCell<ASTNodeVariable>>),
}

#[derive(Debug)]
pub struct ASTNodeGrouping{
    pub location: ProgramLocation,

    pub expression: ASTNodeExpression,
}

#[derive(Debug)]
pub struct ASTNodeVariable {
    pub location: ProgramLocation,

    pub identifier: String,
}


impl ASTNodeExpression {
    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Self::ArrayLiteral(a) => a.borrow().location.clone(),
            Self::BinaryOperator(b) => b.borrow().location.clone(),
            Self::FunctionCall(f) => f.borrow().location.clone(),
            Self::Grouping(g) => g.borrow().location.clone(),
            Self::New(n) => n.borrow().location.clone(),
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
            Self::New(_) => "New".to_string(),
            Self::ObjectLiteral(_) => "Object literal".to_string(),
            Self::PropertyLookup(_) => "Property lookup".to_string(),
            Self::UnaryOperator(u) => format!("Unary operator {:?}", u.borrow().operator_type),
            Self::ValueLiteral(_) => "Value literal".to_string(),
            Self::Variable(_) => "Variable".to_string(),
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
            Self::New(n) => n.borrow().to_tree(),
            Self::ObjectLiteral(o) => o.borrow().to_tree(),
            Self::PropertyLookup(l) => l.borrow().to_tree(),
            Self::UnaryOperator(u) => u.borrow().to_tree(),
            Self::ValueLiteral(v) => v.borrow().to_tree(),
            Self::Variable(v) => v.borrow().to_tree(),
        }
    }
}