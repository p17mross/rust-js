//! The [`Block`] type

use std::fmt::Debug;

use crate::engine::program::ProgramLocation;

use super::*;

/// A block containing statements
#[derive(Debug)]
pub(crate) struct Block {
    /// The location of the start of the block
    pub location: ProgramLocation,
    /// The block's statements
    pub statements: Vec<Statement>,
}

impl ToTree for Block {
    fn to_tree(&self) -> String {
        let mut s = format!("Block at {}:{}", self.location.line, self.location.column);
        for statement in &self.statements {
            s += "\n|-";
            s += &statement.to_tree().indent_tree();
        }
        s
    }
}
