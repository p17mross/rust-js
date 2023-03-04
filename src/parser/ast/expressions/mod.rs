//! Types representing [`Expression`]s which can be evaluated

pub(crate) mod array_literal;
pub(crate) mod assignment;
pub(crate) mod binary_operators;
pub(crate) mod function_call;
pub(crate) mod object_literal;
pub(crate) mod property_lookup;
pub(crate) mod unary_operators;
pub(crate) mod update_expression;
pub(crate) mod value_literals;

pub(crate) use array_literal::*;
pub(crate) use assignment::*;
pub(crate) use binary_operators::*;
pub(crate) use function_call::*;
pub(crate) use object_literal::*;
pub(crate) use property_lookup::*;
pub(crate) use unary_operators::*;
pub(crate) use update_expression::*;
pub(crate) use value_literals::*;

use crate::engine::program::ProgramLocation;

use super::*;

/// All the different types of expression.
/// Variants are boxed to prevent recursive types
#[derive(Debug)]
pub(crate) enum Expression {
    /// An [`ArrayLiteral`]
    ArrayLiteral(Box<ArrayLiteral>),
    /// An [`Assignment`]
    Assignment(Box<Assignment>),
    /// A [`BinaryOperator`][ASTNodeBinaryOperator]
    BinaryOperator(Box<ASTNodeBinaryOperator>),
    /// A [`FunctionCall`]
    FunctionCall(Box<FunctionCall>),
    /// An [`ObjectLiteral`]
    ObjectLiteral(Box<ObjectLiteral>),
    /// A [`PropertyLookup`]
    PropertyLookup(Box<PropertyLookup>),
    /// A [`UnaryOperator`]
    UnaryOperator(Box<ASTNodeUnaryOperator>),
    /// An [`UpdateAssignment`]
    UpdateAssignment(Box<UpdateAssignment>),
    /// An [`UpdateExpression`]
    UpdateExpression(Box<UpdateExpression>),
    /// A [`ValueLiteral`][ASTNodeValueLiteral]
    ValueLiteral(Box<ASTNodeValueLiteral>),
    /// A [`Variable`]
    Variable(Box<Variable>),
}

/// A variable lookup
#[derive(Debug)]
pub(crate) struct Variable {
    /// The location of the variable
    pub location: ProgramLocation,
    /// The variable's name
    pub identifier: String,
}

impl Expression {
    /// Get the location of an [`Expression`]
    #[must_use]
    pub fn get_location(&self) -> ProgramLocation {
        match self {
            Self::ArrayLiteral(a) => a.location.clone(),
            Self::Assignment(a) => a.location.clone(),
            Self::BinaryOperator(b) => b.location.clone(),
            Self::FunctionCall(f) => f.location.clone(),
            Self::ObjectLiteral(o) => o.location.clone(),
            Self::PropertyLookup(l) => l.location.clone(),
            Self::UnaryOperator(u) => u.location.clone(),
            Self::UpdateAssignment(u) => u.location.clone(),
            Self::UpdateExpression(u) => u.location.clone(),
            Self::ValueLiteral(v) => v.location.clone(),
            Self::Variable(v) => v.location.clone(),
        }
    }

    /// Get the type of an [`Expression`] as a [`String`]
    #[must_use]
    pub fn get_type(&self) -> String {
        match self {
            Self::ArrayLiteral(_) => "Array literal".to_string(),
            Self::Assignment(_) => "Assignment".to_string(),
            Self::BinaryOperator(b) => format!("Binary operator {:?}", b.operator_type),
            Self::FunctionCall(_) => "Function call".to_string(),
            Self::ObjectLiteral(_) => "Object literal".to_string(),
            Self::PropertyLookup(_) => "Property lookup".to_string(),
            Self::UnaryOperator(u) => format!("Unary operator {:?}", u.operator_type),
            Self::UpdateAssignment(u) => format!("Update Assignment {:?}", u.operator_type),
            Self::UpdateExpression(u) => format!("{:?} {:?}", u.side, u.operator_type),
            Self::ValueLiteral(_) => "Value literal".to_string(),
            Self::Variable(_) => "Variable".to_string(),
        }
    }
}

impl ToTree for Variable {
    fn to_tree(&self) -> String {
        format!(
            "Variable at {}:{}: \"{}\"",
            self.location.line, self.location.column, self.identifier
        )
    }
}

impl ToTree for Expression {
    fn to_tree(&self) -> String {
        match self {
            Self::ArrayLiteral(a) => a.to_tree(),
            Self::Assignment(a) => a.to_tree(),
            Self::BinaryOperator(b) => b.to_tree(),
            Self::FunctionCall(f) => f.to_tree(),
            Self::ObjectLiteral(o) => o.to_tree(),
            Self::PropertyLookup(l) => l.to_tree(),
            Self::UnaryOperator(u) => u.to_tree(),
            Self::UpdateExpression(u) => u.to_tree(),
            Self::UpdateAssignment(u) => u.to_tree(),
            Self::ValueLiteral(v) => v.to_tree(),
            Self::Variable(v) => v.to_tree(),
        }
    }
}
