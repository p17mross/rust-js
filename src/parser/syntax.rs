use super::*;

impl Parser {
    pub(super) fn parse_destructuring_assignment_target(
        &mut self,
    ) -> Result<DestructuringAssignmentTarget, ParseError> {
        Err(self.get_error(ParseErrorType::SyntaxError))
    }

    pub(super) fn parse_function_args(&mut self) -> Result<Vec<FunctionCallArgument>, ParseError> {
        let Token{token_type: TokenType::OpenParen(start_bracket_index), ..} = self.tokens[self.i - 1] else {panic!()};

        let mut args = vec![];

        'parse_args: loop {
            let t = self
                .tokens
                .get(self.i)
                .expect("Mismatched brackets should have been caught in the lexer");
            match t.token_type {
                TokenType::CloseParen(_) => {
                    debug_assert_eq!(start_bracket_index, self.i);

                    self.i += 1;
                    break 'parse_args;
                }
                TokenType::OperatorSpread => {
                    let location = t.location.clone();
                    self.i += 1;

                    args.push(FunctionCallArgument {
                        location,
                        expression: self.parse_expression(precedences::COMMA + 1)?,
                        spread: true,
                    });
                }
                _ => {
                    let location = t.location.clone();

                    args.push(FunctionCallArgument {
                        location,
                        expression: self.parse_expression(precedences::COMMA + 1)?,
                        spread: false,
                    });
                }
            }

            let t = self.get_token();
            match t.token_type {
                TokenType::Comma => (),
                TokenType::CloseParen(_) => self.i -= 1,
                _ => {
                    let t_type = t.token_type.to_str();
                    return Err(self.get_error(ParseErrorType::UnexpectedToken {
                        found: t_type,
                        expected: Some(","),
                    }));
                }
            }
        }

        Ok(args)
    }
}
