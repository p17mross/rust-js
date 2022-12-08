use super::*;

use std::fmt::Debug;


impl Debug for ASTNodeProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ASTNodeProgram from {:?} {{{:?}}}", self.program.borrow().source, self.block))
    }
}

impl Debug for ASTNodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("ASTNodeProgram at {}:{} {{", self.location.line, self.location.column);
        for (i, statement) in self.statements.iter().enumerate() {
            s += &format!("{statement:?}");
            if i != self.statements.len() - 1 {
                s += ", ";
            }
        }
        s += "}";

        f.write_str(&s)    
    }
}

impl Debug for ASTNodePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeAssignmentLHS at {}:{} {{target: {:?}}}", 
            self.location.line, 
            self.location.column, 
            self.target
        ))
    }
}

impl Debug for ASTNodeLetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ASTNodeLetExpression {{lhs: {:?}, rhs: {:?}}}", self.lhs, self.rhs))
    }
}

impl Debug for ASTNodeVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeVariable at {}:{} {{identifier: {:?}}}", 
            self.location.line, 
            self.location.column, 
            self.identifier
        ))
    }
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

impl Debug for ASTNodeStringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeStringLiteral at {}:{}: \"{}\"",
            self.location.line,
            self.location.column,
            self.string
        ))
    }
}

impl Debug for ASTNodeNumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeNumberLiteral at {}:{}: {}",
            self.location.line,
            self.location.column,
            self.number
        ))
    }
}

impl Debug for ASTNodeBigIntLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeNumberLiteral at {}:{}: {}",
            self.location.line,
            self.location.column,
            self.bigint
        ))
    }
}

impl Debug for ASTNodeUnaryPlus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeNumberLiteral at {}:{}: {:?}",
            self.location.line,
            self.location.column,
            self.expression
        ))
    }
}

impl Debug for ASTNodeUnaryMinus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "ASTNodeNumberLiteral at {}:{}: {:?}",
            self.location.line,
            self.location.column,
            self.expression
        ))
    }
}