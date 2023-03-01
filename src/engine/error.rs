//! Contains error types for loading [`Program`][super::Program]s

use std::fmt::Display;

use crate::{lexer::LexError, parser::ParseError};

#[derive(Debug)]
/// A general type for any syntax error that can occur.
/// Combines [`LexError`] and [`ParseError`] for easier error handling.
pub enum SyntaxError {
    /// A [`LexError`]
    LexError(LexError),
    /// A [`ParseError`]
    ParseError(ParseError),
}

impl From<LexError> for SyntaxError {
    fn from(e: LexError) -> Self {
        Self::LexError(e)
    }
}

impl From<ParseError> for SyntaxError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pass on formatting to either variant
        match self {
            Self::LexError(e) => write!(f, "{e}"),
            Self::ParseError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SyntaxError {}

#[derive(Debug)]
/// An general type for any error that can occur while reading a program from a file.
/// Combines [`std::io::Error`] and [`SyntaxError`] for easier error handling.
pub enum ProgramFromFileError {
    /// There was an error loading the program from the file
    IoError(std::io::Error),
    /// The file did not contain valid javascript
    SyntaxError(SyntaxError),
}

impl From<std::io::Error> for ProgramFromFileError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<SyntaxError> for ProgramFromFileError {
    fn from(e: SyntaxError) -> Self {
        Self::SyntaxError(e)
    }
}

impl Display for ProgramFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{e}"),
            Self::SyntaxError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ProgramFromFileError {}
