use crate::{engine::program::ProgramLocation, lexer::token::ValueLiteral, parser::ast::ToTree};

#[derive(Debug)]
pub struct ASTNodeValueLiteral {
    pub location: ProgramLocation,

    pub value: ValueLiteral,
}

impl ToTree for ASTNodeValueLiteral{
   fn to_tree(&self) -> String {
        let v = match &self.value {
            ValueLiteral::BigInt(b) => format!("BigInt literal {b}"),
            ValueLiteral::Number(n) => format!("Number literal {n}"),
            ValueLiteral::String(s) => format!("String literal \"{s}\""),
        };

        format!("Value Literal at {}:{}: {v}", self.location.line, self.location.column)
    }
}