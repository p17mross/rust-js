use std::fmt::Debug;

use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub struct Block {
    pub location: ProgramLocation,

    pub statements: Vec<Statement>,
}

impl Block {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Block at {}:{}", self.location.line, self.location.column);
        for statement in self.statements.iter() {
            s += "\n|-";
            s += &statement.to_tree().indent_tree();
        }
        s
    }
}