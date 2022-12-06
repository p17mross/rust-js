use num::BigInt;

use crate::engine::{program::ProgramLocation, Gc, Program};

#[derive(Debug, Clone, PartialEq)]
/// An enum for all arithmetic assignment operators
pub(crate) enum ArithmeticAssignmentOperator {
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

#[derive(Debug, Clone, PartialEq)]
/// An enum for all comparison operators
pub(crate) enum ComparisonOperator {
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
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
    //Special tokens
    
    /// Any variable or property name
    Identifier(String),
    /// `;`
    Semicolon,
    /// A new line
    NewLine,
    /// `,`
    Comma,
    /// `.`
    OperatorDot,
    /// `?`
    OperatorQuestionMark,
    /// `:`
    OperatorColon,
    /// '...'
    OperatorSpread,
    /// '=>'
    OperatorFatArrow,

    // Brackets and braces

    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenSquareBracket,
    /// `]`
    CloseSquareBracket,

    // Arithmetic operators

    /// `+`
    OperatorAddition,
    /// `-`
    OperatorSubtraction,
    /// `*`
    OperatorMultiplication,
    /// `/`
    OperatorDivision,
    /// `%`
    OperatorRemainder,
    /// `**`
    OperatorExponentiation,
    /// `++`
    OperatorIncrement,
    /// `--`
    OperatorDecrement,
    
    // Bitwise operators

    /// `|`
    OperatorBitwiseOr,
    /// `&`
    OperatorBitwiseAnd,
    /// `^`
    OperatorBitwiseXor,
    /// `~`
    OperatorBitwiseNot,
    /// `<<`
    OperatorShiftLeft,
    /// `>>`
    OperatorShiftRight,
    /// `>>>`
    OperatorUnsignedShiftRight,

    // Boolean operators

    /// `||`
    OperatorLogicalOr,
    /// `&&`
    OperatorLogicalAnd,
    /// `!`
    OperatorLogicalNot,

    // Assignment operators

    /// `=`
    OperatorAssignment,
    
    /// An arithmetic assignment operator
    ArithmeticAssignment(ArithmeticAssignmentOperator),

    /// A comparison operator
    Comparison(ComparisonOperator),

    // Value literals

    /// A string literal, enclosed in double or single quotes
    StringLiteral(String),
    /// A numeric literal with a decimal
    NumberLiteral(f64),
    /// A numeric literal with no decimal
    BigIntLiteral(BigInt),
}

impl TokenType {
    /// Converts the token type to a `&'static str`
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::Identifier(_) => "identifier",
            Self::StringLiteral(_) => "string literal",
            Self::NumberLiteral(_) => "numeric literal",
            Self::BigIntLiteral(_) => "bigint literal",

            Self::Semicolon => ";",
            Self::NewLine => "newline",
            Self::Comma => ",",
            Self::OperatorDot => ".",
            Self::OperatorQuestionMark => "?",
            Self::OperatorColon => ":",
            Self::OperatorSpread => "...",
            Self::OperatorFatArrow => "=>",

            Self::OpenParen => "(",
            Self::CloseParen => ")",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::OpenSquareBracket => "[",
            Self::CloseSquareBracket => "]",

            Self::OperatorAddition => "+",
            Self::OperatorSubtraction => "-",
            Self::OperatorMultiplication => "*",
            Self::OperatorDivision => "/",
            Self::OperatorRemainder => "%",
            Self::OperatorExponentiation => "**",
            Self::OperatorIncrement => "++",
            Self::OperatorDecrement => "--",

            Self::OperatorBitwiseOr => "|",
            Self::OperatorBitwiseAnd => "&",
            Self::OperatorBitwiseXor => "^",
            Self::OperatorBitwiseNot => "~",
            Self::OperatorShiftLeft => "<<",
            Self::OperatorShiftRight => ">>",
            Self::OperatorUnsignedShiftRight => "<<<",

            Self::OperatorLogicalOr => "||",
            Self::OperatorLogicalAnd => "&&",
            Self::OperatorLogicalNot => "!",

