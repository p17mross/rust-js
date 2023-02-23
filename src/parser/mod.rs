pub mod ast;

mod error;
mod operator_precedence;
mod statement;
mod value;
mod expression;
mod syntax;
mod assignment;

pub use error::ParseError;

use crate::{lexer::{Token, TokenType, token::{ValueLiteral, BinaryOperator}}, engine::{Gc, program::ProgramLocation, Program}};
use operator_precedence::{precedences, BINARY_PRECEDENCES, BinaryPrecedence, Associativity};
use self::{ast::*, error::ParseErrorType};

#[derive(Debug, Default)]
/// Struct responsible for parsing an AST from a token stream
pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    /// Returns the location of the token at `self.tokens[self.i]`
    fn get_location(&self) -> ProgramLocation {
        self.tokens.get(self.i).unwrap().location.clone()
    }

    /// Gets a token from `self.tokens` and increments `self.i` unless it points beyond the end of `self.tokens`.\
    /// ### Returns:
    /// * `Ok(t)` if `self.i` points inside `self.tokens`
    /// * `Err(UnexpectedEOF)` if `self.i` points past the end of `self.tokens`
    fn try_get_token(&mut self) -> Result<&Token, ParseError> {
        let t = self.tokens.get(self.i);
        // Don't increment i past the end of the list
        match t {
            Some(t) => {
                self.i += 1;
                Ok(t)
            },
            None => Err(self.get_error(ParseErrorType::UnexpectedEOF))
        }
    }

    /// Gets a token from `self.tokens` and increments `self.i`, and returns a reference to the token.\
    /// Panics if `self.i` points past the end of `self.tokens` - for non-panicking situations use `self.try_get_token` instead.
    fn get_token(&mut self) -> &Token {
        self.try_get_token().expect("self.i should have pointed inside self.tokens")
    }

    /// Parses an AST from a given [Program]
    pub(crate) fn parse(program: Gc<Program>, tokens: Vec<Token>) -> Result<ASTNodeProgram, ParseError> {
        let mut s = Self {
            tokens,
            i: 0,
        };

        let block = s.parse_statements()?;

        let parsed_program = ASTNodeProgram {
            program,
            block,
        };

        Ok(parsed_program)
    }
}