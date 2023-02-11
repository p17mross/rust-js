use super::*;

impl Parser {
    pub(super) fn parse_destructuring_assignment_target(&mut self) -> Result<ASTNodeDestructuringAssignmentTarget, ParseError> {
        todo!("Destructuring assignment");
    }

    pub(super) fn parse_function_args(&mut self) -> Result<Vec<FunctionCallArgument>, ParseError> {
        let t = self.get_token();
        match t.token_type {
            TokenType::CloseParen(_) => return Ok(vec![]),
            _ => todo!("Parsing function args")
        }
    }
}