            Self::OperatorAssignment => "=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::Addition) => "+=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::Subtraction) => "-=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::Multiplication) => "*=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::Division) => "/=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::Remainder) => "%=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::Exponentiation) => "**=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::ShiftLeft) => "<<=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::ShiftRight) => ">>=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::UnsignedShiftRight) => ">>>=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::BitwiseOr) => "|=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::BitwiseAnd) => "&=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::BitwiseXor) => "^=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::LogicalOr) => "||=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::LogicalAnd) => "&&=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::NullishCoalescing) => "?=",

            Self::Comparison(ComparisonOperator::Equality) => "==",
            Self::Comparison(ComparisonOperator::StrictEquality) => "===",
            Self::Comparison(ComparisonOperator::Inequality) => "!=",
            Self::Comparison(ComparisonOperator::StrictInequality) => "!==",
            Self::Comparison(ComparisonOperator::GreaterThan) => ">",
            Self::Comparison(ComparisonOperator::LessThan) => "<",
            Self::Comparison(ComparisonOperator::GreaterThanOrEqual) => ">=",
            Self::Comparison(ComparisonOperator::LessThanOrEqual) => "<=",
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct Token {
    pub location: ProgramLocation,
    pub token_type: TokenType
}

impl Token {
    #[inline]
    pub const fn new(program: Gc<Program>, line: usize, line_index: usize, token_start: usize, t: TokenType) -> Token {
        Token { location: ProgramLocation {program, line, column: token_start - line_index + 1, index: token_start}, token_type: t }
    }
}

/// A map of strings to operators
pub(crate) const OPERATORS: [(&str, TokenType); 55] = [
    ("(",    TokenType::OpenParen),
    (")",    TokenType::CloseParen),
    ("[",    TokenType::OpenSquareBracket),
    ("]",    TokenType::CloseSquareBracket),
    ("{",    TokenType::OpenBrace),
    ("}",    TokenType::CloseBrace),

    ("...",  TokenType::OperatorSpread),

    (",",    TokenType::Comma),
    (".",    TokenType::OperatorDot),
    (";",    TokenType::Semicolon),
    ("=>",   TokenType::OperatorFatArrow),

    ("!==",  TokenType::Comparison(ComparisonOperator::StrictInequality)),
    ("!=",   TokenType::Comparison(ComparisonOperator::Inequality)),
    ("===",  TokenType::Comparison(ComparisonOperator::StrictEquality)),
    ("==",   TokenType::Comparison(ComparisonOperator::Equality)),
    
    ("=",    TokenType::OperatorAssignment),
    ("-=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::Subtraction)),
    ("*=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::Multiplication)),
    ("**=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::Exponentiation)),
    ("+=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::Addition)),
    ("/=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::Division)),
    ("%=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::Remainder)),

    ("||=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::LogicalOr)),
    ("&&=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::LogicalAnd)),
    ("|=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::BitwiseOr)),
    ("&=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::BitwiseAnd)),
    ("^=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::BitwiseXor)),
    ("??=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::NullishCoalescing)),

    ("<<=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::ShiftLeft)),
    (">>=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::ShiftRight)),
    (">>>=", TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::UnsignedShiftRight)),

    ("++",   TokenType::OperatorIncrement),
    ("--",   TokenType::OperatorDecrement),
    ("**",   TokenType::OperatorExponentiation),

    ("+",    TokenType::OperatorAddition),
    ("-",    TokenType::OperatorSubtraction),
    ("*",    TokenType::OperatorMultiplication),
    ("/",    TokenType::OperatorDivision),
    ("%",    TokenType::OperatorRemainder),

    ("||",   TokenType::OperatorLogicalOr),
    ("&&",   TokenType::OperatorLogicalAnd),
    ("!",    TokenType::OperatorLogicalNot),
    ("|",    TokenType::OperatorBitwiseOr),
    ("&",    TokenType::OperatorBitwiseAnd),
    ("^",    TokenType::OperatorBitwiseXor),
    ("~",    TokenType::OperatorBitwiseNot),

    ("<<",   TokenType::OperatorShiftLeft),
    (">>>",  TokenType::OperatorUnsignedShiftRight),
    (">>",   TokenType::OperatorShiftRight),

    ("<=",   TokenType::Comparison(ComparisonOperator::LessThanOrEqual)),
    (">=",   TokenType::Comparison(ComparisonOperator::GreaterThanOrEqual)),
    ("<",    TokenType::Comparison(ComparisonOperator::LessThan)),
    (">",    TokenType::Comparison(ComparisonOperator::GreaterThan)),

    ("?",    TokenType::OperatorQuestionMark),
    (":",    TokenType::OperatorColon),
];

#[test]
/// Tests that no item in OPERATORS starts with an item before it in the array
fn test_operator_ordering() {
    for i in 0..OPERATORS.len() {
        for j in i + 1..OPERATORS.len() {
            if OPERATORS[j].0.starts_with(OPERATORS[i].0) {
                panic!("Item '{}' at index {j} starts with item '{}' at index {i}", OPERATORS[j].0, OPERATORS[i].0)
            }
        }
    }
}