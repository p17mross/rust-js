pub mod ast;
mod operator_precedence;

use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::{lexer::{Token, TokenType, token::{BinaryOperator, AssignmentOperator}}, engine::{Gc, program::ProgramLocation, Program}};

use self::ast::*;

#[derive(Debug, Clone)]
/// All the types of errors that can occur during parsing
pub enum ParseErrorType {
    UnexpectedToken{found: &'static str, expected: Option<&'static str>},
    UnmatchedBrace,
    UnmatchedParen,
    UnexpectedEOF,
    ExpectedExpression{found: Option<&'static str>},

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
            Self::ExpectedExpression { found} => match found {
                None => f.write_fmt(format_args!("Expected expression")),
                Some(found) => f.write_fmt(format_args!("Expected expression, found '{found}'"))
            },
            Self::UnmatchedBrace => f.write_str("Unmatched brace"),
            Self::UnmatchedParen => f.write_str("Unmatched paren"),
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EndDelimiterRequirement {
    Require,
    Permit,
    Forbid,
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

    /// Consumes any [TokenType::NewLine] and [TokenType::Semicolon] tokens
    fn next_statement(&mut self) {
        while let Some(   Token { location:_, token_type:TokenType::NewLine } 
                        | Token {location: _, token_type:TokenType::Semicolon}
            ) = self.tokens.get(self.i) {
            self.i += 1;
        }
    }

    fn get_error(&self, error_type: ParseErrorType) -> ParseError {
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

    fn parse_pattern(&mut self) -> Result<Option<Rc<RefCell<ASTNodePattern>>>, ParseError> {
        match self.get_token() {
            None => {
                self.i -= 1; 
                Ok(None)
            },
            Some(t) => match &t.token_type {
                // Just a variable
                TokenType::Identifier(i) => Ok(Some(Rc::new(RefCell::new(ASTNodePattern {
                    location: t.location.clone(), 
                    parent: ASTNodePatternParent::Unset,
                    target: ASTNodePatternType::Variable(i.clone())
                })))),
                // Array destructure
                TokenType::OpenSquareBracket => todo!("Array destructure"),
                // Object destructure
                TokenType::OpenBrace => todo!("Object destructure"),
                
                _ => Ok(None),
            }
        }
    }

    fn parse_array_literal(&mut self) -> Result<Rc<RefCell<ASTNodeArrayLiteral>>, ParseError> {
        todo!("Array literals")
    }

    fn parse_object_literal(&mut self) -> Result<Rc<RefCell<ASTNodeObjectLiteral>>, ParseError> {
        todo!("Parsing object literals")
    }

    fn parse_function_args(&mut self) -> Result<Rc<RefCell<ASTNodeFunctionCallArgs>>, ParseError> {
        todo!("Parsing function args")
    }

    fn parse_value(&mut self) -> Result<ASTNodeExpression, ParseError> {
        match self.get_token() {
            None => Err(self.get_error(ParseErrorType::UnexpectedEOF)),
            Some(t) => match &t.token_type {
                // Bracketed expression
                // TODO: this could also be an arrow function
                TokenType::OpenParen => {
                    let t = t.clone();
                    let e = self.parse_expression(EndDelimiterRequirement::Permit, true)?;

                    let Some(Token{location:_, token_type:TokenType::CloseParen}) = self.get_token() else {
                        return Err(self.get_error(ParseErrorType::UnmatchedParen))
                    };

                    let g = ASTNodeExpression::Grouping(Rc::new_cyclic(|p|{
                        e.set_parent(ASTNodeExpressionParent::Grouping(p.clone()));
                        RefCell::new(ASTNodeGrouping {
                            location: t.location.clone(),
                            parent: ASTNodeExpressionParent::Unset,
                            expression: e
                        })
                    }));

                    Ok(g)
                },

                // Close paren
                // This is always a syntax error as if this will only occur with an empty set of parens
                TokenType::CloseParen => {
                    Err(self.get_error(ParseErrorType::ExpectedExpression { found: Some(")") }))
                },

                // Object literal
                // This cannot be a block as they are not allowed inside expressions
                TokenType::OpenBrace => Ok(ASTNodeExpression::ObjectLiteral(self.parse_object_literal()?)),

                // Array literal
                TokenType::OpenSquareBracket => Ok(ASTNodeExpression::ArrayLiteral(self.parse_array_literal()?)),

                // Variable
                // TODO: error on reserved words
                // TODO: this could be a function expression
                TokenType::Identifier(i) => Ok(ASTNodeExpression::Variable(Rc::new(RefCell::new(ASTNodeVariable{
                    location: t.location.clone(),
                    parent: ASTNodeExpressionParent::Unset,
                    identifier: i.clone()
                })))),

                // Value literal
                TokenType::ValueLiteral(v) => Ok(ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    parent: ASTNodeExpressionParent::Unset,
                    value: v.clone()
                })))),
                
                _ => {
                    let e = ParseErrorType::ExpectedExpression { found: Some(t.token_type.to_str()) };
                    Err(self.get_error(e))
                }
            }
        }
    }

    fn parse_expression(&mut self, end_paren: EndDelimiterRequirement, allow_comma_operator: bool) -> Result<ASTNodeExpression, ParseError> {
        #[derive(Debug)]
        enum OperatorOrValue {
            Value(ASTNodeExpression),
            Assignment(ProgramLocation),
            ComputationAssignment(AssignmentOperator, ProgramLocation),
            BinaryOperator(BinaryOperator, ProgramLocation),
            UnaryOperator(UnaryOperator, ProgramLocation),
            PropertyLookup(String, ProgramLocation),
            OptionalChainedLookup(String, ProgramLocation),
            Prefix(PreOrPostfixOperatorType, ProgramLocation),
            Postfix(PreOrPostfixOperatorType, ProgramLocation),
            
        }
        impl OperatorOrValue {
            fn is_operator(&self) -> bool {
                match self {
                    Self::Value(_) => false,
                    Self::Assignment(_) => true,
                    Self::ComputationAssignment(_, _) => true,
                    Self::BinaryOperator(_, _) => true,
                    Self::UnaryOperator(_, _) => true,
                    // In the contexts where this is used, a value of false is useful as the last item was a value
                    Self::PropertyLookup(_, _) => false,
                    Self::OptionalChainedLookup(_, _) => false,

                    Self::Prefix(_, _) => false,
                    Self::Postfix(_, _) => true,
                }
            }

            fn is_value(&self) -> bool {
                !self.is_operator()
            }
        }

        let mut items = Vec::new();

        let mut newline_last_loop = false;
        let mut newline_this_line = false;

        'parse_items: loop {
            match self.get_token() {
                None => match end_paren {
                    EndDelimiterRequirement::Require => return Err(self.get_error(ParseErrorType::UnmatchedParen)),
                    _ => break 'parse_items
                },

                Some(t) => match &t.token_type {
                    // Break on close brace, close square bracket, or semicolon, and don't consume it
                    TokenType::CloseBrace | TokenType::CloseSquareBracket | TokenType::Semicolon => match end_paren {
                        EndDelimiterRequirement::Require => return Err(self.get_error(ParseErrorType::UnmatchedParen)),
                        _ => {
                            self.i -= 1;
                            break 'parse_items;
                        }
                    },
                    // Close paren - handle based on end_paren
                    TokenType::CloseParen => match end_paren {
                        EndDelimiterRequirement::Forbid => return Err(self.get_error(ParseErrorType::UnexpectedToken { found: ")", expected: None })),
                        EndDelimiterRequirement::Require => break 'parse_items,
                        EndDelimiterRequirement::Permit => {
                            self.i -= 1;
                            break 'parse_items;
                        }
                    },
                    
                    TokenType::ValueLiteral(v) => items.push(
                        OperatorOrValue::Value (
                            ASTNodeExpression::ValueLiteral (
                                Rc::new(RefCell::new(ASTNodeValueLiteral {
                                    location: t.location.clone(),
                                    parent: ASTNodeExpressionParent::Unset,
                                    value: v.clone()
                                }))
                            )
                        )
                    ),
                    TokenType::Identifier(i) => items.push(
                        OperatorOrValue::Value (
                            ASTNodeExpression::Variable (
                                Rc::new(RefCell::new(ASTNodeVariable {
                                    location: t.location.clone(),
                                    parent: ASTNodeExpressionParent::Unset,
                                    identifier: i.clone()
                                }))
                            )
                        )
                    ),
                    

                    TokenType::Assignment(a) => items.push(OperatorOrValue::ComputationAssignment(*a, t.location.clone())),
                    TokenType::BinaryOperator(b) => items.push(OperatorOrValue::BinaryOperator(*b, t.location.clone())),
                    TokenType::Comma => if allow_comma_operator {
                        items.push(OperatorOrValue::BinaryOperator(BinaryOperator::Comma, t.location.clone()))
                    } else {
                        self.i -= 1;
                        break 'parse_items;
                    },



                    // Remember newlines to deal with semicolon insertion
                    TokenType::NewLine => newline_this_line = true,
                    // An open paren could be an expression in parens or it could be a function call
                    TokenType::OpenParen => match items.last() {
                        // Function call
                        Some(o) if o.is_value() => {
                            let t = t.clone();
                            let OperatorOrValue::Value(v) = items.pop().unwrap() else {panic!()};
                            let args = self.parse_function_args()?;
                            
                            let f = Rc::new_cyclic(|p| {
                                args.borrow_mut().parent = p.clone();
                                RefCell::new(ASTNodeFunctionCall {
                                    location: t.location,
                                    parent: ASTNodeExpressionParent::Unset,
                                    function: v,
                                    args
                                })
                            });

                            items.push(OperatorOrValue::Value(ASTNodeExpression::FunctionCall(f)))
                        },

                        // Parenthesised expression
                        _ => items.push(OperatorOrValue::Value(self.parse_expression(EndDelimiterRequirement::Require, true)?)),
                    },
                    
                    TokenType::OperatorDot => {
                        let Some(t) = self.get_token() else {
                            return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                        };
                        let t_type = t.token_type.clone();
                        let Token { location, token_type: TokenType::Identifier(i) } = t else {
                            return Err(self.get_error(ParseErrorType::UnexpectedToken { found: t_type.to_str(), expected: Some("identifier") }))
                        };
                        items.push(OperatorOrValue::PropertyLookup(i.clone(), location.clone()));
                    },

                    TokenType::OperatorOptionalChaining => {
                        let Some(t) = self.get_token() else {
                            return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                        };
                        let t_type = t.token_type.clone();
                        let Token { location, token_type: TokenType::Identifier(i) } = t else {
                            return Err(self.get_error(ParseErrorType::UnexpectedToken { found: t_type.to_str(), expected: Some("identifier") }))
                        };
                        items.push(OperatorOrValue::OptionalChainedLookup(i.clone(), location.clone()));
                    },

                    TokenType::OperatorSpread => return Err(self.get_error(ParseErrorType::ExpectedExpression { found: Some("...") })),

                    TokenType::OpenBrace => items.push(OperatorOrValue::Value(ASTNodeExpression::ObjectLiteral(self.parse_object_literal()?))),
                    TokenType::OpenSquareBracket => items.push(OperatorOrValue::Value(ASTNodeExpression::ArrayLiteral(self.parse_array_literal()?))),
                    
                    TokenType::OperatorAssignment => items.push(OperatorOrValue::Assignment(t.location.clone())),
                    
                    // Could be binary or unary
                    TokenType::OperatorAddition => {
                        match items.last() {
                            // Binary operator
                            Some(o) if o.is_value() => {
                                items.push(OperatorOrValue::UnaryOperator(UnaryOperator::Plus, t.location.clone()))
                            }
                            // Unary operator
                            _ => {
                                items.push(OperatorOrValue::BinaryOperator(BinaryOperator::Addition, t.location.clone()))
                            },
                        }
                    },
                    TokenType::OperatorSubtraction => {
                        match items.last() {
                            // Binary operator
                            Some(o) if o.is_value() => {
                                items.push(OperatorOrValue::UnaryOperator(UnaryOperator::Minus, t.location.clone()))
                            }
                            // Unary operator
                            _ => {
                                items.push(OperatorOrValue::BinaryOperator(BinaryOperator::Subtraction, t.location.clone()))
                            },
                        }
                    },
                    
                    // Could be prefix or postfix
                    TokenType::OperatorIncrement => {
                        match items.last() {
                            // Postfix
                            Some(o) if o.is_value() => {
                                items.push(OperatorOrValue::Postfix(PreOrPostfixOperatorType::Addition, t.location.clone()))
                            }
                            // Prefix
                            _ => {
                                items.push(OperatorOrValue::Prefix(PreOrPostfixOperatorType::Addition, t.location.clone()))
                            },
                        }
                    },
                    TokenType::OperatorDecrement => {
                        match items.last() {
                            // Postfix
                            Some(o) if o.is_value() => {
                                items.push(OperatorOrValue::Postfix(PreOrPostfixOperatorType::Subtraction, t.location.clone()))
                            }
                            // Prefix
                            _ => {
                                items.push(OperatorOrValue::Prefix(PreOrPostfixOperatorType::Subtraction, t.location.clone()))
                            },
                        }
                    },

                    TokenType::OperatorLogicalNot => items.push(OperatorOrValue::UnaryOperator(UnaryOperator::LogicalNot, t.location.clone())),
                    TokenType::OperatorBitwiseNot => items.push(OperatorOrValue::UnaryOperator(UnaryOperator::BitwiseNot, t.location.clone())),

                    TokenType::OperatorFatArrow => todo!("Arrow functions"),
                    TokenType::OperatorQuestionMark => todo!("Ternary operator"),
                    TokenType::OperatorColon => todo!("Ternary operator"),

                    
                    //token_type => todo!("Token in expression: {:?}", token_type)
                }
            }
        }

        dbg!(items);

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

        let value = self.parse_expression(EndDelimiterRequirement::Forbid, false)?;

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
                    return Ok(Some(ASTNodeStatement::Block(self.parse_block(EndDelimiterRequirement::Require)?)));
                }

                // TODO: remove once more token types are implemented
                let t_clone = t.token_type.clone();

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

                todo!("{:?} as start of statement", t_clone);

            }
        }
    }

    fn parse_block(&mut self, end_brace: EndDelimiterRequirement) -> Result<Rc<RefCell<ASTNodeBlock>>, ParseError> {
        let block = Rc::new(RefCell::new(ASTNodeBlock {
            location: self.get_location(),
            parent: ASTNodeBlockParent::Unset,

            statements: vec![]
        }));
        
        let mut block_ref = block.borrow_mut();

        'statements: loop {
            self.next_statement();

            let statement = self.parse_statement()?;

            let Some(mut statement) = statement else {
                // If end of file reached without closing brace
                if self.get_token().is_none() && end_brace == EndDelimiterRequirement::Require {
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

        let block = s.parse_block(EndDelimiterRequirement::Forbid)?;
        let mut block_ref = block.borrow_mut();

        let parsed_program = Rc::new(RefCell::new(ASTNodeProgram {
            program,
            block: block.clone(),
        }));

        block_ref.parent = ASTNodeBlockParent::Program(Rc::downgrade(&parsed_program));

        Ok(parsed_program)
    }
}