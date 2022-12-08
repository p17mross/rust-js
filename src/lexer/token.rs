use num::BigInt;

use crate::{engine::{program::ProgramLocation, Gc, Program}, util::PrettyPrint};

#[derive(Debug, Clone, PartialEq)]
/// An enum for all arithmetic assignment operators
pub(crate) enum AssignmentOperator {
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
/// An enum for all operators which take two arguments by value
pub(crate) enum BinaryOperator {

    // Arithmetic operators

    /// `*`
    OperatorMultiplication,
    /// `/`
    OperatorDivision,
    /// `%`
    OperatorRemainder,
    /// `**`
    OperatorExponentiation,

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
    OperatorBitwiseOr,
    /// `&`
    OperatorBitwiseAnd,
    /// `^`
    OperatorBitwiseXor,
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

    /// An assignment operator
    Assignment(AssignmentOperator),

    /// A binary operator
    BinaryOperator(BinaryOperator),

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
            Self::BinaryOperator(BinaryOperator::OperatorMultiplication) => "*",
            Self::BinaryOperator(BinaryOperator::OperatorDivision) => "/",
            Self::BinaryOperator(BinaryOperator::OperatorRemainder) => "%",
            Self::BinaryOperator(BinaryOperator::OperatorExponentiation) => "**",
            Self::OperatorIncrement => "++",
            Self::OperatorDecrement => "--",

            Self::BinaryOperator(BinaryOperator::OperatorBitwiseOr) => "|",
            Self::BinaryOperator(BinaryOperator::OperatorBitwiseAnd) => "&",
            Self::BinaryOperator(BinaryOperator::OperatorBitwiseXor) => "^",
            Self::OperatorBitwiseNot => "~",
            Self::BinaryOperator(BinaryOperator::OperatorShiftLeft) => "<<",
            Self::BinaryOperator(BinaryOperator::OperatorShiftRight) => ">>",
            Self::BinaryOperator(BinaryOperator::OperatorUnsignedShiftRight) => "<<<",

            Self::BinaryOperator(BinaryOperator::OperatorLogicalOr) => "||",
            Self::BinaryOperator(BinaryOperator::OperatorLogicalAnd) => "&&",
            Self::OperatorLogicalNot => "!",

            Self::OperatorAssignment => "=",
            Self::Assignment(AssignmentOperator::Addition) => "+=",
            Self::Assignment(AssignmentOperator::Subtraction) => "-=",
            Self::Assignment(AssignmentOperator::Multiplication) => "*=",
            Self::Assignment(AssignmentOperator::Division) => "/=",
            Self::Assignment(AssignmentOperator::Remainder) => "%=",
            Self::Assignment(AssignmentOperator::Exponentiation) => "**=",
            Self::Assignment(AssignmentOperator::ShiftLeft) => "<<=",
            Self::Assignment(AssignmentOperator::ShiftRight) => ">>=",
            Self::Assignment(AssignmentOperator::UnsignedShiftRight) => ">>>=",
            Self::Assignment(AssignmentOperator::BitwiseOr) => "|=",
            Self::Assignment(AssignmentOperator::BitwiseAnd) => "&=",
            Self::Assignment(AssignmentOperator::BitwiseXor) => "^=",
            Self::Assignment(AssignmentOperator::LogicalOr) => "||=",
            Self::Assignment(AssignmentOperator::LogicalAnd) => "&&=",
            Self::Assignment(AssignmentOperator::NullishCoalescing) => "?=",

            Self::BinaryOperator(BinaryOperator::Equality) => "==",
            Self::BinaryOperator(BinaryOperator::StrictEquality) => "===",
            Self::BinaryOperator(BinaryOperator::Inequality) => "!=",
            Self::BinaryOperator(BinaryOperator::StrictInequality) => "!==",
            Self::BinaryOperator(BinaryOperator::GreaterThan) => ">",
            Self::BinaryOperator(BinaryOperator::LessThan) => "<",
            Self::BinaryOperator(BinaryOperator::GreaterThanOrEqual) => ">=",
            Self::BinaryOperator(BinaryOperator::LessThanOrEqual) => "<=",
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

