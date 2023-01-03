use std::{rc::{Weak, Rc}, cell::RefCell, fmt::Debug};

use crate::engine::program::ProgramLocation;

use super::*;

pub struct ASTNodeFunctionCall {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub function: ASTNodeExpression,
    pub args: Rc<RefCell<ASTNodeFunctionCallArgs>>
}

pub struct ASTNodeFunctionCallArgs {
    pub location: ProgramLocation,
    pub parent: Weak<RefCell<ASTNodeFunctionCall>>,

    pub args: Vec<ASTNodeExpression>,
    pub rest: Option<ASTNodeExpression>,
}

impl Debug for ASTNodeFunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeFunctionCall at {}:{}: {{function: {:?}, args: {:?}}}",
            self.location.line,
            self.location.column,
            self.function,
            self.args
        ))
    }
}

impl Debug for ASTNodeFunctionCallArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeFunctionCallArgs at {}:{}: {{args: {:?}, rest: {:?}}}",
            self.location.line,
            self.location.column,
            self.args,
            self.rest
        ))
    }
}

impl ASTNodeFunctionCall {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Function call at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-function: {}\n", self.function.to_tree().indent_tree());
        s += &format!("|-args: {}", self.args.borrow().to_tree().indent_tree());
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

impl CheckParent for Rc<RefCell<ASTNodeFunctionCall>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on function call at {}:{}", s_ref.location.line, s_ref.location.column)
        }

        s_ref.function.check_parent(ASTNodeExpressionParent::FunctionCall(Rc::downgrade(self)));
        s_ref.args.check_parent(Rc::downgrade(self));
    }
}

impl CheckParent for Rc<RefCell<ASTNodeFunctionCallArgs>> {
    type Parent = Weak<RefCell<ASTNodeFunctionCall>>;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if !s_ref.parent.ptr_eq(&p) {
            panic!("Incorrect parent on function call args at {}:{}", s_ref.location.line, s_ref.location.column)
        }
    }
}