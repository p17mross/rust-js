use crate::{engine::program::ProgramLocation, lexer::token::ValueLiteral};

#[derive(Debug)]
pub struct ASTNodeValueLiteral {
    pub location: ProgramLocation,

    pub value: ValueLiteral,
}

impl ASTNodeValueLiteral {
    pub fn to_tree(&self) -> String {
        format!("String Literal at {}:{}: {:?}", self.location.line, self.location.column, self.value)
    }
}