    ("!==",  TokenType::BinaryOperator(BinaryOperator::StrictInequality)),
    ("!=",   TokenType::BinaryOperator(BinaryOperator::Inequality)),
    ("===",  TokenType::BinaryOperator(BinaryOperator::StrictEquality)),
    ("==",   TokenType::BinaryOperator(BinaryOperator::Equality)),
    
    ("=",    TokenType::OperatorAssignment),
    ("-=",   TokenType::Assignment(AssignmentOperator::Subtraction)),
    ("*=",   TokenType::Assignment(AssignmentOperator::Multiplication)),
    ("**=",  TokenType::Assignment(AssignmentOperator::Exponentiation)),
    ("+=",   TokenType::Assignment(AssignmentOperator::Addition)),
    ("/=",   TokenType::Assignment(AssignmentOperator::Division)),
    ("%=",   TokenType::Assignment(AssignmentOperator::Remainder)),

    ("||=",  TokenType::Assignment(AssignmentOperator::LogicalOr)),
    ("&&=",  TokenType::Assignment(AssignmentOperator::LogicalAnd)),
    ("|=",   TokenType::Assignment(AssignmentOperator::BitwiseOr)),
    ("&=",   TokenType::Assignment(AssignmentOperator::BitwiseAnd)),
    ("^=",   TokenType::Assignment(AssignmentOperator::BitwiseXor)),
    ("??=",  TokenType::Assignment(AssignmentOperator::NullishCoalescing)),

    ("<<=",  TokenType::Assignment(AssignmentOperator::ShiftLeft)),
    (">>=",  TokenType::Assignment(AssignmentOperator::ShiftRight)),
    (">>>=", TokenType::Assignment(AssignmentOperator::UnsignedShiftRight)),

    ("++",   TokenType::OperatorIncrement),
    ("--",   TokenType::OperatorDecrement),
    ("**",   TokenType::BinaryOperator(BinaryOperator::OperatorExponentiation)),

    ("+",    TokenType::OperatorAddition),
    ("-",    TokenType::OperatorSubtraction),
    ("*",    TokenType::BinaryOperator(BinaryOperator::OperatorMultiplication)),
    ("/",    TokenType::BinaryOperator(BinaryOperator::OperatorDivision)),
    ("%",    TokenType::BinaryOperator(BinaryOperator::OperatorRemainder)),

    ("||",   TokenType::BinaryOperator(BinaryOperator::OperatorLogicalOr)),
    ("&&",   TokenType::BinaryOperator(BinaryOperator::OperatorLogicalAnd)),
    ("!",    TokenType::OperatorLogicalNot),
    ("|",    TokenType::BinaryOperator(BinaryOperator::OperatorBitwiseOr)),
    ("&",    TokenType::BinaryOperator(BinaryOperator::OperatorBitwiseAnd)),
    ("^",    TokenType::BinaryOperator(BinaryOperator::OperatorBitwiseXor)),
    ("~",    TokenType::OperatorBitwiseNot),

    ("<<",   TokenType::BinaryOperator(BinaryOperator::OperatorShiftLeft)),
    (">>>",  TokenType::BinaryOperator(BinaryOperator::OperatorUnsignedShiftRight)),
    (">>",   TokenType::BinaryOperator(BinaryOperator::OperatorShiftRight)),

    ("<=",   TokenType::BinaryOperator(BinaryOperator::LessThanOrEqual)),
    (">=",   TokenType::BinaryOperator(BinaryOperator::GreaterThanOrEqual)),
    ("<",    TokenType::BinaryOperator(BinaryOperator::LessThan)),
    (">",    TokenType::BinaryOperator(BinaryOperator::GreaterThan)),

    ("?",    TokenType::OperatorQuestionMark),
    (":",    TokenType::OperatorColon),
];

impl PrettyPrint for Vec<Token> {
    fn pretty_print(&self) {
        println!("Tokens parsed from {:?}", self[0].location.program.borrow().source);
        for token in self {
            println!("{:?} at {}:{}", token.token_type, token.location.line, token.location.column);
        }
    }   
}

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