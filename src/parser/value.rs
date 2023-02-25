use super::*;

impl Parser {
    /// Parses an array literal
    fn parse_array_literal(
        &mut self,
        open_square_bracket_location: ProgramLocation,
    ) -> Result<ArrayLiteral, ParseError> {
        let mut items = vec![];

        'array_items: loop {
            let t = self
                .tokens
                .get(self.i)
                .expect("Mismatched brackets should have been caught by the lexer");
            match t.token_type {
                // The end of the array
                TokenType::CloseSquareBracket(_) => {
                    self.i += 1;
                    break 'array_items;
                }
                // An empty slot
                TokenType::Comma => {
                    self.i += 1;
                    items.push(ArrayItem::Empty(t.location.clone()));
                }
                // A spread
                TokenType::OperatorSpread => {
                    self.i += 1;
                    let e = self.parse_expression(precedences::COMMA + 1)?;
                    let comma = self
                        .try_get_token()
                        .expect("Mismatched brackets should have been caught by the lexer");

                    match comma.token_type {
                        TokenType::Comma => items.push(ArrayItem::Spread(e)),
                        TokenType::CloseSquareBracket(_) => {
                            items.push(ArrayItem::Spread(e));
                            break 'array_items;
                        }
                        _ => {
                            let token_str = comma.token_type.to_str();
                            return Err(self.get_error(ParseErrorType::UnexpectedToken {
                                found: token_str,
                                expected: Some("comma"),
                            }));
                        }
                    };
                }
                // An array item
                _ => {
                    let e = self.parse_expression(precedences::COMMA + 1)?;
                    let comma = self
                        .try_get_token()
                        .expect("Mismatched brackets should have been caught by the lexer");

                    match comma.token_type {
                        TokenType::Comma => items.push(ArrayItem::Item(e)),
                        TokenType::CloseSquareBracket(_) => {
                            items.push(ArrayItem::Item(e));
                            break 'array_items;
                        }
                        _ => {
                            let token_str = comma.token_type.to_str();
                            return Err(self.get_error(ParseErrorType::UnexpectedToken {
                                found: token_str,
                                expected: Some("comma"),
                            }));
                        }
                    };
                }
            }
        }

        let array = ArrayLiteral {
            location: open_square_bracket_location,
            items,
        };

        Ok(array)
    }

    fn parse_object_literal(&mut self) -> Result<ObjectLiteral, ParseError> {
        todo!("Parsing object literals")
    }

    pub(super) fn parse_value(&mut self) -> Result<Expression, ParseError> {
        let t = self.try_get_token()?;
        match &t.token_type {
            // Bracketed expression
            TokenType::OpenParen(close_index) => {
                let close_index = *close_index;

                let e = self.parse_expression(0)?;

                debug_assert_eq!(close_index, self.i);
                self.i += 1;

                Ok(e)
            }

            // Close paren
            // This is always a syntax error as if this will only occur with an empty set of parens
            TokenType::CloseParen(_) => {
                Err(self.get_error(ParseErrorType::ExpectedExpression { found: Some(")") }))
            }

            // Object literal
            // This cannot be a block as they are not allowed inside expressions
            TokenType::OpenBrace(_) => Ok(Expression::ObjectLiteral(Box::new(
                self.parse_object_literal()?,
            ))),

            // Array literal
            TokenType::OpenSquareBracket(close_square_bracket_index) => {
                let close_square_bracket_index = *close_square_bracket_index;

                let open_square_bracket_location = t.location.clone();
                let e = self.parse_array_literal(open_square_bracket_location)?;

                debug_assert_eq!(close_square_bracket_index, self.i - 1);
                Ok(Expression::ArrayLiteral(Box::new(e)))
            }

            // Variable
            // TODO: error on reserved words
            // TODO: this could be a function expression
            TokenType::Identifier(i) => Ok(Expression::Variable(Box::new(Variable {
                location: t.location.clone(),
                identifier: i.clone(),
            }))),

            // Value literal
            TokenType::ValueLiteral(v) => {
                Ok(Expression::ValueLiteral(Box::new(ASTNodeValueLiteral {
                    location: t.location.clone(),
                    value: v.clone(),
                })))
            }

            _ => {
                let e = ParseErrorType::ExpectedExpression {
                    found: Some(t.token_type.to_str()),
                };
                Err(self.get_error(e))
            }
        }
    }
}
