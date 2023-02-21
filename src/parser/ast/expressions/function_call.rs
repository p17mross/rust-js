use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub struct FunctionCall {
    pub location: ProgramLocation,

    pub function: Expression,
    pub args: Vec<FunctionCallArgument>,

    /// Whether the function call is optionally chained e.g. 'a?.(b)'
    pub optional: bool,
}

#[derive(Debug)]
pub struct ASTNodeNew {
    pub location: ProgramLocation,

    pub function: Expression,
    pub args: Vec<FunctionCallArgument>,
}

#[derive(Debug)]
pub struct FunctionCallArgument {
    location: ProgramLocation,

    expression: Expression,
    spread: bool,
}

impl FunctionCall {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Function call at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-function: {}\n", self.function.to_tree().indent_tree());

        if self.args.is_empty() {
            s += "|-no args";
        } else {
            s += "|-args: ";
            for arg in &self.args {
                s += &format!("|-{}", arg.to_tree().indent_tree());
            }
        }
        s
    }
}

impl ASTNodeNew {
    pub fn to_tree(&self) -> String {
        let mut s = format!("New at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-function: {}\n", self.function.to_tree().indent_tree());

        if self.args.is_empty() {
            s += "|-no args";
        } else {
            s += "|-args: ";
            for arg in &self.args {
                s += &format!("|-{}", arg.to_tree().indent_tree());
            }
        }

        s
    }
}

impl FunctionCallArgument {
    pub fn to_tree(&self) -> String {
        let self_description = if self.spread {
            format!("Spread at {}:{} from: ", self.location.line, self.location.column)
        } else {
            "".to_string()
        };
        format!("{}: {}", self_description, self.expression.to_tree())
    }
}