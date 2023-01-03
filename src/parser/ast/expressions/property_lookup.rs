use crate::engine::program::ProgramLocation;
use super::*;


pub struct ASTNodePropertyLookup {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub lhs: ASTNodeExpression,
    pub rhs: String,
}

impl Debug for ASTNodePropertyLookup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodePropertyLookup at {}:{} {{lhs: {:?}, rhs: {:?}}}", 
            self.location.line, 
            self.location.column, 
            self.lhs,
            self.rhs
        ))
    }
}

impl ASTNodePropertyLookup {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Property lookup at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs);
        s
    }
}

impl CheckParent for Rc<RefCell<ASTNodePropertyLookup>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on property lookup at {}:{}", s_ref.location.line, s_ref.location.column);
        }
        s_ref.lhs.check_parent(ASTNodeExpressionParent::PropertyLookup(Rc::downgrade(self)));
    }
}