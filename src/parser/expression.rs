use super::*;

impl Parser {
    /// Parses an expression with precedence 16 or 17.\
    /// Any operator with a lower precedence will be ignored for the caller to parse.
    fn parse_assignment_target_or_new(&mut self) -> Result<Expression, ParseError> {
        // Stores the locations of 'new' operators
        // This disambiguates between a function call and new with argument list
        let mut new_stack = vec![];
        // Consume all 'new' tokens
        'init_new_stack: loop {
            // Get a token
            let maybe_new = self.try_get_token()?;
            // If it's not an identifier, break the loop
            let Token {token_type: TokenType::Identifier(identifier), location, ..} = maybe_new else {
                break 'init_new_stack;
            };
            // If it's not 'new', break the loop
            if identifier != "new" {
                break 'init_new_stack;
            }
            new_stack.push(location.clone());
        }

        // Don't consume the token which broke the loop
        self.i -= 1;

        // Parse an initial value on which to apply operators
        let mut val = self.parse_expression(precedences::GROUPING)?;

        'parse_tokens: loop {
            let Ok(operator_token) = self.try_get_token() else {break 'parse_tokens};
            match operator_token.token_type.clone() {
                // Property lookup
                TokenType::OperatorDot => {
                    let operator_token_location = operator_token.location.clone();
                    val = self.parse_property_lookup(operator_token_location, val)?;
                },
                // Optional chaining
                TokenType::OperatorOptionalChaining => {
                    let operator_token_location = operator_token.location.clone();
                    val = self.parse_optional_chaining(operator_token_location, val)?;
                }
                // Computed member access
                TokenType::OpenSquareBracket(close_square_bracket_index) => {
                    let operator_token_location = operator_token.location.clone();
                    val = self.parse_computed_member_access(operator_token_location, val)?;
                    debug_assert_eq!(close_square_bracket_index, self.i - 1);
                }
                // Function call or arguments to 'new'
                TokenType::OpenParen(close_paren_index) => {
                    let operator_token_location = operator_token.location.clone();
                    val = self.parse_function_call_or_new(operator_token_location, &mut new_stack, val)?;
                    debug_assert_eq!(close_paren_index, self.i - 1);
                },
            
                // Anything else gets passed back up
                _ => {
                    self.i -= 1;
                    break 'parse_tokens;
                }
            }
            
        }

        for location in new_stack.into_iter().rev() {
            val = Expression::FunctionCall(Box::new(FunctionCall {
                location,
                function: val,
                args: Vec::new(),
                call_type: FunctionCallType::New,
            }));
        }

