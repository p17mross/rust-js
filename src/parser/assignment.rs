use super::*;

use super::ast::assignment::{AssignmentTarget, DestructuringAssignmentTarget};
use super::ast::expressions::{Assignment, Expression, UpdateAssignment};

impl Parser {
    /// Parses an expression with precedence 2.\
    /// Any operator with a lower precedence will be ignored for the caller to parse.
    pub(super) fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let pattern_start = self.i;

        let e = self.parse_expression(precedences::ASSIGNMENT + 1);

        let target = match e {
            Ok(e) => match self.tokens.get(self.i) {
                None => return Ok(e),
                Some(t) => match t.token_type {
                    TokenType::UpdateAssignment(_) | TokenType::OperatorAssignment => match e {
                        Expression::Variable(v) => {
                            DestructuringAssignmentTarget::Variable(v.identifier)
                        }
                        Expression::PropertyLookup(p) => {
                            DestructuringAssignmentTarget::PropertyLookup {
                                expression: p.lhs,
                                property: p.rhs,
                            }
                        }
                        Expression::ObjectLiteral(_) | Expression::ArrayLiteral(_) => {
                            self.parse_destructuring_assignment_target()?
                        }
                        _ => return Err(self.get_error(ParseErrorType::InvalidAssignmentLHS)),
                    },
                    _ => return Ok(e),
                },
            },
            Err(err) => {
                self.i = pattern_start;
                let t = self.parse_destructuring_assignment_target();

                match self.tokens.get(self.i) {
                    None => return Err(err),
                    Some(token) => match token.token_type {
                        TokenType::UpdateAssignment(_) | TokenType::OperatorAssignment => {
                            t.map_err(|_| err)?
                        }
                        _ => return Err(err),
                    },
                }
            }
        };

        let t = self.try_get_token().unwrap();
        let assignment_expression = match t.token_type {
            TokenType::OperatorAssignment => {
                let location = t.location.clone();

                let rhs = self.parse_expression(precedences::ASSIGNMENT)?;

                Expression::Assignment(Box::new(Assignment {
                    location,
                    lhs: target,
                    rhs,
                }))
            }
            TokenType::UpdateAssignment(operator_type) => {
                let location = t.location.clone();

                let lhs = match target {
                    DestructuringAssignmentTarget::Variable(v) => AssignmentTarget::Variable(v),
                    DestructuringAssignmentTarget::PropertyLookup {
                        expression,
                        property,
                    } => AssignmentTarget::PropertyLookup {
                        expression,
                        property,
                    },
                    _ => {
                        return Err(
                            self.get_error(ParseErrorType::InvalidDestructuringAssignmentOperator)
                        )
                    }
                };

                let rhs = self.parse_expression(precedences::ASSIGNMENT)?;

                Expression::UpdateAssignment(Box::new(UpdateAssignment {
                    location,
                    operator_type,
                    lhs,
                    rhs,
                }))
            }
            _ => unreachable!(),
        };

        Ok(assignment_expression)
    }
}
