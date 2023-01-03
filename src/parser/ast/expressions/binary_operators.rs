use std::fmt::Debug;

use crate::lexer::token::BinaryOperator;

use super::*;

pub struct ASTNodeBinaryOperator {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub operator_type: BinaryOperator,
    pub lhs: ASTNodeExpression,
    pub rhs: ASTNodeExpression,
}

impl Debug for ASTNodeBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeUnaryOperator({:?}) at {}:{}: {{lhs: {:?}, rhs: {:?}}}",
            self.operator_type,
            self.location.line,
            self.location.column,
            self.lhs,
            self.rhs
        ))
    }
}

impl ASTNodeBinaryOperator {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Binary operator {:?} at {}:{}\n", self.operator_type, self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}

impl CheckParent for Rc<RefCell<ASTNodeBinaryOperator>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on binary operator at {}:{}", s_ref.location.line, s_ref.location.column)
        }

        s_ref.lhs.check_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(self)));
        s_ref.rhs.check_parent(ASTNodeExpressionParent::BinaryOperator(Rc::downgrade(self)));
    }
}