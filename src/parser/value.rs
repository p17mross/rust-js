use super::*;

use super::ast::expressions::{
    ASTNodeValueLiteral, ArrayItem, ArrayLiteral, Expression, ObjectLiteral, ObjectLiteralProperty,
    Variable,
};

impl Parser {
    /// Parse an array literal
    fn parse_array_literal(
        &mut self,
        open_square_bracket_location: ProgramLocation,
    ) -> Result<ArrayLiteral, ParseError> {
        let mut items = vec![];

        'array_items: loop {
            let t = &self.tokens[self.i];
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
                    let comma = self.get_token();

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
                    let comma = self.get_token();

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

    /// Checks the token after a property
    /// If it is a [comma][TokenType::Comma], increment `self.i`
    /// If it is a [close brace][TokenType::CloseBrace], do nothing
    /// Anything else, return an ['expected property' error][ParseErrorType::ExpectedProperty]
    fn check_token_between_properties(&mut self) -> Result<(), ParseError> {
        match self.tokens[self.i].token_type {
            // Comma between items
            TokenType::Comma => {
                self.i += 1;
                Ok(())
            }
            // Close brace for the end of the object literal
            TokenType::CloseBrace(_) => Ok(()),
            // Anything else is an error
            _ => Err(self.get_error(ParseErrorType::ExpectedProperty))
        }
    }

    /// Parse an object literal
    fn parse_object_literal(&mut self) -> Result<ObjectLiteral, ParseError> {
        let location = self.get_location();
        let mut properties = Vec::new();

        'properties: loop {
            let t = self.get_token();

            match t.token_type {
                // The end of the object literal
                TokenType::CloseBrace(_) => break 'properties,
                
                // A normal 'a: b' item or a shortHand property 'a' or a method  
                TokenType::Identifier(_) | TokenType::ValueLiteral(_) => {
                    let (k, can_be_shorthand) = match &t.token_type {
                        TokenType::Identifier(k) => (k.clone(), true),
                        TokenType::ValueLiteral(ValueLiteral::String(k)) => (k.clone(), false),
                        TokenType::ValueLiteral(ValueLiteral::Number(k)) => (k.to_string(), false),
                        TokenType::ValueLiteral(ValueLiteral::BigInt(k)) => (k.to_string(), false),
                        _ => unreachable!(),
                    };

                    match self.get_token().token_type {
                        // A property
                        TokenType::OperatorColon => (),
                        // A shorthand property
                        TokenType::Comma if can_be_shorthand => {
                            properties.push(ObjectLiteralProperty::KeyOnly(k));
                            continue 'properties;
                        }
                        TokenType::CloseBrace(_) if can_be_shorthand => {
                            properties.push(ObjectLiteralProperty::KeyOnly(k));
                            break 'properties;
                        }
                        // A method
                        TokenType::OpenParen(_) => {
                            todo!("Methods in object literals");
                        }
                        // A getter or setter
                        TokenType::Identifier(_) if k == "get" || k == "set" => {
                            todo!("Getters and setters in object literals");
                        }
                        // Anything else is an error
                        _ => {
                            return Err(self.get_error(ParseErrorType::ExpectedProperty));
                        }
                    }

                    let v = self.parse_expression(precedences::COMMA + 1)?;

                    // Check the next token
                    self.check_token_between_properties()?;

                    properties.push(ObjectLiteralProperty::KeyValue(k, v));
                }

                // A computed property
                TokenType::OpenSquareBracket(close_square_bracket_index) => {
                    let k = self.parse_expression(precedences::COMMA + 1)?;

                    // Check that the next token is a close square bracket
                    let TokenType::CloseSquareBracket(_) = self.get_token().token_type else {
                        return Err(self.get_error(ParseErrorType::ExpectedProperty));
                    };
                    debug_assert_eq!(close_square_bracket_index, self.i - 1);

                    // Check that the next token is a colon
                    if self.get_token().token_type != TokenType::OperatorColon {
                        return Err(self.get_error(ParseErrorType::ExpectedProperty));
                    };

                    let v = self.parse_expression(precedences::COMMA + 1)?;
                    properties.push(ObjectLiteralProperty::Computed(k, v));
                }

                // A spread
                TokenType::OperatorSpread => {
                    let e = self.parse_expression(precedences::COMMA + 1)?;

                    // Check the next token
                    self.check_token_between_properties()?;

                    properties.push(ObjectLiteralProperty::Spread(e));

                }

                _ => return Err(self.get_error(ParseErrorType::ExpectedProperty)),
            };
        }

        Ok(ObjectLiteral {
            location,
            properties,
        })
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
