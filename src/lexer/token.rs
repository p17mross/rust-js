use num::BigInt;

use crate::engine::{program::ProgramLocation, Gc, Program};

#[derive(Debug, Clone, PartialEq)]
/// An enum for all arithmetic assignment operators
pub(crate) enum ArithmeticAssignmentOperator {
    /// `+=`
    OperatorAdditionAssignment,
    /// `-=`
    OperatorSubtractionAssignment,
    /// `*=`
    OperatorMultiplicationAssignment,
    /// `/=`
    OperatorDivisionAssignment,
    /// `%=`
    OperatorRemainderAssignment,
    /// `**=`
    OperatorExponentiationAssignment,
    /// `<<=`
    OperatorShiftLeftAssignment,
    /// `>>=`
    OperatorShiftRightAssignment,
    /// `>>>=`
    OperatorUnsignedShiftRightAssignment,
    /// `|=`
    OperatorBitwiseOrAssignment,
    /// `&=`
    OperatorBitwiseAndAssignment,
    /// `^=`
    OperatorBitwiseXorAssignment,
    /// `||=`
    OperatorLogicalOrAssignment,
    /// `&&=`
    OperatorLogicalAndAssignment,
    /// `??=`
    OperatorNullishCoalescingAssignment,
}

#[derive(Debug, Clone, PartialEq)]
/// An enum for all comparison operators
pub(crate) enum ComparisonOperator {
    /// `==`
    OperatorEquality,
    /// `===`
    OperatorStrictEquality,
    /// `!=`
    OperatorInequality,
    /// `!==`
    OperatorStrictInequality,
    /// `>`
    OperatorGreaterThan,
    /// `<`
    OperatorLessThan,
    /// `>=`
    OperatorGreaterThanOrEqual,
    /// `<=`
    OperatorLessThanOrEqual,
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
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorAdditionAssignment) => "+=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorSubtractionAssignment) => "-=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorMultiplicationAssignment) => "*=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorDivisionAssignment) => "/=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorRemainderAssignment) => "%=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorExponentiationAssignment) => "**=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorShiftLeftAssignment) => "<<=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorShiftRightAssignment) => ">>=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorUnsignedShiftRightAssignment) => ">>>=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorBitwiseOrAssignment) => "|=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorBitwiseAndAssignment) => "&=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorBitwiseXorAssignment) => "^=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorLogicalOrAssignment) => "||=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorLogicalAndAssignment) => "&&=",
            Self::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorNullishCoalescingAssignment) => "?=",

            Self::Comparison(ComparisonOperator::OperatorEquality) => "==",
            Self::Comparison(ComparisonOperator::OperatorStrictEquality) => "===",
            Self::Comparison(ComparisonOperator::OperatorInequality) => "!=",
            Self::Comparison(ComparisonOperator::OperatorStrictInequality) => "!==",
            Self::Comparison(ComparisonOperator::OperatorGreaterThan) => ">",
            Self::Comparison(ComparisonOperator::OperatorLessThan) => "<",
            Self::Comparison(ComparisonOperator::OperatorGreaterThanOrEqual) => ">=",
            Self::Comparison(ComparisonOperator::OperatorLessThanOrEqual) => "<=",
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
pub(crate) const OPERATORS: [(&'static str, TokenType); 55] = [
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

    ("!==",  TokenType::Comparison(ComparisonOperator::OperatorStrictInequality)),
    ("!=",   TokenType::Comparison(ComparisonOperator::OperatorInequality)),
    ("===",  TokenType::Comparison(ComparisonOperator::OperatorStrictEquality)),
    ("==",   TokenType::Comparison(ComparisonOperator::OperatorEquality)),
    
    ("=",    TokenType::OperatorAssignment),
    ("-=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorSubtractionAssignment)),
    ("*=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorMultiplicationAssignment)),
    ("**=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorExponentiationAssignment)),
    ("+=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorAdditionAssignment)),
    ("/=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorDivisionAssignment)),
    ("%=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorRemainderAssignment)),

    ("||=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorLogicalOrAssignment)),
    ("&&=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorLogicalAndAssignment)),
    ("|=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorBitwiseOrAssignment)),
    ("&=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorBitwiseAndAssignment)),
    ("^=",   TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorBitwiseXorAssignment)),
    ("??=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorNullishCoalescingAssignment)),

    ("<<=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorShiftLeftAssignment)),
    (">>=",  TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorShiftRightAssignment)),
    (">>>=", TokenType::ArithmeticAssignment(ArithmeticAssignmentOperator::OperatorUnsignedShiftRightAssignment)),

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

    ("<=",   TokenType::Comparison(ComparisonOperator::OperatorLessThanOrEqual)),
    (">=",   TokenType::Comparison(ComparisonOperator::OperatorGreaterThanOrEqual)),
    ("<",    TokenType::Comparison(ComparisonOperator::OperatorLessThan)),
    (">",    TokenType::Comparison(ComparisonOperator::OperatorGreaterThan)),

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