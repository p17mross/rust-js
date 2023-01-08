use std::{rc::Rc, cell::RefCell};

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub struct ASTNodeFunctionCall {
    pub location: ProgramLocation,

    pub function: ASTNodeExpression,
    pub args: Rc<RefCell<ASTNodeFunctionCallArgs>>,

    /// Whether the function call is optionally chained e.g. 'a?.(b)'
    pub optional: bool,
}

#[derive(Debug)]
pub struct ASTNodeNew {
    pub location: ProgramLocation,

    pub function: ASTNodeExpression,
    pub args: Option<Rc<RefCell<ASTNodeFunctionCallArgs>>>,
}

#[derive(Debug)]
pub struct ASTNodeFunctionCallArgs {
    pub location: ProgramLocation,

    pub args: Vec<ASTNodeExpression>,
    pub rest: Option<ASTNodeExpression>,
}

impl ASTNodeFunctionCall {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Function call at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-function: {}\n", self.function.to_tree().indent_tree());
        s += &format!("|-args: {}", self.args.borrow().to_tree().indent_tree());
        s
    }
}

impl ASTNodeNew {
    pub fn to_tree(&self) -> String {
        let mut s = format!("New at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-function: {}\n", self.function.to_tree().indent_tree());
        s += &format!("|-args: {}", match &self.args {
            Some(a) => a.borrow().to_tree().indent_tree(),
            None => "[]".to_string()
        });
        s
    }
}

impl ASTNodeFunctionCallArgs {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Function call at {}:{}\n", self.location.line, self.location.column);
        for (i, arg) in self.args.iter().enumerate() {
            s += &format!("|-args[{i}]: {}\n", arg.to_tree().indent_tree());
        }
        if let Some(rest) = &self.rest {
            s += &format!("|-rest: {}", rest.to_tree().indent_tree());
        }
        
        s

    }
}