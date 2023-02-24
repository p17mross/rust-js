mod expressions;
mod statements;
mod assignment;

pub use expressions::*;
pub use statements::*;
pub use assignment::*;

use std::fmt::Debug;
use crate::engine::{Gc, Program, garbage_collection::GarbageCollectable, program::ProgramLocation};

// Format for ASTNode files: 
// 1) Struct/enum definitions
// 2) Method impls on types
// 3) Trait impls on these types (e.g. Debug, From)
// 4) to_tree() impls


pub struct ASTNodeProgram {
    pub program: Gc<Program>,
    pub block: Block,
}

impl GarbageCollectable for ASTNodeProgram {
    fn get_children(&self) -> Vec<crate::engine::garbage_collection::GarbageCollectionId> {
        vec![self.program.get_id()]
    }
}

impl ASTNodeProgram {
    #[must_use]
    pub fn new(program: Gc<Program>) -> Self {
        Self {
            program: program.clone(),
            block: Block {
                location: ProgramLocation { 
                    program,
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

pub trait ToTree {
    #[must_use]
    fn to_tree(&self) -> String;
}

impl ToTree for ASTNodeProgram {
    fn to_tree(&self) -> String {
        let mut s = format!("Program from {}\n", self.program.borrow().source);
        s += &self.block.to_tree();
        s
    }
}