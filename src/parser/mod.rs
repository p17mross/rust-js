use std::fmt::Display;

use crate::{lexer::Token, engine::{Gc, program::ProgramLocation, garbagecollection::{GarbageCollectable, GarbageCollectionId}}};

#[derive(Debug, Clone, PartialEq, Eq)]
/// All the types of errors that can occur during parsing
pub enum ParseErrorType {

}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => f.write_str("TODO")
        }
    }
}

#[derive(Debug, Clone)]
/// An error that occurs during parsing.
pub struct ParseError {
    location: ProgramLocation,
    pub error_type: ParseErrorType,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}:{}\nSyntax Error: {}", self.location.program.borrow().source, self.location.line, self.location.column, self.error_type))
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
/// All the types of ASTNode
pub enum ASTNodeType {

}

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// A struct for an AST node.
pub struct ASTNode {
    location: ProgramLocation,
    node_type: ASTNodeType,
}

impl GarbageCollectable for ASTNode {
    fn get_children(&self) -> Vec<GarbageCollectionId> {
        match self.node_type {

        }
    }
}

#[derive(Debug, Default)]
/// Struct responsible for parsing
pub struct Parser { }

impl Parser {
    pub(crate) fn parse(tokens: Vec<Token>) -> Result<Gc<ASTNode>, ParseError> {
        todo!("{tokens:?}")
    }
}