use crate::engine::program::ProgramLocation;

use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    Await,
    TypeOf,
    Void,
}

pub struct ASTNodeUnaryOperator {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub operator_type: UnaryOperator,
    pub expression: ASTNodeExpression,
}

impl Debug for ASTNodeUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeUnaryOperator({:?}) at {}:{}: {:?}",
            self.operator_type,
            self.location.line,
            self.location.column,
            self.expression
        ))
    }
}


impl ASTNodeUnaryOperator {
    pub fn to_tree(&self) -> String {
        format!("Unary operator ({:?}) at {}:{}: {}", self.operator_type, self.location.line, self.location.column, self.expression.to_tree())
    }
}

impl CheckParent for Rc<RefCell<ASTNodeUnaryOperator>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();

        if s_ref.parent != p {
            panic!("Incorrect parent on unary operator {:?} at {}:{}", s_ref.operator_type, s_ref.location.line, s_ref.location.column);
        }

        s_ref.expression.check_parent(ASTNodeExpressionParent::UnaryOperator(Rc::downgrade(self)))
    }
}