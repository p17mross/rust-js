use std::{rc::Rc, cell::RefCell, collections::HashMap};

use num::BigInt;

use crate::engine::{Gc, Program, garbagecollection::GarbageCollectable, program::ProgramLocation};

use super::*;



pub struct ASTNodeProgram {
    pub program: Gc<Program>,
    pub block: Rc<RefCell<ASTNodeBlock>>,
}

impl GarbageCollectable for ASTNodeProgram {
    fn get_children(&self) -> Vec<crate::engine::garbagecollection::GarbageCollectionId> {
        vec![self.program.get_id()]
    }
}

impl ASTNodeProgram {
    pub fn new(program: Gc<Program>) -> Rc<RefCell<Self>> {
        let block = Rc::new(RefCell::new(ASTNodeBlock {
            location: ProgramLocation { 
                program: program.clone(),
                line: 0, 
                column: 0, 
                index: 0 
            }, 
            statements: vec![],
            parent: ASTNodeBlockParent::Unset,
        }));

        let s = Rc::new(RefCell::new(Self {
            program,
            block: block.clone()
        }));
        
        block.borrow_mut().parent = ASTNodeBlockParent::Program(Rc::downgrade(&s));

        s
    }
}

pub struct ASTNodeBlock {
    pub location: ProgramLocation,
    pub parent: ASTNodeBlockParent,

    pub statements: Vec<ASTNodeStatement>,
}

#[derive(Debug)]
pub enum ASTNodePatternType {
    Variable(String),
    ArrayDestructure {
        items: Vec<ASTNodePattern>, 
        spread: Option<Rc<RefCell<ASTNodePattern>>>,
    },
    ObjectDestructure(HashMap<String, ASTNodePattern>)
}

pub struct ASTNodePattern {
    pub location: ProgramLocation,
    pub parent: ASTNodePatternParent,

    pub target: ASTNodePatternType,
}

pub struct ASTNodeLetExpression {
    pub location: ProgramLocation,
    pub parent: ASTNodeStatementParent,

    pub lhs: Rc<RefCell<ASTNodePattern>>,
    pub rhs: ASTNodeExpression,
}

pub struct ASTNodeVariable {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub identifier: String,
}

pub struct ASTNodeObjectLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub properties: HashMap<String, ASTNodeExpression>
}

#[derive(Debug)]
pub enum ASTNodeArrayItem {
    Item(Rc<RefCell<ASTNodeExpression>>),
    Spread(Rc<RefCell<ASTNodeExpression>>),
}

pub struct ASTNodeArrayLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub items: Vec<ASTNodeArrayItem>
}

pub struct ASTNodeStringLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub string: String,
}

pub struct ASTNodeNumberLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub number: f64,
}

pub struct ASTNodeBigIntLiteral {
    pub location: ProgramLocation,
    pub parent: ASTNodeExpressionParent,

    pub bigint: BigInt,
}