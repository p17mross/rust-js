pub mod ast;

use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::{lexer::{Token, TokenType}, engine::{Gc, program::ProgramLocation, Program}};

use self::ast::*;

#[derive(Debug, Clone)]
/// All the types of errors that can occur during parsing
pub enum ParseErrorType {
    UnexpectedToken{found: &'static str, expected: Option<&'static str>},
    UnmatchedBrace,
    UnexpectedEOF,

    /// Any other syntax errors
    SyntaxError,
}

impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError => f.write_str(""),
            Self::UnexpectedEOF => f.write_str("Unexpected EOF"),
            Self::UnexpectedToken { found, expected } => match expected {
                None => f.write_fmt(format_args!("Unexpected token '{found}'")),
                Some(expected) => f.write_fmt(format_args!("Unexpected token: expected '{expected}', found '{found}'"))
            },
            Self::UnmatchedBrace => f.write_str("Unmatched brace")
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

#[derive(Debug, Default)]
/// Struct responsible for parsing
pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {

    fn get_location(&self) -> ProgramLocation {
        self.tokens.get(self.i).unwrap().location.clone()
    }

    fn get_token(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.i);
        self.i += 1;
        t
    }

    fn get_error(&self, error_type: ParseErrorType) -> ParseError {
        ParseError {
            location: self.tokens.get(self.i - 1).unwrap().location.clone(),
            error_type
        }
    }

    fn parse_pattern(&mut self) -> Result<Option<Rc<RefCell<ASTNodePattern>>>, ParseError> {
        todo!()
    }

    fn parse_expression(&mut self) -> Result<ASTNodeExpression, ParseError> {
        todo!()
    }

    fn parse_object_literal(&mut self) -> Result<Option<Rc<RefCell<ASTNodeObjectLiteral>>>, ParseError> {
        todo!()
    }

    fn parse_let_expression(&mut self) -> Result<Option<Rc<RefCell<ASTNodeLetExpression>>>, ParseError> {

        // Save location for later
        let start_i = self.i;
        let location = self.get_location();

        let pattern = self.parse_pattern()?;
        let Some(pattern) = pattern else {
            self.i = start_i;
            return Ok(None)
        };

        let Some(t) = self.get_token().cloned() else {
            return Err(self.get_error(ParseErrorType::UnexpectedEOF));
        };

        if t.token_type != TokenType::OperatorAssignment {
            return Err(self.get_error(ParseErrorType::UnexpectedToken {found: t.token_type.to_str(), expected: Some("=")}))
        };
        let mut value = self.parse_expression()?;

        let l = Rc::new(RefCell::new(ASTNodeLetExpression {
            location,
            parent: ASTNodeStatementParent::Unset,
            lhs: pattern.clone(),
            rhs: value.clone(),
        }));

        (*pattern).borrow_mut().parent = ASTNodePatternParent::LetExpression(Rc::downgrade(&l));
        value.set_parent(ASTNodeExpressionParent::LetExpression(Rc::downgrade(&l)));

        Ok(Some(l))
    }

    fn parse_block(&mut self, require_end_brace: bool) -> Result<Option<Rc<RefCell<ASTNodeBlock>>>, ParseError> {
        // Save location for later
        let start_i = self.i;

        let block = Rc::new(RefCell::new(ASTNodeBlock {
            location: self.get_location(),
            parent: ASTNodeBlockParent::Unset,

            statements: vec![]
        }));
        
        'statements: loop {
            match self.get_token() {
                None => if require_end_brace {
                    return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                } else {
                    return Ok(Some(block))
                }

                Some(t) => {

                    // If found end brace, return
                    if t.token_type == TokenType::CloseBrace {
                        return Ok(Some(block))
                    }

                    // Ignore newlines
                    if t.token_type == TokenType::NewLine {continue 'statements;}

                    if t.token_type == TokenType::OpenBrace {
                        let inner_block = self.parse_block(true)?;
                        if let Some(inner_block) = inner_block {
                            (*block).borrow_mut().statements.push(ASTNodeStatement::Block(inner_block));
                            continue 'statements;
                        }
                        
                        let object_literal = self.parse_object_literal()?;
                        if let Some(object_literal) = object_literal {
                            (*block).borrow_mut().statements.push(ASTNodeStatement::Expression(Rc::new(RefCell::new(ASTNodeExpression::ObjectLiteral(object_literal)))));
                            continue 'statements;
                        };
                        return Err(self.get_error(ParseErrorType::SyntaxError));
                    }

                    todo!();

                }
            }
        }
    }

    pub(crate) fn parse(program: Gc<Program>, tokens: Vec<Token>) -> Result<Rc<RefCell<ASTNodeProgram>>, ParseError> {
        
        let mut s = Self {
            tokens,
            i: 0,
        };

        Ok(Rc::new(RefCell::new(ASTNodeProgram {
            program,
            block: s.parse_block(false)?.ok_or_else(||s.get_error(ParseErrorType::SyntaxError))?
        })))
    }
}