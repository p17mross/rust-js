use std::fmt::Display;

use super::*;

#[derive(Debug, Clone)]
/// A type of error that can occur during parsing
pub enum ParseErrorType {
    /// A certain type of token was found when it was not expected
    UnexpectedToken{found: &'static str, expected: Option<&'static str>},
    /// The end of the file was reached when it was not expected
    UnexpectedEOF,
    /// An expression was expected, but not found
    ExpectedExpression{found: Option<&'static str>},
    /// The argument to an increment or decrement operator was not an assignment target
    InvalidUpdateExpressionOperand(UpdateExpressionSide),

    /// Any other syntax errors
    SyntaxError,
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError => f.write_str("invalid syntax"),
            Self::UnexpectedEOF => f.write_str("unexpected EOF"),
            Self::UnexpectedToken { found, expected } => match expected {
                None => f.write_fmt(format_args!("unexpected token '{found}'")),
                Some(expected) => f.write_fmt(format_args!("unexpected token: expected '{expected}', found '{found}'"))
            },
            Self::ExpectedExpression { found} => match found {
                None => f.write_fmt(format_args!("expected expression")),
                Some(found) => f.write_fmt(format_args!("expected expression, found '{found}'"))
            },
            Self::InvalidUpdateExpressionOperand(s) => f.write_fmt(format_args!("invalid {} operand", s))
        }
    }
}

impl std::error::Error for ParseErrorType {}

#[derive(Debug, Clone)]
/// An error that occurs during parsing
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

impl Parser {
    pub(super) fn get_error(&self, error_type: ParseErrorType) -> ParseError {
        match error_type {
            // Handle UnexpectedEOF differently as self.i likely points beyond the end of self.tokens
            ParseErrorType::UnexpectedEOF => {
                ParseError {
                    // TODO: get actual line:column of end of file not just the last token
                    location: self.tokens.last().unwrap().location.clone(),
                    error_type
                }
            },
            _ => {
                ParseError {
                    location: self.tokens.get(self.i - 1).unwrap().location.clone(),
                    error_type
                }
            }
        }
    }
}