pub(crate) mod assignment;
pub(crate) mod ast;
pub(crate) mod error;
pub(crate) mod expression;
pub(crate) mod operator_precedence;
pub(crate) mod statement;
pub(crate) mod syntax;
pub(crate) mod value;

pub use error::ParseError;

use self::{ast::*, error::ParseErrorType};
use crate::engine::{program::ProgramLocation, Gc, Program};
use crate::lexer::{
    token::{BinaryOperator, ValueLiteral},
    Token, TokenType,
};
use operator_precedence::{precedences, Associativity, BinaryPrecedence, BINARY_PRECEDENCES};

#[derive(Debug, Default)]
/// Struct responsible for parsing an AST from a token stream
pub(super) struct Parser {
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
            }
            None => Err(self.get_error(ParseErrorType::UnexpectedEOF)),
        }
    }

    /// Gets a token from `self.tokens` and increments `self.i`, and returns a reference to the token.\
    /// Panics if `self.i` points past the end of `self.tokens` - for non-panicking situations use `self.try_get_token` instead.
    fn get_token(&mut self) -> &Token {
        self.try_get_token()
            .expect("self.i should have pointed inside self.tokens")
    }

    /// Parses an AST from a given [Program]
    pub(crate) fn parse(
        program: Gc<Program>,
        tokens: Vec<Token>,
    ) -> Result<ASTNodeProgram, ParseError> {
        let mut s = Self { tokens, i: 0 };

        let location = ProgramLocation {
            program: program.clone(),
            line: 1,
            column: 1,
            index: 0,
        };
        let block = s.parse_statements(Some(location))?;

        let parsed_program = ASTNodeProgram { program, block };

        Ok(parsed_program)
    }
}
