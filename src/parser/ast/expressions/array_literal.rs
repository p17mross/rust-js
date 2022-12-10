use crate::engine::program::ProgramLocation;

use super::*;

#[derive(Debug)]
pub enum ASTNodeArrayItem {
    Item(ASTNodeExpression),
    Spread(ASTNodeExpression),
}

pub struct ASTNodeArrayLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub items: Vec<ASTNodeArrayItem>
}

impl Debug for ASTNodeArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeArrayLiteral at {}:{} {{items: {:?}}}",
            self.location.line,
            self.location.column,
            self.items
        ))
    }
}

impl ASTNodeArrayItem {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Item(e) => e.to_tree(),
            Self::Spread(e) => format!("Spread from {}", e.to_tree()),
        }
    }
}

impl ASTNodeArrayLiteral {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Array Literal at {}:{}", self.location.line, self.location.column);

        for (i, expression) in self.items.iter().enumerate() {
            s += &format!("|-{i}: {}", expression.to_tree().indent_tree())
        }

        s
    }
}

impl CheckParent for Rc<RefCell<ASTNodeArrayLiteral>> {
    type Parent = ASTNodeExpressionParent;
    fn check_parent(&self, p: Self::Parent) {
        let s_ref = self.borrow();
        if s_ref.parent != p {
            panic!("Incorrect parent on array literal at {}:{}", s_ref.location.line, s_ref.location.column);
        }

        for item in &s_ref.items {
            match item {
                ASTNodeArrayItem::Item(i) => i.check_parent(ASTNodeExpressionParent::ArrayLiteral(Rc::downgrade(&self))),
                ASTNodeArrayItem::Spread(s) => s.check_parent(ASTNodeExpressionParent::ArrayLiteral(Rc::downgrade(&self))),
            }
        }
    }
}