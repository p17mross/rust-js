//! Functionality to do with the [`LexError`] type

use std::fmt::Display;

use crate::{
    engine::{Gc, ProgramLocation},
    util::NumberLiteralBase,
    Program,
};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
/// All the types of errors that can occur during lexing
pub enum LexErrorType {
    /// When an EOF occurs during a string literal.
    /// char is the type of quote used in the string
    UnclosedString(char),
    /// When a newline occurs during a string literal.
    /// char is the type of quote used in the string
    NewlineInString(char),
    /// When an identifier starts immediately after a numeric literal
    IdentifierAfterNumber,
    /// When the start of a numeric literal occurs with no digits following
    MissingDigits {
        /// The base of the invalid literal
        base: NumberLiteralBase
    },
    /// When an invalid unicode occurs outside of a string
    InvalidChar(char),
    /// When brackets are incorrectly matched
    MismatchedBracket,
    /// Unclosed Bracket
    UnclosedBracket,
    /// An invalid BigInt literal, such as `012n`
    InvalidBigInt,
    /// A comment wasn't closed
    UnclosedComment,
    /// An underscore was in an invalid place in a numeric literal
    InvalidUnderscore,
}

impl Display for LexErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnclosedString(c) => {
                write!(f, "{c}{c} literal not terminated before end of script")
            }
            Self::NewlineInString(c) => {
                write!(f, "{c}{c} literal contains an unescaped line break")
            }
            Self::IdentifierAfterNumber => {
                f.write_str("Identifier starts immediately after numeric literal")
            }
            Self::MissingDigits { base: n } => {
                write!(
                    f,
                    "Missing {} digits after '{}'",
                    n.get_name(),
                    n.get_start()
                )
            }
            Self::InvalidChar(c) => {
                write!(f, "Illegal character U+{:x}", *c as u32)
            }
            Self::MismatchedBracket => f.write_str("Mismatched bracket"),
            Self::UnclosedBracket => f.write_str("Unclosed bracket"),
            Self::InvalidBigInt => f.write_str("Invalid BigInt literal"),
            Self::UnclosedComment => f.write_str("Unclosed comment"),
            Self::InvalidUnderscore => f.write_str("Underscore in invalid position in literal"),
        }
    }
}

#[derive(Debug, Clone)]
/// An error that occurs during lexing.
pub struct LexError {
    /// Where the error occured
    pub(super) location: ProgramLocation,
    /// The type of the error
    pub(super) error_type: LexErrorType,
}

impl LexError {
    /// Creates a new [`ProgramLocation`] from a given [`Program`] and location information
    pub(super) const fn new(
        program: Gc<Program>,
        line: usize,
        line_index: usize,
        index: usize,
        e: LexErrorType,
    ) -> Self {
        Self {
            location: ProgramLocation {
                program,
                line,
                column: index - line_index + 1,
                index,
            },
            error_type: e,
        }
    }

    /// Get the location of the error
    #[must_use]
    pub fn get_location(&self) -> ProgramLocation {
        self.location.clone()
    }

    /// Get the type of the error
    #[must_use]
    pub fn get_type(&self) -> LexErrorType {
        self.error_type
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}\nSyntax Error: {}",
            self.location.program.borrow().source,
            self.location.line,
            self.location.column,
            self.error_type
        )
    }
}
