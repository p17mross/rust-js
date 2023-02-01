mod expressions;
mod statements;

pub use expressions::*;
pub use statements::*;

use std::fmt::Debug;
use crate::engine::{Gc, Program, garbagecollection::GarbageCollectable, program::ProgramLocation};

// Format for ASTNode files: 
// 1) Struct/enum definitions
// 2) Method impls on types
// 3) Trait impls on these types (e.g. Debug, From)
// 4) to_tree() impls


pub struct ASTNodeProgram {
    pub program: Gc<Program>,
    pub block: ASTNodeBlock,
}

impl GarbageCollectable for ASTNodeProgram {
    fn get_children(&self) -> Vec<crate::engine::garbagecollection::GarbageCollectionId> {
        vec![self.program.get_id()]
    }
}

impl ASTNodeProgram {
    pub fn new(program: Gc<Program>) -> Self {
        Self {
            program: program.clone(),
            block: ASTNodeBlock {
                location: ProgramLocation { 
                    program: program.clone(),
                    line: 0, 
                    column: 0, 
                    index: 0 
                }, 
                statements: vec![],
            },
        }
    }
}


impl Debug for ASTNodeProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ASTNodeProgram from {:?} {{{:?}}}", self.program.borrow().source, self.block))
    }
}

pub(crate) trait StringExtTreeIndent {
    fn indent_tree(&self) -> Self;
}

impl StringExtTreeIndent for String {
    fn indent_tree(&self) -> Self {
        self.replace('\n', "\n| ")
    }
}

impl ASTNodeProgram {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Program from {}\n", self.program.borrow().source);
        s += &self.block.to_tree();
        s
    }
}