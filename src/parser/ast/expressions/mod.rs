mod array_literal;
mod binary_operators;
mod function_call;
mod object_literal;
mod property_lookup;
mod unary_operators;
mod value_literals;
mod update_expression;
mod assignment;

pub use array_literal::*;
pub use binary_operators::*;
pub use function_call::*;
pub use object_literal::*;
pub use property_lookup::*;
pub use unary_operators::*;
pub use value_literals::*;
pub use update_expression::*;
pub use assignment::*;

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum Expression {
    ArrayLiteral(Box<ArrayLiteral>),
    Assignment(Box<Assignment>),
    BinaryOperator(Box<ASTNodeBinaryOperator>),
    FunctionCall(Box<FunctionCall>),
    ObjectLiteral(Box<ObjectLiteral>),
    PropertyLookup(Box<PropertyLookup>),
    UnaryOperator(Box<ASTNodeUnaryOperator>),
    UpdateAssignment(Box<UpdateAssignment>),
    UpdateExpression(Box<UpdateExpression>),
    ValueLiteral(Box<ASTNodeValueLiteral>),
    Variable(Box<Variable>),
}

#[derive(Debug)]
pub struct Variable {
    pub location: ProgramLocation,

    pub identifier: String,
}


impl Expression {
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
            Self::UpdateExpression(u) => {
                format!("{:?} {:?}", u.side, u.operator_type)
            },
            Self::ValueLiteral(_) => "Value literal".to_string(),
            Self::Variable(_) => "Variable".to_string(),
        }
    }
}

impl ToTree for Variable{
   fn to_tree(&self) -> String {
        format!("Variable at {}:{}: \"{}\"", self.location.line, self.location.column, self.identifier)
    }
}

impl ToTree for Expression{
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