pub mod ast;
mod operator_precedence;

use std::fmt::Display;

use crate::{lexer::{Token, TokenType, token::ValueLiteral}, engine::{Gc, program::ProgramLocation, Program}, parser::operator_precedence::precedences};

use self::ast::*;

#[derive(Debug, Clone)]
/// All the types of errors that can occur during parsing
pub enum ParseErrorType {
    UnexpectedToken{found: &'static str, expected: Option<&'static str>},
    UnexpectedEOF,
    ExpectedExpression{found: Option<&'static str>},
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
        // Don't increment i past the end of the list
        if t.is_some() {self.i += 1;}
        t
    }

    /// Consumes any [TokenType::Semicolon] tokens\
    /// Returns true if the end of the file or a [TokenType::CloseBrace] token was reached.
    fn next_statement(&mut self) -> bool {
        loop {
            let Some(t) = self.tokens.get(self.i) else {return true};
            let Token {token_type, ..} = t;
            match token_type {
                TokenType::Semicolon => {
                    self.i += 1;
                    continue
                },
                TokenType::CloseBrace(_) => return true,
                _ => return false,
            }
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

    fn parse_pattern(&mut self) -> Result<ASTNodePattern, ParseError> {
        match self.get_token() {
            None => {
                Err(self.get_error(ParseErrorType::UnexpectedEOF))
            },
            Some(t) => match &t.token_type {
                // Just a variable
                TokenType::Identifier(i) => Ok(ASTNodePattern {
                    location: t.location.clone(), 
                    target: ASTNodePatternType::Variable(i.clone())
                }),
                // Array destructure
                TokenType::OpenSquareBracket(_) => todo!("Array destructure"),
                // Object destructure
                TokenType::OpenBrace(_) => todo!("Object destructure"),
                
                t => {
                    let t = t.clone();
                    Err(self.get_error(ParseErrorType::UnexpectedToken { found: t.to_str(), expected: Some("Pattern") }))
                },
            }
        }
    }

    fn parse_array_literal(&mut self) -> Result<ASTNodeArrayLiteral, ParseError> {
        todo!("Array literals")
    }

    fn parse_object_literal(&mut self) -> Result<ASTNodeObjectLiteral, ParseError> {
        todo!("Parsing object literals")
    }

    fn parse_function_args(&mut self) -> Result<Vec<FunctionCallArgument>, ParseError> {
        let t = self.get_token().expect("Unclosed bracket should have been caught in the lexer");
        match t.token_type {
            TokenType::CloseParen(_) => return Ok(vec![]),
            _ => todo!("Parsing function args")
        }
    }

    fn parse_value(&mut self) -> Result<ASTNodeExpression, ParseError> {
        match self.get_token() {
            None => Err(self.get_error(ParseErrorType::UnexpectedEOF)),
            Some(t) => match &t.token_type {
                // Bracketed expression
                TokenType::OpenParen(close_index) => {
                    let close_index = close_index.clone();

                    let e = self.parse_expression(0)?;

                    assert_eq!(close_index, self.i);
                    self.i += 1;

                    Ok(e)
                },

                // Close paren
                // This is always a syntax error as if this will only occur with an empty set of parens
                TokenType::CloseParen(_) => {
                    Err(self.get_error(ParseErrorType::ExpectedExpression { found: Some(")") }))
                },

                // Object literal
                // This cannot be a block as they are not allowed inside expressions
                TokenType::OpenBrace(_) => Ok(ASTNodeExpression::ObjectLiteral(Box::new(self.parse_object_literal()?))),

                // Array literal
                TokenType::OpenSquareBracket(_) => Ok(ASTNodeExpression::ArrayLiteral(Box::new(self.parse_array_literal()?))),

                // Variable
                // TODO: error on reserved words
                // TODO: this could be a function expression
                TokenType::Identifier(i) => Ok(ASTNodeExpression::Variable(Box::new(ASTNodeVariable{
                    location: t.location.clone(),
                    identifier: i.clone()
                }))),

                // Value literal
                TokenType::ValueLiteral(v) => Ok(ASTNodeExpression::ValueLiteral(Box::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    value: v.clone()
                }))),
                
                _ => {
                    let e = ParseErrorType::ExpectedExpression { found: Some(t.token_type.to_str()) };
                    Err(self.get_error(e))
                }
            }
        }
    }

