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
        match error_type {
            // Handle UnexpectedEOF differently as self.i likely points beyond the end of self.tokens
            ParseErrorType::UnexpectedEOF => {
                ParseError {
                    // TODO: get actual line:column of end of file not just the last token
                    location: self.tokens.get(self.tokens.len() - 1).unwrap().location.clone(),
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
        let mut lhs = match self.get_token() {
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
                    let o = Rc::new_cyclic(|p|{
                        expression.set_parent(ASTNodeExpressionParent::UnaryOperator(p.clone()));
                        RefCell::new(ASTNodeUnaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type: UnaryOperatorType::Plus,
                            expression: expression
                        })
                    });
                    ASTNodeExpression::UnaryOperator(o)
                }

                // Unary minus
                TokenType::OperatorSubtraction => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new_cyclic(|p|{
                        expression.set_parent(ASTNodeExpressionParent::UnaryOperator(p.clone()));
                        RefCell::new(ASTNodeUnaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type: UnaryOperatorType::Minus,
                            expression: expression
                        })
                    });
                    ASTNodeExpression::UnaryOperator(o)
                }

                // Logical not
                TokenType::OperatorLogicalNot => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new_cyclic(|p|{
                        expression.set_parent(ASTNodeExpressionParent::UnaryOperator(p.clone()));
                        RefCell::new(ASTNodeUnaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type: UnaryOperatorType::LogicalNot,
                            expression: expression
                        })
                    });
                    ASTNodeExpression::UnaryOperator(o)
                }

                // Bitwise not
                TokenType::OperatorBitwiseNot => {
                    let location = t.location.clone();
                    let mut expression = self.parse_expression(false)?;
                    let o = Rc::new_cyclic(|p|{
                        expression.set_parent(ASTNodeExpressionParent::UnaryOperator(p.clone()));
                        RefCell::new(ASTNodeUnaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type: UnaryOperatorType::BitwiseNot,
                            expression: expression
                        })
                    });
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
                    let mut rhs = self.parse_expression(false)?;
                    let b = Rc::new_cyclic(|p|{
                        lhs.set_parent(ASTNodeExpressionParent::BinaryOperator(p.clone()));
                        rhs.set_parent(ASTNodeExpressionParent::BinaryOperator(p.clone()));
                        RefCell::new(ASTNodeBinaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type: BinaryOperator::Addition,
                            lhs,
                            rhs,
                        }
                    )});

                    Ok(ASTNodeExpression::BinaryOperator(b))
                }

                TokenType::OperatorSubtraction => {
                    let location = t.location.clone();
                    let mut rhs = self.parse_expression(false)?;
                    let b = Rc::new_cyclic(|p|{
                        lhs.set_parent(ASTNodeExpressionParent::BinaryOperator(p.clone()));
                        rhs.set_parent(ASTNodeExpressionParent::BinaryOperator(p.clone()));
                        RefCell::new(ASTNodeBinaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type: BinaryOperator::Subtraction,
                            lhs,
                            rhs,
                        }
                    )});

                    Ok(ASTNodeExpression::BinaryOperator(b))
                }

                TokenType::BinaryOperator(operator_type) => {
                    let operator_type = operator_type.clone();
                    let location = t.location.clone();
                    let mut rhs = self.parse_expression(false)?;
                    let b = Rc::new_cyclic(|p|{
                        lhs.set_parent(ASTNodeExpressionParent::BinaryOperator(p.clone()));
                        rhs.set_parent(ASTNodeExpressionParent::BinaryOperator(p.clone()));
                        RefCell::new(ASTNodeBinaryOperator {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            operator_type,
                            lhs,
                            rhs,
                        }
                    )});

                    Ok(ASTNodeExpression::BinaryOperator(b))
                }

                TokenType::OperatorDot => {
                    let t = t.clone();
                    let location = t.location;
                    let Some(rhs) = self.get_token() else {
                        return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                    };
                    let TokenType::Identifier(i) = rhs.token_type.clone() else {
                        let t_str = t.token_type.to_str();
                        return Err(self.get_error(ParseErrorType::UnexpectedToken { found: t_str, expected: Some("identifier") }))
                    };


                    Ok(ASTNodeExpression::PropertyLookup(Rc::new_cyclic(|p|{
                        lhs.set_parent(ASTNodeExpressionParent::PropertyLookup(p.clone()));
                        RefCell::new(ASTNodePropertyLookup {
                            location,
                            parent: ASTNodeExpressionParent::Unset,
                            lhs,
                            rhs: i
                        })
                    })))
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

        let l = Rc::new_cyclic(|p|{
            pattern.borrow_mut().parent = ASTNodePatternParent::LetExpression(p.clone());
            value.set_parent(ASTNodeExpressionParent::LetExpression(p.clone()));
            RefCell::new(ASTNodeLetExpression {
                location,
                parent: ASTNodeStatementParent::Unset,
                lhs: pattern,
                rhs: value,
            })
        });

        // Consume semicolon or newline
        self.get_token();

        Ok(Some(l))
    }

    fn parse_statement(&mut self) -> Result<Option<ASTNodeStatement>, ParseError> {
        match self.get_token() {
            None => Ok(None),
            Some(t) => {

                // If found end brace, return
                if t.token_type == TokenType::CloseBrace {
                    self.i -= 1;
                    return Ok(None)
                }

                // Parse block
                if t.token_type == TokenType::OpenBrace {
                    return Ok(Some(ASTNodeStatement::Block(self.parse_block(true)?)));
                }

                // TODO: remove once more token types are implemented
                let temp_t = t.token_type.clone();

                if let TokenType::Identifier(i) = &t.token_type {
                    match i.as_str() {
                        "let" => {
                            let s = self.parse_let_expression()?;
                            if let Some(s) = s {
                                return Ok(Some(ASTNodeStatement::LetExpression(s)));
                            }
                        },

                        _ => ()
                    }
                }

                todo!("{:?} as start of statement", temp_t);

            }
        }
    }

    fn parse_block(&mut self, require_end_brace: bool) -> Result<Rc<RefCell<ASTNodeBlock>>, ParseError> {
        let block = Rc::new(RefCell::new(ASTNodeBlock {
            location: self.get_location(),
            parent: ASTNodeBlockParent::Unset,

            statements: vec![]
        }));
        
        let mut block_ref = block.borrow_mut();

        'statements: loop {
            self.consume_newlines();
            let statement = self.parse_statement()?;

            let Some(mut statement) = statement else {
                // If end of file reached without closing brace
                if self.get_token().is_none() && require_end_brace {
                    return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                }
                break 'statements;
            };
            statement.set_parent(ASTNodeStatementParent::Block(Rc::downgrade(&block)));
            block_ref.statements.push(statement);
        }

        drop(block_ref);
        Ok(block)
    }

    pub(crate) fn parse(program: Gc<Program>, tokens: Vec<Token>) -> Result<Rc<RefCell<ASTNodeProgram>>, ParseError> {
        let mut s = Self {
            tokens,
            i: 0,
        };

        let block = s.parse_block(false)?;
        let mut block_ref = block.borrow_mut();

        let parsed_program = Rc::new(RefCell::new(ASTNodeProgram {
            program,
            block: block.clone(),
        }));

        block_ref.parent = ASTNodeBlockParent::Program(Rc::downgrade(&parsed_program));

        Ok(parsed_program)
    }
}