//! The [`ParseError`] type for representing errors during parsing

use std::fmt::Display;

use super::*;

use super::ast::expressions::UpdateExpressionSide;

#[derive(Debug, Clone)]
/// A type of error that can occur during parsing
pub enum ParseErrorType {
    /// A certain type of token was found when it was not expected
    UnexpectedToken {
        /// A string representation of the token which was found
        found: &'static str,
        /// What token was expected
        expected: Option<&'static str>,
    },
    /// The end of the file was reached when it was not expected
    UnexpectedEOF,
    /// An expression was expected, but not found
    ExpectedExpression {
        /// A string representation of the token which was found
        found: Option<&'static str>
    },
    /// The argument to an increment or decrement operator was not an assignment target
    InvalidUpdateExpressionOperand(UpdateExpressionSide),
    /// The LHS of an assignment is not valid
    InvalidAssignmentLHS,
    /// An update assignment operator was used on a destructuring assignment
    InvalidDestructuringAssignmentOperator,
    /// There were items after the rest element of an array destructure
    ItemsAfterRestElementInArrayDestructure,
    /// Expected a property of an object literal
    ExpectedProperty,

    /// Any other syntax errors
    SyntaxError,
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError => f.write_str("invalid syntax"),
            Self::UnexpectedEOF => f.write_str("unexpected EOF"),
            Self::UnexpectedToken { found, expected } => match expected {
                None => write!(f, "unexpected token '{found}'"),
                Some(expected) => write!(
                    f,
                    "unexpected token: expected '{expected}', found '{found}'"
                ),
            },
            Self::ExpectedExpression { found } => match found {
                None => write!(f, "expected expression"),
                Some(found) => write!(f, "expected expression, found '{found}'"),
            },
            Self::InvalidUpdateExpressionOperand(s) => write!(f, "invalid {s} operand"),
            Self::InvalidAssignmentLHS => f.write_str("Invalid assignment left hand side"),
            Self::InvalidDestructuringAssignmentOperator => {
                f.write_str("Only the '=' operator may be used in a destructuring assignment")
            }
            Self::ItemsAfterRestElementInArrayDestructure => {
                f.write_str("The rest element of an array destructure must be the last item")
            }
            Self::ExpectedProperty => f.write_str("Expected a property"),
        }
    }
}

impl std::error::Error for ParseErrorType {}

#[derive(Debug, Clone)]
/// An error that occurs during parsing
pub struct ParseError {
    /// Where the error occurred
    location: ProgramLocation,
    /// The type of the error
    error_type: ParseErrorType,
}

impl ParseError {
    /// Get the location of the error
    #[must_use]
    pub fn get_location(&self) -> ProgramLocation {
        self.location.clone()
    }

    /// Get the type of the error
    #[must_use]
    pub fn get_type(&self) -> ParseErrorType {
        self.error_type.clone()
    }
}

impl Display for ParseError {
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

impl std::error::Error for ParseError {}

impl Parser {
    /// Get an error at the location of the token at `self.tokens[self.i]` with the given [`ParseErrorType`]
    pub(super) fn get_error(&self, error_type: ParseErrorType) -> ParseError {
        match error_type {
            // Handle UnexpectedEOF differently as self.i likely points beyond the end of self.tokens
            ParseErrorType::UnexpectedEOF => {
                ParseError {
                    // TODO: get actual line:column of end of file not just the last token
                    location: self.tokens.last().unwrap().location.clone(),
                    error_type,
                }
            }
            _ => ParseError {
                location: self
                    .tokens
                    .get(self.i.saturating_sub(1))
                    .unwrap()
                    .location
                    .clone(),
                error_type,
            },
        }
    }
}