    /// Parses an expression with precedence 16 or 17.\
    /// Any operator with a lower precedence will be ignored for the caller to parse.
    fn parse_assignment_target_or_new(&mut self) -> Result<ASTNodeExpression, ParseError> {
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
                        val = ASTNodeExpression::PropertyLookup(Box::new(ASTNodePropertyLookup {
                                location: t_location,
                                lhs: val,
                                rhs: ASTNodeExpression::ValueLiteral(Box::new(
                                    ASTNodeValueLiteral {
                                        location: i_location,
                                        value: ValueLiteral::String(i.clone())
                                    }
                                )),
                                optional: false
                            })
                        );
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
                                val = ASTNodeExpression::FunctionCall(Box::new(ASTNodeFunctionCall {
                                        location: i_location,
                                        function: val,
                                        args,
                                        optional: true,
                                    }
                                ));
                                continue 'parse_tokens;
                            }

                            // Property lookup 'a?.b'
                            TokenType::Identifier(id) => ASTNodeExpression::ValueLiteral(Box::new(
                                ASTNodeValueLiteral {
                                    location: i_location,
                                    value: ValueLiteral::String(id.clone())
                                }
                            )),
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


                        val = ASTNodeExpression::PropertyLookup(Box::new(ASTNodePropertyLookup {
                                location: t_location,
                                lhs: val,
                                rhs,
                                optional: true
                            })
                        );
                    }
                    // Computed member access
                    TokenType::OpenSquareBracket(i) => {
                        let t = t.clone();
                        let i = i.clone();
                        let e = self.parse_expression(precedences::ANY_EXPRESSION)?;
                        
                        // Check that the end square bracket is the right one
                        assert_eq!(i, self.i);
                        self.i += 1;

                        val = ASTNodeExpression::PropertyLookup(Box::new(ASTNodePropertyLookup{
                                location: t.location,

                                lhs: val,
                                rhs: e,
                                optional: false,
                            })
                        );
                    }
                    // Function call or arguments to 'new'
                    TokenType::OpenParen(_) => {
                        let t = t.clone();
                        let args = self.parse_function_args()?;

                        if let Some(location) = new_stack.pop() {
                            val = ASTNodeExpression::New(Box::new(ASTNodeNew { 
                                    location,
                                    function: val,
                                    args,
                                })
                            );
                        }
                        else {
                            let location = t.location.clone();
                            val = ASTNodeExpression::FunctionCall(Box::new(ASTNodeFunctionCall { 
                                    location,
                                    function: val,
                                    args,
                                    optional: false
                                })
                            );
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
            val = ASTNodeExpression::New(Box::new(ASTNodeNew {
                location,
                function: val,
                args: Vec::new(),
            }))
        }

        Ok(val)
    }

    /// Parses an expression with precedence 15 - this function parses postfix inrements and decrements.\
    /// If no postfix is found, an expression will be parsed instead.\
    /// Any subsequent increment or decrement tokens wll be ignored for the caller to parse or error on.
    fn parse_postfix(&mut self) -> Result<ASTNodeExpression, ParseError> {
        let a = self.parse_expression(precedences::ASSIGNMENT_TARGET)?;

        match self.get_token() {
            Some(Token {token_type, location, ..}) if token_type == &TokenType::OperatorIncrement || token_type == &TokenType::OperatorDecrement => {
                let target = match a {
                    ASTNodeExpression::PropertyLookup(p) => UpdateExpressionTarget::Property(p),
                    ASTNodeExpression::Variable(v) => UpdateExpressionTarget::Variable(v),
                    _ => return Err(self.get_error(ParseErrorType::InvalidUpdateExpressionOperand(UpdateExpressionSide::Postfix)))
                };

                let operator_type = match token_type {
                    TokenType::OperatorIncrement => UpdateExpressionOperatorType::Addition,
                    TokenType::OperatorDecrement => UpdateExpressionOperatorType::Subtraction,
                    _ => panic!("Expected only '++' or '--' token due to outer match expression"),
                };

                Ok(ASTNodeExpression::UpdateExpression(Box::new(ASTNodeUpdateExpression {
                    location: location.clone(),
                    target,
                    operator_type,
                    side: UpdateExpressionSide::Postfix,
                })))
            },

            _ => {
                // Don't consume the token if it's not an increment or decrement
                self.i -= 1;
                Ok(a)
            },

        }
    }


    #[inline]
    /// Recursively parses an expression with the given precedence.\
    /// This function does not parse anything, but just calls the relevent function for the given precedence.
    fn parse_expression(&mut self, precedence: usize) -> Result<ASTNodeExpression, ParseError> {

        match precedence {
            // Base case - just return a value
            precedences::GROUPING =>  {
                self.parse_value()
            }
            // '.', '?.', '[...]', new with argument list, function call
            precedences::MEMBER_ACCESS | precedences::NEW_WITHOUT_ARGUMENT_LIST => self.parse_assignment_target_or_new(),

            // Postfix increment and decrement
            precedences::POSTFIX => self.parse_postfix(),


            precedences::ANY_EXPRESSION..=precedences::GROUPING => {self.parse_expression(precedence + 1)}

            _ => panic!("Precedence too high")

        }

    }

    fn parse_let_expression(&mut self) -> Result<ASTNodeLetExpression, ParseError> {

        // Save location for later
        let location = self.get_location();

        let pattern = self.parse_pattern()?;


        let Some(t) = self.get_token().cloned() else {
            return Err(self.get_error(ParseErrorType::UnexpectedEOF));
        };

        if t.token_type != TokenType::OperatorAssignment {
            return Err(self.get_error(ParseErrorType::UnexpectedToken {found: t.token_type.to_str(), expected: Some("=")}))
        };

        let value = self.parse_expression(precedences::COMMA + 1 )?;

        let l = ASTNodeLetExpression {
            location,
            lhs: pattern,
            rhs: value,
        };

        // Consume semicolon or newline
        self.get_token();

        Ok(l)
    }

    /// Parses a statement, for example a let binding or an if statement.\
    /// ### Returns:
    /// * Ok(Some(...)) if a statement could be parsed successfully.\
    /// * Ok(None) if no statement could be parsed, but it may be parseable as an expression instead.\
    /// * Err(...) if an error is encountered while parsing.\
    fn parse_statement(&mut self) -> Result<Option<ASTNodeStatement>, ParseError> {
        
        dbg!(&self.tokens[self.i].location);
        
        match self.get_token() {
            None => Err(self.get_error(ParseErrorType::UnexpectedEOF)),
            Some(t) => {

                let t = t.clone();

                // Parse block
                if let TokenType::OpenBrace(close_index) = t.token_type  {
                    let b = self.parse_statements()?;

                    // Make sure the end of the block was reached
                    assert_eq!(close_index, self.i);
                    self.i += 1;

                    return Ok(Some(ASTNodeStatement::Block(Box::new(b))));
                }

                // TODO: remove once more token types are implemented
                let t_clone = t.token_type.clone();

                if let TokenType::Identifier(i) = &t.token_type {
                    match i.as_str() {
                        // This could be a let declaration, but it can also be an expresion starting with the identifier 'let'
                        "let" => {
                            match self.tokens.get(self.i) {
                                // A let binding
                                Some(Token {
                                    // With a token_type of:
                                    token_type: TokenType::Identifier(_) // Identifier: a single variable let binding e.g. 'let a = b;'
                                    | TokenType::OpenBrace(_) // An object destructuring e.g. 'let {a} = {a: 10};'
                                    | TokenType::OpenSquareBracket(_), ..} // An array destructure e.g. 'let [a, b] = [10, 20];' 
                                ) => (), // Don't do anything - just keep parsing as a let expression

                                // Anything else is just an expression
                                _ => {
                                    // Don't consume the 'let' token
                                    self.i -= 1;
                                    return Ok(None)
                                },
                            }

                            return Ok(Some(ASTNodeStatement::LetExpression(Box::new(self.parse_let_expression()?))));
                        },
                        "var" => todo!(),
                        "if" => todo!(),
                        "while" => todo!(),
                        "for" => todo!(),
                        "function" => todo!(),
                        "do" => todo!(),

                        _ => ()
                    }
                }

                dbg!(self.i);

                // No expression could be parsed - reset self.i and try to parse as an expression
                self.i -= 1;
                Ok(None)

            }
        }
    }

    fn parse_statements(&mut self) -> Result<ASTNodeBlock, ParseError> {
        let mut block = ASTNodeBlock {
            location: self.get_location(),

            statements: vec![]
        };

        'statements: loop {
            let should_break = self.next_statement();
            if should_break {
                break 'statements
            }

            if self.i + 1 == self.tokens.len() {
                break 'statements;
            }

            let statement = match self.parse_statement()? {
                Some(s) => s,
                None => ASTNodeStatement::Expression(self.parse_expression(precedences::ANY_EXPRESSION)?)   
            };

            block.statements.push(statement);
        }

        Ok(block)
    }

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