use std::fmt::Display;

use crate::engine::program::ProgramLocation;

use super::*;

/// The type of a function call
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FunctionCallType {
    FunctionCall,
    OptionalChainedFunctionCall,
    New,
}

#[derive(Debug)]
pub(crate) struct FunctionCall {
    pub location: ProgramLocation,

    pub function: Expression,
    pub args: Vec<FunctionCallArgument>,

    /// The type of the call
    pub call_type: FunctionCallType,
}

#[derive(Debug)]
pub(crate) struct FunctionCallArgument {
    pub location: ProgramLocation,

    pub expression: Expression,
    pub spread: bool,
}

impl Display for FunctionCallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionCall => f.write_str("Function Call"),
            Self::OptionalChainedFunctionCall => f.write_str("Optionally Chained Function Call"),
            Self::New => f.write_str("New"),
        }
    }
}

impl ToTree for FunctionCall {
    fn to_tree(&self) -> String {
        let mut s = format!(
            "{} at {}:{}\n",
            self.call_type, self.location.line, self.location.column
        );
        s += &format!("|-function: {}\n", self.function.to_tree().indent_tree());

        s += &if self.args.is_empty() {
            "|-no args".to_string()
        } else {
            let args = self
                .args
                .iter()
                .map(|arg| format!("\n|-{}", arg.to_tree().indent_tree()))
                .collect::<String>();

            "|-args:".to_string() + &args
        }
        .indent_tree();

        s
    }
}

impl ToTree for FunctionCallArgument {
    fn to_tree(&self) -> String {
        if self.spread {
            format!(
                "Spread at {}:{} from {}",
                self.location.line,
                self.location.column,
                self.expression.to_tree()
            )
        } else {
            self.expression.to_tree()
        }
    }
}
