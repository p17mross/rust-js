use crate::{lexer::token::BinaryOperator};
use BinaryOperator::*;

const BINARY_PRECEDENCES: [&[BinaryOperator]; 19] = [
    /* Precedence 0 - Just to make indexing work */ &[],
    /* Precedence 1  */ &[],
    /* Precedence 2  */ &[],
    /* Precedence 3  */ &[LogicalOr, NullishCoalescing],
    /* Precedence 4  */ &[LogicalAnd],
    /* Precedence 5  */ &[BitwiseOr],
    /* Precedence 6  */ &[BitwiseXor],
    /* Precedence 7  */ &[BitwiseAnd],
    /* Precedence 8  */ &[Equality, Inequality, StrictEquality, StrictInequality],
    /* Precedence 9  */ &[LessThan, LessThanOrEqual, GreaterThan, GreaterThanOrEqual, In, InstanceOf],
    /* Precedence 10 */ &[ShiftLeft, ShiftRight, UnsignedShiftRight],
    /* Precedence 11 */ &[Addition, Subtraction],
    /* Precedence 12 */ &[Multiplication, Division, Remainder],
    /* Precedence 13 */ &[Exponentiation],
    /* Precedence 14 */ &[],
    /* Precedence 15 */ &[],
    /* Precedence 16 */ &[],
    /* Precedence 17 */ &[],
    /* Precedence 18 */ &[],
];

pub(crate) fn get_binary_precedence(b: BinaryOperator) -> usize {
    for (i, ops_with_precedence) in BINARY_PRECEDENCES.iter().enumerate() {
        if ops_with_precedence.contains(&b) {return i}
    }
    panic!("o should have been in BINARY_PRECEDENCES");
}

enum Associativity {
    LeftToRight,
    RightToLeft,
}

use Associativity::*;

const ASSOCIATIVITIES: [Option<Associativity>; 19] = [    
    /* Precedence 0  - Just to make indexing work */ None, 
    /* Precedence 1  */ Some(LeftToRight), 
    /* Precedence 2  */ Some(RightToLeft), 
    /* Precedence 3  */ Some(LeftToRight), 
    /* Precedence 4  */ Some(LeftToRight), 
    /* Precedence 5  */ Some(LeftToRight),
    /* Precedence 6  */ Some(LeftToRight),
    /* Precedence 7  */ Some(LeftToRight),
    /* Precedence 8  */ Some(LeftToRight),
    /* Precedence 9  */ Some(LeftToRight),
    /* Precedence 10 */ Some(LeftToRight),
    /* Precedence 11 */ Some(LeftToRight),
    /* Precedence 12 */ Some(LeftToRight),
    /* Precedence 13 */ Some(RightToLeft),
    /* Precedence 14 */ None,
    /* Precedence 15 */ None,
    /* Precedence 16 */ None,
    /* Precedence 17 */ None,
    /* Precedence 18 */ None,
];