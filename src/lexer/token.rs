//! Structs and enums for tokens

use num::BigInt;

use crate::engine::{garbage_collection::Gc};
use crate::engine::program::{Program, ProgramLocation};

/// A type of literal value
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ValueLiteral {
    /// A string literal
    String(String),
    /// A number literal
    Number(f64),
    /// A BigInt literal
    BigInt(BigInt),
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// An enum for all arithmetic assignment operators
pub(crate) enum UpdateAssignmentOperator {
    /// `+=`
    Addition,
    /// `-=`
    Subtraction,
    /// `*=`
    Multiplication,
    /// `/=`
    Division,
    /// `%=`
    Remainder,
    /// `**=`
    Exponentiation,
    /// `<<=`
    ShiftLeft,
    /// `>>=`
    ShiftRight,
    /// `>>>=`
    UnsignedShiftRight,
    /// `|=`
    BitwiseOr,
    /// `&=`
    BitwiseAnd,
    /// `^=`
    BitwiseXor,
    /// `||=`
    LogicalOr,
    /// `&&=`
    LogicalAnd,
    /// `??=`
    NullishCoalescing,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// An enum for all operators which take two arguments by value
pub(crate) enum BinaryOperator {
    // Arithmetic operators
    /// '+'. This is not emitted by the tokeniser, in favour of [TokenType::OperatorAddition], but it will be used by the parser
    Addition,
    /// '-'. This is not emitted by the tokeniser, in favour of [TokenType::OperatorSubtraction], but it will be used by the parser
    Subtraction,

    /// `*`
    Multiplication,
    /// `/`
    Division,
    /// `%`
    Remainder,
    /// `**`
    Exponentiation,

    // Comparison operators
    /// `==`
    Equality,
    /// `===`
    StrictEquality,
    /// `!=`
    Inequality,
    /// `!==`
    StrictInequality,
    /// `>`
    GreaterThan,
    /// `<`
    LessThan,
    /// `>=`
    GreaterThanOrEqual,
    /// `<=`
    LessThanOrEqual,

    // Bitwise operators
    /// `|`
    BitwiseOr,
    /// `&`
    BitwiseAnd,
    /// `^`
    BitwiseXor,
    /// `<<`
    ShiftLeft,
    /// `>>`
    ShiftRight,
    /// `>>>`
    UnsignedShiftRight,

    // Boolean operators
    /// `||`
    LogicalOr,
    /// `&&`
    LogicalAnd,

    /// '??'
    NullishCoalescing,
    /// ','
    Comma,

    // Keyword operators
    /// 'in'
    In,
    /// 'instanceof'
    InstanceOf,
}

// TODO: maybe box identifier + valueliteral variants to reduce size of enum
/// A type of token that is parsed by the lexer
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
    //Special tokens
    /// Any variable or property name
    Identifier(String),
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `.`
    OperatorDot,
    /// `?`
    OperatorQuestionMark,
    /// '?.'
    OperatorOptionalChaining,
    /// `:`
    OperatorColon,
    /// '...'
    OperatorSpread,
    /// '=>'
    OperatorFatArrow,

    // Brackets and braces
    /// `(`. usize is matching [CloseParen][TokenType::CloseParen]
    OpenParen(usize),
    /// `)`. usize is matching [OpenParen][TokenType::OpenParen]
    CloseParen(usize),
    /// `{`. usize is matching [CloseBrace][TokenType::CloseBrace]
    OpenBrace(usize),
    /// `}`. usize is matching [OpenBrace][TokenType::OpenBrace]
    CloseBrace(usize),
    /// `[`. usize is matching [CloseSquareBracket][TokenType::CloseSquareBracket]
    OpenSquareBracket(usize),
    /// `]`. usize is matching [OpenSquareBracket][TokenType::OpenSquareBracket]
    CloseSquareBracket(usize),

    // This is not in BinaryOperator as let, var, and const declarations treat it differently to other assignment operators
    /// `=`
    OperatorAssignment,

    // Arithmetic operators
    // '+' and '-' are not in BinaryOperator as they can be used as unary operators
    // e.g. '-10' is lexed as [OperatorSubtraction, NumberLiteral(10)]
    /// `+`
    OperatorAddition,
    /// `-`
    OperatorSubtraction,
    /// `++`
    OperatorIncrement,
    /// `--`
    OperatorDecrement,

    /// `!`
    OperatorLogicalNot,
    /// `~`
    OperatorBitwiseNot,

    /// An update assignment operator
    UpdateAssignment(UpdateAssignmentOperator),

    /// A binary operator
    BinaryOperator(BinaryOperator),

    // Value literals
    /// A value literal
    ValueLiteral(ValueLiteral),
}

impl TokenType {
    /// Converts the token type to a `&'static str`
    pub(crate) const fn to_str(&self) -> &'static str {
        match self {
            Self::Identifier(_) => "identifier",
            Self::ValueLiteral(_) => "value literal",

            Self::Semicolon => ";",
            Self::Comma | Self::BinaryOperator(BinaryOperator::Comma) => ",",
            Self::OperatorDot => ".",
            Self::OperatorQuestionMark => "?",
            Self::OperatorOptionalChaining => "?.",
            Self::OperatorColon => ":",
            Self::OperatorSpread => "...",
            Self::OperatorFatArrow => "=>",

            Self::OpenParen(_) => "(",
            Self::CloseParen(_) => ")",
            Self::OpenBrace(_) => "{",
            Self::CloseBrace(_) => "}",
            Self::OpenSquareBracket(_) => "[",
            Self::CloseSquareBracket(_) => "]",

            Self::OperatorAddition | Self::BinaryOperator(BinaryOperator::Addition) => "+",
            Self::OperatorSubtraction | Self::BinaryOperator(BinaryOperator::Subtraction) => "-",
            Self::BinaryOperator(BinaryOperator::Multiplication) => "*",
            Self::BinaryOperator(BinaryOperator::Division) => "/",
            Self::BinaryOperator(BinaryOperator::Remainder) => "%",
            Self::BinaryOperator(BinaryOperator::Exponentiation) => "**",
            Self::OperatorIncrement => "++",
            Self::OperatorDecrement => "--",

            Self::BinaryOperator(BinaryOperator::BitwiseOr) => "|",
            Self::BinaryOperator(BinaryOperator::BitwiseAnd) => "&",
            Self::BinaryOperator(BinaryOperator::BitwiseXor) => "^",
            Self::OperatorBitwiseNot => "~",
            Self::BinaryOperator(BinaryOperator::ShiftLeft) => "<<",
            Self::BinaryOperator(BinaryOperator::ShiftRight) => ">>",
            Self::BinaryOperator(BinaryOperator::UnsignedShiftRight) => "<<<",

            Self::BinaryOperator(BinaryOperator::LogicalOr) => "||",
            Self::BinaryOperator(BinaryOperator::LogicalAnd) => "&&",
            Self::OperatorLogicalNot => "!",

            Self::OperatorAssignment => "=",
            Self::UpdateAssignment(Addition) => "+=",
            Self::UpdateAssignment(Subtraction) => "-=",
            Self::UpdateAssignment(Multiplication) => "*=",
            Self::UpdateAssignment(Division) => "/=",
            Self::UpdateAssignment(Remainder) => "%=",
            Self::UpdateAssignment(Exponentiation) => "**=",
            Self::UpdateAssignment(ShiftLeft) => "<<=",
            Self::UpdateAssignment(ShiftRight) => ">>=",
            Self::UpdateAssignment(UnsignedShiftRight) => ">>>=",
            Self::UpdateAssignment(BitwiseOr) => "|=",
            Self::UpdateAssignment(BitwiseAnd) => "&=",
            Self::UpdateAssignment(BitwiseXor) => "^=",
            Self::UpdateAssignment(LogicalOr) => "||=",
            Self::UpdateAssignment(LogicalAnd) => "&&=",
            Self::UpdateAssignment(NullishCoalescing) => "?=",

            Self::BinaryOperator(BinaryOperator::Equality) => "==",
            Self::BinaryOperator(BinaryOperator::StrictEquality) => "===",
            Self::BinaryOperator(BinaryOperator::Inequality) => "!=",
            Self::BinaryOperator(BinaryOperator::StrictInequality) => "!==",
            Self::BinaryOperator(BinaryOperator::GreaterThan) => ">",
            Self::BinaryOperator(BinaryOperator::LessThan) => "<",
            Self::BinaryOperator(BinaryOperator::GreaterThanOrEqual) => ">=",
            Self::BinaryOperator(BinaryOperator::LessThanOrEqual) => "<=",

            Self::BinaryOperator(BinaryOperator::NullishCoalescing) => "??",

            Self::BinaryOperator(BinaryOperator::In) => "in",
            Self::BinaryOperator(BinaryOperator::InstanceOf) => "instanceof",
        }
    }
}

