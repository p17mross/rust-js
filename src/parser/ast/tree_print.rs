use super::*;

trait StringExtTreeIndent {
    fn indent_tree(&self) -> Self;
}

impl StringExtTreeIndent for String {
    fn indent_tree(&self) -> Self {
        self.replace('\n', "\n| ")
    }
}

impl ASTNodeStatement {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Block(n) => n.borrow().to_tree(),
            Self::LetExpression(n) => n.borrow().to_tree(),
            Self::Expression(e) => e.borrow().to_tree(),
        }
    }
}

impl ASTNodeExpression {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Variable(v) => v.borrow().to_tree(),
            Self::ObjectLiteral(o) => o.borrow().to_tree(),
            Self::ArrayLiteral(a) => a.borrow().to_tree(),
            Self::StringLiteral(s) => s.borrow().to_tree(),
            Self::NumberLiteral(n) => n.borrow().to_tree(),
            Self::BigIntLiteral(n) => n.borrow().to_tree(),
        }
    }
}

impl ASTNodeProgram {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Program from {}\n", self.program.borrow().source);
        s += &self.block.borrow().to_tree();
        s
    }
}

impl ASTNodeBlock {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Block at {}:{}", self.location.line, self.location.column);
        for statement in self.statements.iter() {
            s += "\n|-";
            s += &statement.to_tree().indent_tree();
        }
        s
    }
}

impl ASTNodePattern {
    pub fn to_tree(&self) -> String {
        match &self.target {
            ASTNodePatternType::Variable(identifier) => format!("\"{identifier}\""),
            ASTNodePatternType::ArrayDestructure{items, spread} => {
                let mut s = format!("Array destructure at {}:{}\n|-Items: ", self.location.line, self.location.column);
                for (i, item) in items.iter().enumerate() {
                    s += &format!("|-{i}: {}", item.to_tree().indent_tree());
                }
                if let Some(spread) = spread {
                    s += &format!("|-Spread: {}", spread.borrow().to_tree());
                }
                s
            },
            ASTNodePatternType::ObjectDestructure(items) => {
                let mut s = format!("Object destructure at {}:{}\n", self.location.line, self.location.column);
                for key in items.keys() {
                    s += &format!("|-\"{key}\": {}", items[key].to_tree());
                }
                s
            }
        }
    }
}

impl ASTNodeLetExpression {
    pub fn to_tree(&self) -> String {
        let mut s = format!("Let expression at {}:{}\n", self.location.line, self.location.column);
        s += &format!("|-lhs: {}\n", self.lhs.borrow().to_tree().indent_tree());
        s += &format!("|-rhs: {}", self.rhs.to_tree().indent_tree());
        s
    }
}

impl ASTNodeVariable {
    pub fn to_tree(&self) -> String {
        format!("Variable at {}:{}: \"{}\"", self.location.line, self.location.column, self.identifier)
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

impl ASTNodeArrayItem {
    pub fn to_tree(&self) -> String {
        match self {
            Self::Item(e) => e.borrow().to_tree(),
            Self::Spread(e) => format!("Spread from {}", e.borrow().to_tree()),
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

impl ASTNodeStringLiteral {
    pub fn to_tree(&self) -> String {
        format!("String Literal at {}:{}: \"{}\"", self.location.line, self.location.column, self.string)
    }
}

impl ASTNodeNumberLiteral {
    pub fn to_tree(&self) -> String {
        format!("Number Literal at {}:{}: {}", self.location.line, self.location.column, self.number)
    }
}

impl ASTNodeBigIntLiteral {
    pub fn to_tree(&self) -> String {
        format!("Number Literal at {}:{}: {}", self.location.line, self.location.column, self.bigint)
    }
}