        Ok(val)
    }

    /// Parse a [`FunctionCall`]
    /// 
    /// ### Params
    /// * `operator_token_location`: the location of the [open paren][TokenType::OpenParen] token
    /// * `new_stack`: the stack of locations of `new` tokens. The top value of the stack will be popped and a [new][FunctionCallType::New] will be called.
    ///     If the stack is empty, a [function call][FunctionCallType::FunctionCall] will be returned.
    /// * `val`: the expression to be the left hand side of the call expression
    fn parse_function_call_or_new(&mut self, operator_token_location: ProgramLocation, new_stack: &mut Vec<ProgramLocation>, val: Expression) -> Result<Expression, ParseError> {
        let args = self.parse_function_args()?;

        // Get the location and call type based on whether there is something on the call stack
        let (location, call_type) = match new_stack.pop() {
            Some(location) => (location, FunctionCallType::New),
            None => (operator_token_location, FunctionCallType::FunctionCall)
        };

        let val = Expression::FunctionCall(Box::new(FunctionCall { 
            location,
            function: val,
            args,
            call_type,
        }));
        

        Ok(val)
    }

    /// Parse a [`PropertyLookup`] using the computed member access syntax (`a["some property"]`)
    /// 
    /// ### Params
    /// * `operator_token_location`: the location of the [open square bracket][TokenType::OpenSquareBracket] token
    /// * `val`: the expression to be the left hand side of the property lookup
    fn parse_computed_member_access(&mut self, operator_token_location: ProgramLocation, val: Expression) -> Result<Expression, ParseError> {
        let computed_expression = self.parse_expression(precedences::ANY_EXPRESSION)?;
        
        self.i += 1;
        
        let val = Expression::PropertyLookup(Box::new(PropertyLookup {
            location: operator_token_location,

            lhs: val,
            rhs: computed_expression,
            optional: false,
        }));

        Ok(val)
    }

    /// Parse a [`PropertyLookup`] using the member access syntax (`a.some_property`)
    /// 
    /// ### Params
    /// * `operator_token_location`: the location of the [dot][TokenType::OperatorDot] token
    /// * `val`: the expression to be the left hand side of the property lookup
    fn parse_property_lookup(&mut self, operator_token_location: ProgramLocation, mut val: Expression) -> Result<Expression, ParseError> {
        let argument_token = self.try_get_token()?.clone();
        
        // Get the property name
        let Token{token_type:TokenType::Identifier(identifier), location: argument_token_location, ..} = argument_token else {
            let found = argument_token.token_type.to_str();
            return Err(self.get_error(ParseErrorType::UnexpectedToken { found, expected: Some("identifier") }))
        };
        
        val = Expression::PropertyLookup(Box::new(PropertyLookup {
            location: operator_token_location,
            lhs: val,
            rhs: Expression::ValueLiteral(Box::new(
                ASTNodeValueLiteral {
                    location: argument_token_location,
                    value: ValueLiteral::String(identifier)
                }
            )),
            optional: false
        }));

        Ok(val)
    }

    /// Parse something after the [optional chaining operator][TokenType::OperatorOptionalChaining]. This syntax can have multiple forms, so the function can return a [`FunctionCall`] or a [`PropertyLookup`].
    /// 
    /// ### Params
    /// * `operator_token_location`: the location of the [optional chaining operator][TokenType::OperatorOptionalChaining]
    /// * `val`: the lhs of the parsed expression
    fn parse_optional_chaining(&mut self, operator_token_location: ProgramLocation, val: Expression) -> Result<Expression, ParseError> {
        let argument_token = self.try_get_token()?.clone();
        let argument_token_location = argument_token.location.clone();

        let rhs = match argument_token.token_type {

            // Optional chained function call 'a?.()'
            TokenType::OpenParen(_) => {
                let args = self.parse_function_args()?;
                let val = Expression::FunctionCall(Box::new(FunctionCall {
                        location: argument_token_location,
                        function: val,
                        args,
                        call_type: FunctionCallType::OptionalChainedFunctionCall,
                    }
                ));
                return Ok(val);
            }

            // Property lookup 'a?.b'
            TokenType::Identifier(id) => Expression::ValueLiteral(Box::new(
                ASTNodeValueLiteral {
                    location: argument_token_location,
                    value: ValueLiteral::String(id)
                }
            )),

            // Computed property lookup 'a?.["b"]'
            TokenType::OpenSquareBracket(i) => {
                let computed_expression = self.parse_expression(precedences::ANY_EXPRESSION)?;
                
                // Check that the end square bracket is the right one
                debug_assert_eq!(i, self.i);
                self.i += 1;
                
                computed_expression
            }

            // Any other token is an error
            _ => return Err(self.get_error(ParseErrorType::UnexpectedToken {
                found: argument_token.token_type.to_str(),
                expected: Some("identifier, '[', or '('")
            }))
        };

        let val = Expression::PropertyLookup(Box::new(PropertyLookup {
            location: operator_token_location,
            lhs: val,
            rhs,
            optional: true
        }));

        Ok(val)
    }

    /// Parses an expression with precedence 15 - this function parses postfix increments and decrements.\
    /// If no postfix is found, an expression will be parsed instead.\
    /// Any subsequent increment or decrement tokens wll be ignored for the caller to parse or error on.
    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let target = self.parse_expression(precedences::ASSIGNMENT_TARGET)?;

        match self.try_get_token() {
            // An increment or decrement token
            Ok(Token {token_type, location, ..})
            if token_type == &TokenType::OperatorIncrement || token_type == &TokenType::OperatorDecrement => {
                let target = match target {
                    Expression::PropertyLookup(p) => UpdateExpressionTarget::Property(p),
                    Expression::Variable(v) => UpdateExpressionTarget::Variable(v),
                    _ => return Err(self.get_error(ParseErrorType::InvalidUpdateExpressionOperand(UpdateExpressionSide::Postfix)))
                };

                let operator_type = match token_type {
                    TokenType::OperatorIncrement => UpdateExpressionOperatorType::Increment,
                    TokenType::OperatorDecrement => UpdateExpressionOperatorType::Decrement,
                    _ => panic!("Expected only '++' or '--' token due to outer match expression"),
                };

                Ok(Expression::UpdateExpression(Box::new(UpdateExpression {
                    location: location.clone(),
                    target,
                    operator_type,
                    side: UpdateExpressionSide::Postfix,
                })))
            },
            Ok(_) => {
                // Don't consume the token if it's not an increment or decrement
                self.i -= 1;
                Ok(target)
            }
            Err(_) => {
                // Don't decrement self.i as try_get_token won't hae incremented it
                Ok(target)
            },

        }
    }

    /// Parses an expression with precedence 14.\
    fn parse_unary_operator(&mut self) -> Result<Expression, ParseError> {
        let maybe_unary_operator_token = self.try_get_token()?;
        let location = maybe_unary_operator_token.location.clone();

        let op = match &maybe_unary_operator_token.token_type {
            TokenType::OperatorLogicalNot => Some(UnaryOperator::LogicalNot),
            TokenType::OperatorBitwiseNot => Some(UnaryOperator::BitwiseNot),
            TokenType::OperatorAddition => Some(UnaryOperator::Plus),
            TokenType::OperatorSubtraction => Some(UnaryOperator::Minus),
            TokenType::Identifier(identifier) => match identifier.as_str() {
                "typeof" => Some(UnaryOperator::TypeOf),
                "void" => Some(UnaryOperator::Void),
                "delete" => Some(UnaryOperator::Delete),

                // TODO: await
                _ => None,
            },
            _ => None,
        };

        // If the token is a prefix increment / decrement
        if let TokenType::OperatorIncrement | TokenType::OperatorDecrement = maybe_unary_operator_token.token_type {
            // Map the token type to an UpdateExpressionOperatorType
            let operator_type = match &maybe_unary_operator_token.token_type {
                TokenType::OperatorIncrement => UpdateExpressionOperatorType::Increment,
                TokenType::OperatorDecrement => UpdateExpressionOperatorType::Decrement,
                _ => panic!()
            };

            let target = self.parse_expression(precedences::UNARY_OPERATOR)?;
            let Ok(target) = target.try_into() else {
                return Err(self.get_error(ParseErrorType::InvalidUpdateExpressionOperand(UpdateExpressionSide::Prefix)))
            };

            return Ok(Expression::UpdateExpression(Box::new(UpdateExpression {
                location,
                target,
                operator_type,
                side: UpdateExpressionSide::Prefix,
            })))
        }

        match op {
            // If a unary operator was parsed
            Some(operator_type) => {
                let expression = self.parse_expression(precedences::UNARY_OPERATOR)?;
                Ok(Expression::UnaryOperator(Box::new(ASTNodeUnaryOperator {
                    location,
                    operator_type,
                    expression
                })))
            },
            // Otherwise, parse expression with lower precedence
            None => {
                // Token was not a unary/prefix operator, so don't consume it
                self.i -= 1;
                // Parse with one higher precedence
                self.parse_expression(precedences::UNARY_OPERATOR + 1)
            }
        }
    }

    /// Parses a series of binary operators with a given precedence
    fn parse_binary_operator(&mut self, precedence: usize) -> Result<Expression, ParseError>{
        // This unwrap should never fail as this function should only ever get called with precedences which have binary operators
        let BinaryPrecedence {associativity, operators: operators_in_precedence} = BINARY_PRECEDENCES[precedence].unwrap();

        // At each precedence, a sequence of binary operators always goes <value> (<operator> <value>) (<operator> <value>) etc
        // Stores the values
        let mut values = vec![];
        // Stores the types of operator and their locations
        let mut operators = vec![];

        loop {
            // Parse a value and then maybe an operator
            values.push(self.parse_expression(precedence + 1)?);
            match self.try_get_token() {
                // If EOF reached, break loop
                Err(_) => break,

                // A binary operator in this precedence
                Ok(Token {token_type: TokenType::BinaryOperator(b), location, ..}) if operators_in_precedence.contains(b) => {
                    operators.push((*b, location.clone()));
                },

                // Addition and subtraction are their own tokens, so check for them separately
                Ok(Token {token_type: TokenType::OperatorAddition, location, ..}) if precedence == precedences::ADDITION => {
                    operators.push((BinaryOperator::Addition, location.clone()));
                },
                Ok(Token {token_type: TokenType::OperatorSubtraction, location, ..}) if precedence == precedences::SUBTRACTION => {
                    operators.push((BinaryOperator::Subtraction, location.clone()));
                },

                // Comma operator is its own token, so check for it separately
                Ok(Token {token_type: TokenType::Comma, location, ..}) if precedence == precedences::COMMA => {
                    operators.push((BinaryOperator::Comma, location.clone()))
                }
                // Anything else means the end of this run of operators, so break the loop
                Ok(_) => {
                    // Don't consume the token
                    self.i -= 1;
                    break;
                }
            }
        };

        // If there are no operators, just return the value
        if operators.is_empty() {
            return Ok(values.remove(0))
        }

        // Collapse the values into one using the correct associativity
        match associativity {
            Associativity::LeftToRight => {

                let mut values = values.into_iter();
                let mut lhs = values.next().unwrap();

                // Iterate over value-operator pairs
                for (rhs, (operator_type, location)) in values.zip(operators) {
                    lhs = Expression::BinaryOperator(Box::new(ASTNodeBinaryOperator{
                        location,
                        operator_type,
                        lhs,
                        rhs
                    }))
                }
                Ok(lhs)
            },
            Associativity::RightToLeft => {
                // Reverse the iterator to iterate from right to left
                let mut values = values.into_iter().rev();
                let mut rhs = values.next().unwrap();

                // Iterate over value-operator pairs
                for (lhs, (operator_type, location)) in values.zip(operators) {
                    rhs = Expression::BinaryOperator(Box::new(ASTNodeBinaryOperator{
                        location,
                        operator_type,
                        lhs,
                        rhs
                    }))
                }
                Ok(rhs)
            },
        }
    }

    #[inline(always)]
    /// Recursively parses an expression with the given precedence.\
    /// This function does not parse anything, but just calls the relevant function for the given precedence.
    pub(super) fn parse_expression(&mut self, precedence: usize) -> Result<Expression, ParseError> {
        match precedence {
            // Base case - parse a value
            18 =>  {
                self.parse_value()
            }
            // '.', '?.', '[...]', new with argument list, function call
            17 | 16 => self.parse_assignment_target_or_new(),

            // Postfix increment and decrement
            15 => self.parse_postfix(),

            // Unary operators, prefix increment and decrement
            14 => self.parse_unary_operator(),

            // Precedences 13 down to 3 have only binary operators
            3 ..= 13 => self.parse_binary_operator(precedence),

            // Assignment operators
            2 => self.parse_assignment(),

            // Precedence 1 is the comma operator, which is a binary operator
            1 => self.parse_binary_operator(precedence),

            precedences::ANY_EXPRESSION => self.parse_expression(1),

            _ => panic!("Precedence too high")

        }

    }
}