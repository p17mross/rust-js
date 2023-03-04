//! The [`parse_statement`][Parser::parse_statement] method

use crate::lexer::{Token, TokenType};

use super::*;

use super::ast::statements::{Block, LetExpression, Statement};

impl Parser {
    /// Consumes any [semicolon][TokenType::Semicolon] tokens\
    /// Returns true if the end of the file or a [closing brace][TokenType::CloseBrace] token was reached.
    fn next_statement(&mut self) -> bool {
        // Loop over the tokens
        loop {
            match self.tokens.get(self.i) {
                // EOF or close brace means the end of the block has been reached, so return true
                None
                | Some(Token {
                    token_type: TokenType::CloseBrace(_),
                    ..
                }) => {
                    return true;
                }
                // Semicolon - keep looping
                Some(Token {
                    token_type: TokenType::Semicolon,
                    ..
                }) => {
                    self.i += 1;
                }
                // Any other token means next statement has been reached so return false
                Some(_) => {
                    return false;
                }
            }
        }
    }

    /// Parses a `let a = b` binding
    fn parse_let_expression(&mut self) -> Result<LetExpression, ParseError> {
        // Save location for later
        let location = self.get_location();

        let pattern = self.parse_destructuring_assignment_target()?;

        // TODO: let bindings can have no expression e.g. `let a;`
        // TODO: let bindings can have multiple statements e.g. `let a = 0, b = 1;`

        let t = self.try_get_token().cloned()?;

        if t.token_type != TokenType::OperatorAssignment {
            return Err(self.get_error(ParseErrorType::UnexpectedToken {
                found: t.token_type.to_str(),
                expected: Some("="),
            }));
        };

        let value = self.parse_expression(precedences::COMMA + 1)?;

        let l = LetExpression {
            location,
            lhs: pattern,
            rhs: value,
        };

        // Consume semicolon or newline
        self.i += 1;

        Ok(l)
    }

    /// Parses a statement, for example a let binding or an if statement.\
    /// If the statement is an expression, it will be parsed in this function too.
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let t = self.try_get_token()?.clone();

        // Some = successfully parsed statement
        // None = try to parse expression
        let statement = match t.token_type {
            TokenType::OpenBrace(close_index) => {
                let b = self.parse_statements(None)?;

                // Make sure the end of the block was reached
                debug_assert_eq!(close_index, self.i);
                self.i += 1;

                Some(Statement::Block(Box::new(b)))
            }
            TokenType::Identifier(i) => {
                match i.as_str() {
                    // This could be a let declaration, but it can also be an expression starting with the identifier 'let'
                    "let" => {
                        match self.tokens.get(self.i) {
                            // A let binding
                            Some(Token {
                                // With a token_type of:
                                token_type: TokenType::Identifier(_) // Identifier: a single variable let binding e.g. 'let a = b;'
                                | TokenType::OpenBrace(_) // An object destructuring e.g. 'let {a} = {a: 10};'
                                | TokenType::OpenSquareBracket(_), ..} // An array destructure e.g. 'let [a, b] = [10, 20];' 
                            ) => Some(Statement::LetExpression(Box::new(self.parse_let_expression()?))), // Don't do anything - just keep parsing as a let expression
    
                            // Anything else is just an expression
                            _ => None,
                        }
                    }
                    "var" => todo!(),
                    "const" => todo!(),

                    "if" => todo!(),
                    "else" => todo!(),

                    "while" => todo!(),
                    "for" => todo!(),
                    "in" => todo!(),
                    "do" => todo!(),
                    "break" => todo!(),
                    "continue" => todo!(),

                    "switch" => todo!(),
                    "case" => todo!(),
                    "default" => todo!(),

                    "try" => todo!(),
                    "catch" => todo!(),
                    "finally" => todo!(),

                    "function" => todo!(),
                    "return" => todo!(),
                    "class" => todo!(),
                    "extends" => todo!(),

                    "export" => todo!(),
                    "import" => todo!(),

                    "with" => todo!(),

                    _ => None,
                }
            }

            // No statement - parse as expression
            _ => None,
        };

        // Get a statement either straight from Some or by parsing an expression if None
        let statement = if let Some(s) = statement {
            s
        } else {
            self.i -= 1;
            Statement::Expression(self.parse_expression(precedences::ANY_EXPRESSION)?)
        };

        // Get last token of this statement and one after to parse semicolon insertion
        let this_t = self
            .tokens
            .get(self.i - 1)
            .expect("Should have been Some as this token was just parsed");
        let next_t = self.tokens.get(self.i);

        // For this to be a valid statement, either the next token has to be a semicolon or the end of a block
        // Or this token has to have a newline after it
        match next_t {
            // End of the program
            None => Ok(statement),
            Some(next_t) => match next_t.token_type {
                // Semicolon or end of block
                TokenType::Semicolon | TokenType::CloseBrace(_) => Ok(statement),
                // Semicolon insertion
                _ if this_t.newline_after => Ok(statement),
                // Otherwise there are two statements on the same line, so error
                _ => Err(self.get_error(ParseErrorType::UnexpectedToken {
                    found: next_t.token_type.to_str(),
                    expected: Some(";"),
                })),
            },
        }
    }

    /// Parses a sequence of statements.\
    /// Ends on EOF or on a [closing brace][TokenType::CloseBrace], which is not consumed.
    /// The given location will be used if it is `Some`, or the location of the current token if it is `None`.
    pub(super) fn parse_statements(
        &mut self,
        location: Option<ProgramLocation>,
    ) -> Result<Block, ParseError> {
        let mut block = Block {
            location: location.unwrap_or_else(|| self.get_location()),
            statements: vec![],
        };

        'statements: loop {
            // Break on close brace or EOF
            let should_break = self.next_statement();
            if should_break {
                break 'statements;
            }

            // Parse a statement
            let statement = self.parse_statement()?;
            block.statements.push(statement);
        }

        Ok(block)
    }
}
