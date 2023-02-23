use num::BigInt;

use crate::{engine::{program::ProgramLocation, Gc, Program}, util::PrettyPrint};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueLiteral {
    String(String),
    Number(f64),
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
pub enum BinaryOperator {

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

    /// `(`. usize is matching [CloseParen](TokenType::CloseParen)
    OpenParen(usize),
    /// `)`. usize is matching [OpenParen](TokenType::OpenParen)
    CloseParen(usize),
    /// `{`. usize is matching [CloseBrace](TokenType::CloseBrace)
    OpenBrace(usize),
    /// `}`. usize is matching [OpenBrace](TokenType::OpenBrace)
    CloseBrace(usize),
    /// `[`. usize is matching [CloseSquareBracket](TokenType::CloseSquareBracket)
    OpenSquareBracket(usize),
    /// `]`. usize is matching [OpenSquareBracket](TokenType::OpenSquareBracket)
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
    pub const fn to_str(&self) -> &'static str {
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
            Self::UpdateAssignment(UpdateAssignmentOperator::Addition) => "+=",
            Self::UpdateAssignment(UpdateAssignmentOperator::Subtraction) => "-=",
            Self::UpdateAssignment(UpdateAssignmentOperator::Multiplication) => "*=",
            Self::UpdateAssignment(UpdateAssignmentOperator::Division) => "/=",
            Self::UpdateAssignment(UpdateAssignmentOperator::Remainder) => "%=",
            Self::UpdateAssignment(UpdateAssignmentOperator::Exponentiation) => "**=",
            Self::UpdateAssignment(UpdateAssignmentOperator::ShiftLeft) => "<<=",
            Self::UpdateAssignment(UpdateAssignmentOperator::ShiftRight) => ">>=",
            Self::UpdateAssignment(UpdateAssignmentOperator::UnsignedShiftRight) => ">>>=",
            Self::UpdateAssignment(UpdateAssignmentOperator::BitwiseOr) => "|=",
            Self::UpdateAssignment(UpdateAssignmentOperator::BitwiseAnd) => "&=",
            Self::UpdateAssignment(UpdateAssignmentOperator::BitwiseXor) => "^=",
            Self::UpdateAssignment(UpdateAssignmentOperator::LogicalOr) => "||=",
            Self::UpdateAssignment(UpdateAssignmentOperator::LogicalAnd) => "&&=",
            Self::UpdateAssignment(UpdateAssignmentOperator::NullishCoalescing) => "?=",

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

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub location: ProgramLocation,
    pub token_type: TokenType,
    pub newline_after: bool,
}

impl Token {
    #[inline]
    pub const fn new(program: Gc<Program>, line: usize, line_index: usize, token_start: usize, t: TokenType) -> Token {
        Token { location: ProgramLocation {program, line, column: token_start - line_index + 1, index: token_start}, token_type: t, newline_after: false}
    }
}

/// A map of strings to operators
pub(crate) const OPERATORS: [(&str, TokenType); 50] = [

("...",  TokenType::OperatorSpread),

    (",",    TokenType::Comma),
    ("?.",   TokenType::OperatorOptionalChaining),
    (".",    TokenType::OperatorDot),
    (";",    TokenType::Semicolon),
    ("=>",   TokenType::OperatorFatArrow),

    ("!==",  TokenType::BinaryOperator(BinaryOperator::StrictInequality)),
    ("!=",   TokenType::BinaryOperator(BinaryOperator::Inequality)),
    ("===",  TokenType::BinaryOperator(BinaryOperator::StrictEquality)),
    ("==",   TokenType::BinaryOperator(BinaryOperator::Equality)),
    
    ("=",    TokenType::OperatorAssignment),
    ("-=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::Subtraction)),
    ("*=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::Multiplication)),
    ("**=",  TokenType::UpdateAssignment(UpdateAssignmentOperator::Exponentiation)),
    ("+=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::Addition)),
    ("/=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::Division)),
    ("%=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::Remainder)),

    ("||=",  TokenType::UpdateAssignment(UpdateAssignmentOperator::LogicalOr)),
    ("&&=",  TokenType::UpdateAssignment(UpdateAssignmentOperator::LogicalAnd)),
    ("|=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::BitwiseOr)),
    ("&=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::BitwiseAnd)),
    ("^=",   TokenType::UpdateAssignment(UpdateAssignmentOperator::BitwiseXor)),
    ("??=",  TokenType::UpdateAssignment(UpdateAssignmentOperator::NullishCoalescing)),

    ("<<=",  TokenType::UpdateAssignment(UpdateAssignmentOperator::ShiftLeft)),
    (">>=",  TokenType::UpdateAssignment(UpdateAssignmentOperator::ShiftRight)),
    (">>>=", TokenType::UpdateAssignment(UpdateAssignmentOperator::UnsignedShiftRight)),

    ("++",   TokenType::OperatorIncrement),
    ("--",   TokenType::OperatorDecrement),
    ("**",   TokenType::BinaryOperator(BinaryOperator::Exponentiation)),

    ("+",    TokenType::OperatorAddition),
    ("-",    TokenType::OperatorSubtraction),
    ("*",    TokenType::BinaryOperator(BinaryOperator::Multiplication)),
    ("/",    TokenType::BinaryOperator(BinaryOperator::Division)),
    ("%",    TokenType::BinaryOperator(BinaryOperator::Remainder)),

    ("||",   TokenType::BinaryOperator(BinaryOperator::LogicalOr)),
    ("&&",   TokenType::BinaryOperator(BinaryOperator::LogicalAnd)),
    ("!",    TokenType::OperatorLogicalNot),
    ("|",    TokenType::BinaryOperator(BinaryOperator::BitwiseOr)),
    ("&",    TokenType::BinaryOperator(BinaryOperator::BitwiseAnd)),
    ("^",    TokenType::BinaryOperator(BinaryOperator::BitwiseXor)),
    ("~",    TokenType::OperatorBitwiseNot),

    ("<<",   TokenType::BinaryOperator(BinaryOperator::ShiftLeft)),
    (">>>",  TokenType::BinaryOperator(BinaryOperator::UnsignedShiftRight)),
    (">>",   TokenType::BinaryOperator(BinaryOperator::ShiftRight)),

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
    for (i, first) in OPERATORS.iter().enumerate() {
        for (j, second) in OPERATORS[i + 1..].iter().enumerate() {
            assert!(!second.0.starts_with(first.0), "Item '{}' at index {j} starts with item '{}' at index {i}", second.0, first.0);
        }
    }
}