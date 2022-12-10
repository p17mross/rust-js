pub mod ast;

use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::{lexer::{Token, TokenType, token::BinaryOperator}, engine::{Gc, program::ProgramLocation, Program}};

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

    /// Consumes any [TokenType::NewLine] tokens
    /// Returns true if any tokens were consumed
    #[allow(dead_code)]
    fn consume_newlines(&mut self) -> bool{
        let mut any_consumed = false;
        while let Some(Token { location:_, token_type:TokenType::NewLine }) = self.tokens.get(self.i) {
            self.i += 1;
            any_consumed = true;
        }
        any_consumed
    }

    fn get_error(&self, error_type: ParseErrorType) -> ParseError {
        ParseError {
            location: self.tokens.get(self.i - 1).unwrap().location.clone(),
            error_type
        }
    }

    fn parse_pattern(&mut self) -> Result<Option<Rc<RefCell<ASTNodePattern>>>, ParseError> {
        match self.get_token() {
            None => {self.i -= 1; return Ok(None)},
            Some(t) => match &t.token_type {
                // Just a variable
                TokenType::Identifier(i) => return Ok(Some(Rc::new(RefCell::new(ASTNodePattern {
                    location: t.location.clone(), 
                    parent: ASTNodePatternParent::Unset,
                    target: ASTNodePatternType::Variable(i.clone())
                })))),
                // Array destructure
                TokenType::OpenSquareBracket => todo!("Array destructure"),
                // Object destructure
                TokenType::OpenBrace => todo!("Object destructure"),
                
                _ => return Ok(None),
            }
        }
    }

    fn parse_array_literal(&mut self) -> Result<Option<Rc<RefCell<ASTNodeArrayLiteral>>>, ParseError> {
        todo!("Array literals")
    }

    fn parse_expression(&mut self, require_end_paren: bool) -> Result<ASTNodeExpression, ParseError> {
        let lhs = match self.get_token() {
            None => return Err(self.get_error(ParseErrorType::UnexpectedEOF)),
            Some(t) => match &t.token_type {
                // Bracketed expression
                // TODO: this could also be an arrow function
                TokenType::OpenParen => self.parse_expression(true)?,

                // Object literal
                // This cannot be a block as they are not allowed inside expressions
                TokenType::OpenBrace => match self.parse_object_literal()? {
                    None => return Err(self.get_error(ParseErrorType::SyntaxError)),
                    Some(o) => ASTNodeExpression::ObjectLiteral(o),
                },

                // Array literal
                TokenType::OpenSquareBracket => match self.parse_array_literal()? {
                    None => return Err(self.get_error(ParseErrorType::SyntaxError)),
                    Some(a) => ASTNodeExpression::ArrayLiteral(a)
                }

                // Variable
                // TODO: error on reserved words
                // TODO: this could be a function expression
                TokenType::Identifier(i) => ASTNodeExpression::Variable(Rc::new(RefCell::new(ASTNodeVariable{
                    location: t.location.clone(),
                    parent: ASTNodeExpressionParent::Unset,
                    identifier: i.clone()
                }))),

                // String literal
                TokenType::StringLiteral(s) => ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    parent: ASTNodeExpressionParent::Unset,
                    value: ValueLiteral::String(s.clone())
                }))),

                // Number literal
                TokenType::NumberLiteral(n) => ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    parent: ASTNodeExpressionParent::Unset,
                    value: ValueLiteral::Number(*n)
                }))),

                // Bigint literal
                TokenType::BigIntLiteral(n) => ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    parent: ASTNodeExpressionParent::Unset,
                    value: ValueLiteral::BigInt(n.clone())
                }))),

                // Unary plus
                TokenType::OperatorAddition => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new(RefCell::new(ASTNodeUnaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type: UnaryOperatorType::Plus,
                        expression: expression.clone()
                    }));

                    expression.set_parent(ASTNodeExpressionParent::UnaryOperator(Rc::downgrade(&o)));
                    ASTNodeExpression::UnaryOperator(o)
                }

                // Unary minus
                TokenType::OperatorSubtraction => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new(RefCell::new(ASTNodeUnaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type: UnaryOperatorType::Minus,
                        expression: expression.clone()
                    }));

                    expression.set_parent(ASTNodeExpressionParent::UnaryOperator(Rc::downgrade(&o)));
                    ASTNodeExpression::UnaryOperator(o)
                }

                // Logical not
                TokenType::OperatorLogicalNot => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new(RefCell::new(ASTNodeUnaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type: UnaryOperatorType::LogicalNot,
                        expression: expression.clone()
                    }));

                    expression.set_parent(ASTNodeExpressionParent::UnaryOperator(Rc::downgrade(&o)));
                    ASTNodeExpression::UnaryOperator(o)
                }

                // Bitwise not
                TokenType::OperatorBitwiseNot => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new(RefCell::new(ASTNodeUnaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type: UnaryOperatorType::BitwiseNot,
                        expression: expression.clone()
                    }));

                    expression.set_parent(ASTNodeExpressionParent::UnaryOperator(Rc::downgrade(&o)));
                    ASTNodeExpression::UnaryOperator(o)
                }

                t => todo!("{t:?} as lhs of expression"),
            }
        };

        match self.get_token() {
            None => Ok(lhs),
            Some(t) => match &t.token_type {
                TokenType::Semicolon => {
                    self.i -= 1;
                    Ok(lhs)
                },
                TokenType::CloseParen => if require_end_paren {
                    Ok(lhs)
                } else {
                    Err(self.get_error(ParseErrorType::UnexpectedToken {
                        found: "}",
                        expected: None
                    }))
                },
                TokenType::OperatorAddition => {
                    let location = t.location.clone();
                    let rhs = self.parse_expression(false)?;
                    let b = Rc::new(RefCell::new(ASTNodeBinaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type: BinaryOperator::Addition,
                        lhs,
                        rhs,
                    }));

                    b.borrow_mut().lhs.set_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(&b)));
                    b.borrow_mut().rhs.set_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(&b)));

                    Ok(ASTNodeExpression::BinaryOperator(b))
                }

                TokenType::OperatorSubtraction => {
                    let location = t.location.clone();
                    let rhs = self.parse_expression(false)?;
                    let b = Rc::new(RefCell::new(ASTNodeBinaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type: BinaryOperator::Subtraction,
                        lhs,
                        rhs,
                    }));

                    b.borrow_mut().lhs.set_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(&b)));
                    b.borrow_mut().rhs.set_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(&b)));

                    Ok(ASTNodeExpression::BinaryOperator(b))
                }

                TokenType::BinaryOperator(b) => {
                    let operator_type = b.clone();
                    let location = t.location.clone();
                    let rhs = self.parse_expression(false)?;
                    let b = Rc::new(RefCell::new(ASTNodeBinaryOperator {
                        location,
                        parent: ASTNodeExpressionParent::Unset,
                        operator_type,
                        lhs,
                        rhs,
                    }));

                    b.borrow_mut().lhs.set_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(&b)));
                    b.borrow_mut().rhs.set_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(&b)));

                    Ok(ASTNodeExpression::BinaryOperator(b))
                }


                t => todo!("{t:?} as middle of expression."),
            }
        }
    }

    fn parse_object_literal(&mut self) -> Result<Option<Rc<RefCell<ASTNodeObjectLiteral>>>, ParseError> {
        todo!("Parsing object literals")
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

        let mut value = self.parse_expression(false)?;

        let l = Rc::new(RefCell::new(ASTNodeLetExpression {
            location,
            parent: ASTNodeStatementParent::Unset,
            lhs: pattern.clone(),
            rhs: value.clone(),
        }));

        (*pattern).borrow_mut().parent = ASTNodePatternParent::LetExpression(Rc::downgrade(&l));
        value.set_parent(ASTNodeExpressionParent::LetExpression(Rc::downgrade(&l)));

        // Consume semicolon or newline
        self.get_token();

        Ok(Some(l))
    }

    fn parse_block(&mut self, require_end_brace: bool) -> Result<Option<Rc<RefCell<ASTNodeBlock>>>, ParseError> {
        // Save location for later
        let _start_i = self.i;

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
                            (*block).borrow_mut().statements.push(ASTNodeStatement::Expression(ASTNodeExpression::ObjectLiteral(object_literal)));
                            continue 'statements;
                        };
                        return Err(self.get_error(ParseErrorType::SyntaxError));
                    }

                    // TODO: remove once more token types are implemented
                    let temp_t = t.token_type.clone();

                    if let TokenType::Identifier(i) = &t.token_type {
                        match i.as_str() {
                            "let" => {
                                if let Some(l) = self.parse_let_expression()? {
                                    (*l).borrow_mut().parent = ASTNodeStatementParent::Block(Rc::downgrade(&block));
                                    (*block).borrow_mut().statements.push(ASTNodeStatement::LetExpression(l));
                                    continue 'statements;
                                }
                            },

                            _ => ()
                        }
                    }

                    todo!("{:?} as start of statement", temp_t);

                }
            }
        }
    }

    pub(crate) fn parse(program: Gc<Program>, tokens: Vec<Token>) -> Result<Rc<RefCell<ASTNodeProgram>>, ParseError> {
        let mut s = Self {
            tokens,
            i: 0,
        };

        let parsed_program = Rc::new(RefCell::new(ASTNodeProgram {
            program,
            block: s.parse_block(false)?.ok_or_else(||s.get_error(ParseErrorType::SyntaxError))?
        }));

        let parsed_ref = (*parsed_program).borrow_mut();
        let mut block_ref = (*parsed_ref.block).borrow_mut();

        block_ref.parent = ASTNodeBlockParent::Program(Rc::downgrade(&parsed_program));

        drop(block_ref);
        drop(parsed_ref);

        Ok(parsed_program)
    }
}