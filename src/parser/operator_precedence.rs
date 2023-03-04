//! Constants and structs for representing the precedence of different operators

use crate::lexer::token::BinaryOperator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A binary operator's associativity.\
pub(super) enum Associativity {
    /// Operators are grouped left to right, e.g. `a + b + c` is equivalent to `(a + b) + c`
    LeftToRight,
    /// Operators are grouped right to left, e.g. `a ** b ** c` is equivalent to `a ** (b ** c)`
    RightToLeft,
}

#[derive(Debug, Clone, Copy)]
/// A certain precedence of binary operator.\
/// Stores which operators are in this precedence level and which associativity they are
pub(super) struct BinaryPrecedence {
    /// Whether the operators in this precedence are left or right associative
    pub associativity: Associativity,
    /// Which operators are included in the precedence
    pub operators: &'static [BinaryOperator],
}

use Associativity::*;
use BinaryOperator::*;

/// Which operators belong to a given precedence.
/// This is used by [`parse_binary_operator`][Parser::parse_binary_operator] to implement precedence
#[rustfmt::skip]
pub(super) const BINARY_PRECEDENCES: &[Option<BinaryPrecedence>] = &[
    /* Precedence 0 - Not a real precedence, but having it here makes offsets nicer */ None,
    /* Precedence 1  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[Comma] }),
    /* Precedence 2  */ None,
    /* Precedence 3  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[LogicalOr, NullishCoalescing] }),
    /* Precedence 4  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[LogicalAnd] }),
    /* Precedence 5  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[BitwiseOr] }),
    /* Precedence 6  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[BitwiseXor] }),
    /* Precedence 7  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[BitwiseAnd] }),
    /* Precedence 8  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[Equality, Inequality, StrictEquality, StrictInequality] }),
    /* Precedence 9  */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[LessThan, LessThanOrEqual, GreaterThan, GreaterThanOrEqual, In, InstanceOf] }),
    /* Precedence 10 */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[ShiftLeft, ShiftRight, UnsignedShiftRight] }),
    /* Precedence 11 */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[Addition, Subtraction] }),
    /* Precedence 12 */ Some(BinaryPrecedence{ associativity: LeftToRight, operators: &[Multiplication, Division, Remainder] }),
    /* Precedence 13 */ Some(BinaryPrecedence{ associativity: RightToLeft, operators: &[Exponentiation] }),
    /* Precedences 14 - 18 have no binary operators */
];

/// Constants for which operators have what precedence.
/// Not all operators are listed here, only ones where the precedence number is needed somewhere in the parser.
pub(crate) mod precedences {
    /// The highest precedence - the precedence of value, array and object literals, and parentheses.
    pub const GROUPING: usize = 18;

    /// The precedence of operators which make up assignment targets - `.`, `?.`, `[...]`, new with argument list, function call
    pub const ASSIGNMENT_TARGET: usize = 17;

    /// The precedence of the unary operators `!`, `~`, `+`, `-`, prefix increment and decrement, `typeof`, `void`, `delete`, and `await`
    pub const UNARY_OPERATOR: usize = 14;

    /// The precedence of the binary addition operator
    pub const ADDITION: usize = 11;
    /// The precedence of the binary subtraction operator
    pub const SUBTRACTION: usize = 11;

    /// The precedence of assignment operators
    pub const ASSIGNMENT: usize = 2;

    /// The precedence of the comma operator
    pub const COMMA: usize = 1;

    /// The lowest precedence - will parse any expression
    pub const ANY_EXPRESSION: usize = 0;
}