/// A token parsed by the lexer
#[derive(Debug, Clone)]
pub(crate) struct Token {
    /// The token's location
    pub location: ProgramLocation,
    /// The type of the token
    pub token_type: TokenType,
    /// Whether there is a newline after the token.
    /// This is used to implement semicolon insertion.
    pub newline_after: bool,
}

impl Token {
    /// Create a new [`Token`] from a [`Program`], location info, and a [`TokenType`]
    pub(super) const fn new(
        program: Gc<Program>,
        line: usize,
        line_index: usize,
        token_start: usize,
        t: TokenType,
    ) -> Self {
        Self {
            location: ProgramLocation {
                program,
                line,
                column: token_start - line_index + 1,
                index: token_start,
            },
            token_type: t,
            newline_after: false,
        }
    }
}

use TokenType::*;
use UpdateAssignmentOperator::*;

/// A map of strings to operators. 
/// Because of the way [`lex_operator`][super::Lexer::lex_operator] is implemented, any operator which starts with another operator should come before the operator that it starts with. 
/// For instance, `===` should come before `==` in this array.
#[rustfmt::skip]
pub(super) const OPERATORS: [(&str, TokenType); 51] = [
    
    ("...",  OperatorSpread),

    (",",    Comma),
    ("?.",   OperatorOptionalChaining),
    (".",    OperatorDot),
    (";",    Semicolon),
    ("=>",   OperatorFatArrow),

    ("!==",  BinaryOperator(BinaryOperator::StrictInequality)),
    ("!=",   BinaryOperator(BinaryOperator::Inequality)),
    ("===",  BinaryOperator(BinaryOperator::StrictEquality)),
    ("==",   BinaryOperator(BinaryOperator::Equality)),
    
    ("=",    OperatorAssignment),
    ("-=",   UpdateAssignment(Subtraction)),
    ("*=",   UpdateAssignment(Multiplication)),
    ("**=",  UpdateAssignment(Exponentiation)),
    ("+=",   UpdateAssignment(Addition)),
    ("/=",   UpdateAssignment(Division)),
    ("%=",   UpdateAssignment(Remainder)),

    ("||=",  UpdateAssignment(LogicalOr)),
    ("&&=",  UpdateAssignment(LogicalAnd)),
    ("|=",   UpdateAssignment(BitwiseOr)),
    ("&=",   UpdateAssignment(BitwiseAnd)),
    ("^=",   UpdateAssignment(BitwiseXor)),
    ("??=",  UpdateAssignment(NullishCoalescing)),

    ("<<=",  UpdateAssignment(ShiftLeft)),
    (">>=",  UpdateAssignment(ShiftRight)),
    (">>>=", UpdateAssignment(UnsignedShiftRight)),

    ("++",   OperatorIncrement),
    ("--",   OperatorDecrement),
    ("**",   BinaryOperator(BinaryOperator::Exponentiation)),

    ("+",    OperatorAddition),
    ("-",    OperatorSubtraction),
    ("*",    BinaryOperator(BinaryOperator::Multiplication)),
    ("/",    BinaryOperator(BinaryOperator::Division)),
    ("%",    BinaryOperator(BinaryOperator::Remainder)),

    ("||",   BinaryOperator(BinaryOperator::LogicalOr)),
    ("&&",   BinaryOperator(BinaryOperator::LogicalAnd)),
    ("!",    OperatorLogicalNot),
    ("|",    BinaryOperator(BinaryOperator::BitwiseOr)),
    ("&",    BinaryOperator(BinaryOperator::BitwiseAnd)),
    ("^",    BinaryOperator(BinaryOperator::BitwiseXor)),
    ("~",    OperatorBitwiseNot),
    ("??",   BinaryOperator(BinaryOperator::NullishCoalescing)),

    ("<<",   BinaryOperator(BinaryOperator::ShiftLeft)),
    (">>>",  BinaryOperator(BinaryOperator::UnsignedShiftRight)),
    (">>",   BinaryOperator(BinaryOperator::ShiftRight)),

    ("<=",   BinaryOperator(BinaryOperator::LessThanOrEqual)),
    (">=",   BinaryOperator(BinaryOperator::GreaterThanOrEqual)),
    ("<",    BinaryOperator(BinaryOperator::LessThan)),
    (">",    BinaryOperator(BinaryOperator::GreaterThan)),

    ("?",    OperatorQuestionMark),
    (":",    OperatorColon),
];

#[test]
/// Tests that no item in [`OPERATORS`] starts with an item before it in the array
fn test_operator_ordering() {
    for (i, first) in OPERATORS.iter().enumerate() {
        for (j, second) in OPERATORS[i + 1..].iter().enumerate() {
            assert!(
                !second.0.starts_with(first.0),
                "Item '{}' at index {j} starts with item '{}' at index {i}",
                second.0,
                first.0
            );
        }
    }
}
