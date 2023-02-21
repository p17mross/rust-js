use super::*;


impl Parser {
    fn parse_array_literal(&mut self) -> Result<ArrayLiteral, ParseError> {
        todo!("Array literals")
    }

    fn parse_object_literal(&mut self) -> Result<ObjectLiteral, ParseError> {
        todo!("Parsing object literals")
    }

    pub(super) fn parse_value(&mut self) -> Result<Expression, ParseError> {
        let t = self.try_get_token()?;
        match &t.token_type {
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
            TokenType::OpenBrace(_) => Ok(Expression::ObjectLiteral(Box::new(self.parse_object_literal()?))),

            // Array literal
            TokenType::OpenSquareBracket(_) => Ok(Expression::ArrayLiteral(Box::new(self.parse_array_literal()?))),

            // Variable
            // TODO: error on reserved words
            // TODO: this could be a function expression
            TokenType::Identifier(i) => Ok(Expression::Variable(Box::new(Variable{
                location: t.location.clone(),
                identifier: i.clone()
            }))),

            // Value literal
            TokenType::ValueLiteral(v) => Ok(Expression::ValueLiteral(Box::new(ASTNodeValueLiteral {
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