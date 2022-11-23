use num::BigInt;

#[derive(Debug, Clone)]
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

    // Equality operators

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

    // Value literals

    /// A string literal, enclosed in double or single quotes
    StringLiteral(String),
    /// A numeric literal with a decimal
    NumberLiteral(f64),
    /// A numeric literal with no decimal
    BigIntLiteral(BigInt),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct Token {
    pub line: usize,
    pub char: usize,
    pub index: usize,
    pub token: TokenType
}

impl Token {
    #[inline]
    pub const fn new(line: usize, line_index: usize, token_start: usize, t: TokenType) -> Token {
        Token { line, char: token_start - line_index + 1, index: token_start, token: t }
    }
}

/// A map of strings to operators
pub(crate) const OPERATORS: [(&'static str, TokenType); 55] = [
    ("(", TokenType::OpenParen),
    (")", TokenType::CloseParen),
    ("[", TokenType::OpenSquareBracket),
    ("]", TokenType::CloseSquareBracket),
    ("{", TokenType::OpenBrace),
    ("}", TokenType::CloseBrace),

    ("...", TokenType::OperatorSpread),

    (",", TokenType::Comma),
    (".", TokenType::OperatorDot),
    (";", TokenType::Semicolon),
    ("=>", TokenType::OperatorFatArrow),

    ("!==", TokenType::OperatorStrictInequality),
    ("!=", TokenType::OperatorInequality),
    ("===", TokenType::OperatorStrictEquality),
    ("==", TokenType::OperatorEquality),
    
    ("=", TokenType::OperatorAssignment),
    ("+=", TokenType::OperatorAdditionAssignment),
    ("-=", TokenType::OperatorSubtractionAssignment),
    ("*=", TokenType::OperatorMultiplicationAssignment),
    ("**=", TokenType::OperatorExponentiationAssignment),
    ("/=", TokenType::OperatorDivisionAssignment),
    ("%=", TokenType::OperatorRemainderAssignment),

    ("||=", TokenType::OperatorLogicalOrAssignment),
    ("&&=", TokenType::OperatorLogicalAndAssignment),
    ("|=", TokenType::OperatorBitwiseOrAssignment),
    ("&=", TokenType::OperatorBitwiseAndAssignment),
    ("^=", TokenType::OperatorBitwiseXorAssignment),
    ("??=", TokenType::OperatorNullishCoalescingAssignment),

    ("<<=", TokenType::OperatorShiftLeftAssignment),
    (">>=", TokenType::OperatorShiftRightAssignment),
    (">>>=", TokenType::OperatorUnsignedShiftRightAssignment),

    ("++", TokenType::OperatorIncrement),
    ("--", TokenType::OperatorDecrement),
    ("**", TokenType::OperatorExponentiation),

    ("+", TokenType::OperatorAddition),
    ("-", TokenType::OperatorSubtraction),
    ("*", TokenType::OperatorMultiplication),
    ("/", TokenType::OperatorDivision),
    ("%", TokenType::OperatorRemainder),

    ("||", TokenType::OperatorLogicalOr),
    ("&&", TokenType::OperatorLogicalAnd),
    ("!", TokenType::OperatorLogicalNot),
    ("|", TokenType::OperatorBitwiseOr),
    ("&", TokenType::OperatorBitwiseAnd),
    ("^", TokenType::OperatorBitwiseXor),
    ("~", TokenType::OperatorBitwiseNot),

    ("<<", TokenType::OperatorShiftLeft),
    (">>>", TokenType::OperatorUnsignedShiftRight),
    (">>", TokenType::OperatorShiftRight),

    ("<=", TokenType::OperatorLessThanOrEqual),
    (">=", TokenType::OperatorGreaterThanOrEqual),
    ("<", TokenType::OperatorLessThan),
    (">", TokenType::OperatorGreaterThan),

    ("?", TokenType::OperatorQuestionMark),
    (":", TokenType::OperatorColon),
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