//! Types for storing the AST of a program

pub(crate) mod assignment;
pub(crate) mod expressions;
pub(crate) mod statements;

use assignment::*;
use expressions::*;
use statements::*;

use crate::engine::{
    garbage_collection::GarbageCollectable, garbage_collection::Gc, program::Program,
};
use std::fmt::Debug;

// Format for ASTNode files:
// 1) Struct/enum definitions
// 2) Method impls on types
// 3) Trait impls on these types (e.g. Debug, From)
// 4) to_tree() impls

/// A parsed AST for a [`Program`]
pub(crate) struct ASTNodeProgram {
    /// The program which this AST is for
    pub program: Gc<Program>,
    /// The AST
    pub block: Block,
}

impl GarbageCollectable for ASTNodeProgram {
    fn get_children(&self) -> Vec<crate::engine::garbage_collection::GarbageCollectionId> {
        vec![self.program.get_id()]
    }
}

impl Debug for ASTNodeProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ASTNodeProgram from {:?} {{{:?}}}",
            self.program.borrow().source,
            self.block
        )
    }
}

/// An extension trait for [`String`]s to replace "\n" with "\n| ".
/// This is used by `to_tree` implementations to indent subtrees.
pub(crate) trait StringExtTreeIndent {
    /// Replaces all instances of "\n" with "\n| "
    fn indent_tree(&self) -> Self;
}

impl StringExtTreeIndent for String {
    fn indent_tree(&self) -> Self {
        self.replace('\n', "\n| ")
    }
}

/// A trait for printing AST nodes as a tree structure
pub trait ToTree {
    /// Get the tree representation of this node and its children
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
