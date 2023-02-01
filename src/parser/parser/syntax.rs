use super::*;

impl Parser {
    pub(super) fn parse_pattern(&mut self) -> Result<ASTNodePattern, ParseError> {
        let t = self.try_get_token()?;

        match &t.token_type {
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

    pub(super) fn parse_function_args(&mut self) -> Result<Vec<FunctionCallArgument>, ParseError> {
        let t = self.get_token();
        match t.token_type {
            TokenType::CloseParen(_) => return Ok(vec![]),
            _ => todo!("Parsing function args")
        }
    }
}