use std::collections::HashMap;

use crate::engine::program::ProgramLocation;

use super::*;


pub struct ASTNodeObjectLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub properties: HashMap<String, ASTNodeExpression>
}

impl Debug for ASTNodeObjectLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeObjectLiteral at {}:{} {{properties: {:?}}}",
            self.location.line,
            self.location.column,
            self.properties
        ))
    }
}

impl ASTNodeObjectLiteral {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Object Literal at {}:{}", self.location.line, self.location.column);

        for (key, expression) in &self.properties {
            s += &format!("|-\"{key}\": {}", expression.to_tree().indent_tree());
        }

        s
    }
}

impl CheckParent for Rc<RefCell<ASTNodeObjectLiteral>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on object literal at {}:{}", s_ref.location.line, s_ref.location.column);
        }

        for (_, property) in &s_ref.properties {
            property.check_parent(ASTNodeExpressionParent::ObjectLiteral(Rc::downgrade(&self)));
        }
    }
}