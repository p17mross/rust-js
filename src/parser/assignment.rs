use super::*;

impl Parser {
    /// Parses an expression with precedence 2.\
    /// Any operator with a lower precedence will be ignored for the caller to parse.
    pub(super) fn parse_assignment(&mut self) -> Result<ASTNodeExpression, ParseError> {
        let pattern_start = self.i;

        let e = self.parse_expression(precedences::ASSIGNMENT + 1);

        let target = match e {
            Ok(e) => {
                match self.tokens.get(self.i) {
                    None => return Ok(e),
                    Some(t) => match t.token_type {
                        TokenType::UpdateAssignment(_) | TokenType::OperatorAssignment => match e {
                            ASTNodeExpression::Variable(v) => ASTNodeDestructuringAssignmentTarget::Variable(v.identifier),
                            ASTNodeExpression::PropertyLookup(p) => ASTNodeDestructuringAssignmentTarget::PropertyLookup { expression: p.lhs, property: p.rhs },
                            ASTNodeExpression::ObjectLiteral(_) => self.parse_destructuring_assignment_target()?,
                            ASTNodeExpression::ArrayLiteral(_) => self.parse_destructuring_assignment_target()?,
                            _ => return Err(self.get_error(ParseErrorType::InvalidAssignmentLHS))
                        }
                        _ => return Ok(e),
                    }
                }
            }
            Err(err) => {
                self.i = pattern_start;
                let t = self.parse_destructuring_assignment_target();

                match self.tokens.get(self.i) {
                    None => return Err(err),
                    Some(token) => match token.token_type {
                        TokenType::UpdateAssignment(_) | TokenType::OperatorAssignment => t.map_err(|_|err)?,
                        _ => return Err(err),
                    }
                }
            }
        };

        let assignment_expression = match self.tokens.get(self.i) {
            None => unreachable!(),
            Some(t) => match t.token_type {
                TokenType::OperatorAssignment => {
                    let location = t.location.clone();
                    self.i += 1;

                    let rhs = self.parse_expression(precedences::ASSIGNMENT)?;

                    ASTNodeExpression::Assignment(Box::new(ASTNodeAssignment {
                        location,
                        lhs: target, 
                        rhs
                    }))
                }
                TokenType::UpdateAssignment(operator_type) => {
                    let location = t.location.clone();
                    self.i += 1;

                    let lhs = match target {
                        ASTNodeDestructuringAssignmentTarget::Variable(v) => ASTNodeAssignmentTarget::Variable(v),
                        ASTNodeDestructuringAssignmentTarget::PropertyLookup { expression, property } => ASTNodeAssignmentTarget::PropertyLookup { expression, property },
                        _ => return Err(self.get_error(ParseErrorType::InvalidDestructuringAssignmentOperator))
                    };

                    let rhs = self.parse_expression(precedences::ASSIGNMENT)?;

                    ASTNodeExpression::UpdateAssignment(Box::new(ASTNodeUpdateAssignment {
                        location,
                        operator_type,
                        lhs,
                        rhs
                    }))
                },
                _ => unreachable!()
            }
        };

        Ok(assignment_expression)
    }

}