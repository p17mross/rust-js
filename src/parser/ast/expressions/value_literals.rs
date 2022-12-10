use num::BigInt;

use crate::engine::program::ProgramLocation;

use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub enum ValueLiteral {
    String(String),
    Number(f64),
    BigInt(BigInt),
}

pub struct ASTNodeValueLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub value: ValueLiteral,
}


impl Debug for ASTNodeValueLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeValueLiteral at {}:{}: {:?}",
            self.location.line,
            self.location.column,
            self.value
        ))
    }
}

impl ASTNodeValueLiteral {
    pub fn to_tree(&self) -> String {
        format!("String Literal at {}:{}: {:?}", self.location.line, self.location.column, self.value)
    }
}


impl CheckParent for Rc<RefCell<ASTNodeValueLiteral>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: ASTNodeExpressionParent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            println!("{:?}, {:?}", s_ref.parent, p);
            panic!("Incorrect parent on value literal ({:?}) at {}:{}", s_ref.value, s_ref.location.line, s_ref.location.column);
        }
    }
}