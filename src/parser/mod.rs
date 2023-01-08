pub mod ast;
mod operator_precedence;

use std::{fmt::Display, rc::Rc, cell::RefCell};

use crate::{lexer::{Token, TokenType, token::ValueLiteral}, engine::{Gc, program::ProgramLocation, Program}, parser::operator_precedence::precedences};

use self::ast::*;

#[derive(Debug, Clone)]
/// All the types of errors that can occur during parsing
pub enum ParseErrorType {
    UnexpectedToken{found: &'static str, expected: Option<&'static str>},
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
        }
    }
}

impl std::error::Error for ParseErrorType {}

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

    /// Consumes any [TokenType::Semicolon] tokens
    fn next_statement(&mut self) {
        while let Some(Token {token_type:TokenType::Semicolon, ..}
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
                    target: ASTNodePatternType::Variable(i.clone())
                })))),
                // Array destructure
                TokenType::OpenSquareBracket(_) => todo!("Array destructure"),
                // Object destructure
                TokenType::OpenBrace(_) => todo!("Object destructure"),
                
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
                TokenType::OpenParen(close_index) => {
                    let t = t.clone();
                    let close_index = close_index.clone();

                    let e = self.parse_expression(0)?;

                    assert_eq!(close_index, self.i);
                    self.i += 1;

                    let g = ASTNodeExpression::Grouping(Rc::new(
                        RefCell::new(ASTNodeGrouping {
                            location: t.location.clone(),
                            expression: e
                        })
                    ));

                    Ok(g)
                },

                // Close paren
                // This is always a syntax error as if this will only occur with an empty set of parens
                TokenType::CloseParen(_) => {
                    Err(self.get_error(ParseErrorType::ExpectedExpression { found: Some(")") }))
                },

                // Object literal
                // This cannot be a block as they are not allowed inside expressions
                TokenType::OpenBrace(_) => Ok(ASTNodeExpression::ObjectLiteral(self.parse_object_literal()?)),

                // Array literal
                TokenType::OpenSquareBracket(_) => Ok(ASTNodeExpression::ArrayLiteral(self.parse_array_literal()?)),

                // Variable
                // TODO: error on reserved words
                // TODO: this could be a function expression
                TokenType::Identifier(i) => Ok(ASTNodeExpression::Variable(Rc::new(RefCell::new(ASTNodeVariable{
                    location: t.location.clone(),
                    identifier: i.clone()
                })))),

                // Value literal
                TokenType::ValueLiteral(v) => Ok(ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    value: v.clone()
                })))),
                
                _ => {
                    let e = ParseErrorType::ExpectedExpression { found: Some(t.token_type.to_str()) };
                    Err(self.get_error(e))
                }
            }
        }
    }

    fn parse_expression(&mut self, precedence: usize) -> Result<ASTNodeExpression, ParseError> {

        match precedence {
            // Base case - just return a value
            precedences::GROUPING =>  {
                self.parse_value()
            }
            // Precedence of '.', '?.', '[...]', new with argument list, function call
            precedences::MEMBER_ACCESS | precedences::NEW_WITHOUT_ARGUMENT_LIST => {
                // Stores the locations of 'new' operators
                // This disambiguates between a function call and new with argument list
                let mut new_stack = vec![];
                // Consume all 'new' tokens
                'init_new_stack: loop {
                    // Get a token
                    let Some(t) = self.get_token() else {
                        return Err(self.get_error(ParseErrorType::UnexpectedEOF));
                    };
                    // If it's not an identifier, break the loop
                    let Token {token_type: TokenType::Identifier(i), location, ..} = t else {
                        break 'init_new_stack;
                    };
                    // If it's not 'new', break the loop
                    if i != "new" {
                        break 'init_new_stack;
                    }
                    new_stack.push(location.clone());
                }

                // Don't consume the token which broke the loop
                self.i -= 1;

                let mut val = self.parse_expression(precedences::GROUPING)?;
                'parse_tokens: loop {
                    match self.get_token() {
                        None => break 'parse_tokens,
                        Some(t) => match &t.token_type {
                            // Property lookup
                            TokenType::OperatorDot => {
                                let t_location = t.location.clone();
                                let Some(i) = self.get_token() else {
                                    return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                                };
                                let i = i.clone();
                                // '.' is always followed by an identifier
                                let Token{token_type:TokenType::Identifier(i), location: i_location, newline_after: _} = i else {
                                    let found = i.token_type.to_str();
                                    return Err(self.get_error(ParseErrorType::UnexpectedToken { found, expected: Some("identifier") }))
                                };
                                val = ASTNodeExpression::PropertyLookup(Rc::new(
                                    RefCell::new(ASTNodePropertyLookup {
                                        location: t_location,
                                        lhs: val,
                                        rhs: ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(
                                            ASTNodeValueLiteral {
                                                location: i_location,
                                                value: ValueLiteral::String(i.clone())
                                            }
                                        ))),
                                        optional: false
                                    })
                                ));
                            }
                            // Optional chaining
                            TokenType::OperatorOptionalChaining => {
                                let t_location = t.location.clone();
                                let Some(i) = self.get_token() else {
                                    return Err(self.get_error(ParseErrorType::UnexpectedEOF))
                                };
                                let i = i.clone();

                                let i_location = i.location.clone();

                                let rhs = match i.token_type {

                                    // Optional chained function call 'a?.()'
                                    TokenType::OpenParen(_) => {
                                        let args = self.parse_function_args()?;
                                        val = ASTNodeExpression::FunctionCall(Rc::new(
                                            RefCell::new(ASTNodeFunctionCall {
                                                location: i_location,
                                                function: val,
                                                args,
                                                optional: true,
                                            })
                                        ));
                                        continue 'parse_tokens;
                                    }

                                    // Property lookup 'a?.b'
                                    TokenType::Identifier(id) => ASTNodeExpression::ValueLiteral(Rc::new(RefCell::new(
                                        ASTNodeValueLiteral {
                                            location: i_location,
                                            value: ValueLiteral::String(id.clone())
                                        }
                                    ))),
                                    // Computed property lookup 'a?.["b"]'
                                    TokenType::OpenSquareBracket(i) => {
                                        let e = self.parse_expression(precedences::ANY_EXPRESSION)?;
                                        
                                        // Check that the end square bracket is the right one
                                        assert_eq!(i, self.i);
                                        self.i += 1;
                                        
                                        e
                                    }
                                    _ => return Err(self.get_error(ParseErrorType::UnexpectedToken { found: i.token_type.to_str(), expected: Some("identifier, '[', or '('") }))
                                };


                                val = ASTNodeExpression::PropertyLookup(Rc::new(
                                    RefCell::new(ASTNodePropertyLookup {
                                        location: t_location,
                                        lhs: val,
                                        rhs,
                                        optional: true
                                    })
                                ));
                            }
                            // Computed member access
                            TokenType::OpenSquareBracket(i) => {
                                let t = t.clone();
                                let i = i.clone();
                                let e = self.parse_expression(precedences::ANY_EXPRESSION)?;
                                
                                // Check that the end square bracket is the right one
                                assert_eq!(i, self.i);
                                self.i += 1;

                                val = ASTNodeExpression::PropertyLookup(Rc::new(
                                    RefCell::new(ASTNodePropertyLookup{
                                        location: t.location,

                                        lhs: val,
                                        rhs: e,
                                        optional: false,
                                    })
                                ));
                            }
                            // Function call or arguments to 'new'
                            TokenType::OpenParen(_) => {
                                let t = t.clone();
                                let args = self.parse_function_args()?;

                                if let Some(location) = new_stack.pop() {
                                    val = ASTNodeExpression::FunctionCall(Rc::new(
                                        RefCell::new(ASTNodeFunctionCall { 
                                            location,
                                            function: val,
                                            args,
                                            optional: false
                                        })
                                    ));
                                }
                                else {
                                    let location = t.location.clone();
                                    val = ASTNodeExpression::FunctionCall(Rc::new(
                                        RefCell::new(ASTNodeFunctionCall { 
                                            location,
                                            function: val,
                                            args,
                                            optional: false
                                        })
                                    ));
                                }
                            }
                        
                            // Anything else gets passed back up
                            _ => {
                                self.i -= 1;
                                break 'parse_tokens;
                            }
                        }
                    }
                }

                for location in new_stack.into_iter().rev() {
                    val = ASTNodeExpression::New(Rc::new(RefCell::new(ASTNodeNew {
                        location,
                        function: val,
                        args: None
                    })))
                }

                Ok(val)
            }

            precedences::ANY_EXPRESSION..=precedences::GROUPING => {self.parse_expression(precedence + 1)}

            _ => panic!("Precedence too high")

        }

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

        let value = self.parse_expression(0)?;

        let l = Rc::new(
            RefCell::new(ASTNodeLetExpression {
                location,
                lhs: pattern,
                rhs: value,
            })
        );

        // Consume semicolon or newline
        self.get_token();

        Ok(Some(l))
    }

    fn parse_statement(&mut self) -> Result<Option<ASTNodeStatement>, ParseError> {
        match self.get_token() {
            None => Err(self.get_error(ParseErrorType::UnexpectedEOF)),
            Some(t) => {

                // Parse block
                if let TokenType::OpenBrace(close_index) = t.token_type  {
                    let b = self.parse_statements()?;

                    // Make sure the end of the block was reached
                    assert_eq!(close_index, self.i);
                    self.i += 1;

                    return Ok(Some(ASTNodeStatement::Block(b)));
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
                        "var" => todo!(),
                        "if" => todo!(),
                        "while" => todo!(),
                        "for" => todo!(),
                        "function" => todo!(),

                        _ => ()
                    }
                }

                todo!("{:?} as start of statement", t_clone);

            }
        }
    }

    fn parse_statements(&mut self) -> Result<Rc<RefCell<ASTNodeBlock>>, ParseError> {
        let block = Rc::new(RefCell::new(ASTNodeBlock {
            location: self.get_location(),

            statements: vec![]
        }));
        
        let mut block_ref = block.borrow_mut();

        'statements: loop {
            self.next_statement();

            if self.i == self.tokens.len() {
                break 'statements;
            }

            let statement = self.parse_statement()?;

            let Some(statement) = statement else {
                break 'statements;
            };

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

        let block = s.parse_statements()?;

        let parsed_program = Rc::new(RefCell::new(ASTNodeProgram {
            program,
            block: block.clone(),
        }));

        Ok(parsed_program)
    }
}