pub(crate) mod token;

use std::fmt::Display;

pub(crate) use token::{Token, TokenType};

use num::{BigInt, Num, ToPrimitive};

use crate::engine::{Program, Gc};
use crate::engine::program::ProgramLocation;
use crate::util::is_identifier_continue;
use crate::util::{is_identifier_start, NumberLiteralBase};

use self::token::OPERATORS;

#[derive(Debug, Clone, Copy)]
/// All the types of errors that can occur during lexing
pub enum LexErrorType {
    /// When an EOF occurs during a string literal.
    /// char is the type of quote used in the string
    UnclosedString(char),
    /// When a newline occurs during a string literal.
    /// char is the type of quote used in the string
    NewlineInString(char),
    /// When an identifier starts immediately after a numeric literal
    IdentifierAfterNumber,
    /// When the start of a numberic literal occurs with no digits following
    MissingDigits(NumberLiteralBase),
    /// When an invalid unicode occurs outside of a string
    InvalidChar(char),
}


impl Display for LexErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnclosedString(c) => f.write_fmt(format_args!("{c}{c} literal not terminated before end of script")),
            Self::NewlineInString(c) => f.write_fmt(format_args!("{c}{c} literal contains an unescaped line break")),
            Self::IdentifierAfterNumber => f.write_str("identifier starts immediately after numeric literal"),
            Self::MissingDigits(n) => f.write_fmt(format_args!("missing {} digits after '{}'", n.get_name(), n.get_start())),
            Self::InvalidChar(c) => f.write_fmt(format_args!("illegal character U+{:x}", *c as u32))
        }
    }
}

#[derive(Debug, Clone)]
/// An error that occurs during lexing.
pub struct LexError {
    pub location: ProgramLocation,
    pub error_type: LexErrorType,
}

impl LexError {
    #[inline]
    const fn new(program: Gc<Program>, line: usize, line_index: usize, token_start: usize, e: LexErrorType) -> LexError {
        LexError { location: ProgramLocation {program, line, column: token_start - line_index + 1, index: token_start}, error_type: e }
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}:{}\nSyntax Error: {}", self.location.program.borrow().source, self.location.line, self.location.column, self.error_type))
    }
}

#[derive(Debug, Default)]
/// Struct responsible for lexical analysis.
pub struct Lexer {}

impl Lexer {
    /// Constructs a list of tokens from a string.
    pub(crate) fn lex(p: Gc<Program>) -> Result<Vec<Token>, LexError> {
        // Stores the tokens
        let mut tokens: Vec<Token> = vec![];

        let p_ref = p.borrow();

        // The current index into `program`
        let mut i = 0;
        // The current line
        let mut line = 1;
        // The index into `program` of the start of the current line
        let mut line_index = 0;

        // The value of `i` from the start of the previous loop
        // Used to detect if `i` has not changed since the last loop, to detect infinite loops
        let mut prev_i = 0;

        'tokens: loop {
            // Panic if an infinite loop is detected
            if i == prev_i && i != 0 {
                panic!("Loop detected: line {line}:{}\ntokens so far:{tokens:?}", i-line_index);
            }
            prev_i = i;

            // Store the current `i` to calculate the token's line and column
            let token_start = i;

            // Get char or break on EOF
            let Some(&c) = p_ref.program.get(i) else {
                break 'tokens;
            };

            match c {
                // String literal
                quote if quote == '"' || quote == '\'' || quote == '`' => {
                    let mut s = String::new();
                    'string: loop {
                        i += 1;
                        match p_ref.program.get(i) {
                            // Error on EOF
                            None => return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::UnclosedString(quote))),
                            // Error on newlines in the string
                            // Does not error for backtick enclosed strings
                            Some('\n') if quote != '`' => return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::NewlineInString(quote))),
                            // If in a backtick string, update line on newline
                            Some('\n') => {
                                line += 1;
                                line_index = i;
                            }
                            // Detect the end of the string
                            Some(&c) if c == quote => break 'string,
                            // Parse escape sequences
                            Some('\\') => {
                                i += 1;
                                // Add character to string
                                s += &match p_ref.program.get(i) {
                                    None => return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::UnclosedString(quote))),
                                    // Line continuation
                                    Some('\n') => "".to_string(),
                                    // Newline
                                    Some('n') => "\n".to_string(),
                                    // Carriage return
                                    Some('r') => "\r".to_string(),
                                    // Tab
                                    Some('t') => "\t".to_string(),
                                    // Backspace
                                    Some('b') => "\u{0008}".to_string(),
                                    // Form feed
                                    Some('f') => "\u{000C}".to_string(),
                                    // Vertical tab
                                    Some('v') => "\u{000C}".to_string(),
                                    // TODO: unicode strings
                                    Some('u') | Some('x') => todo!(),
                                    // Any other character
                                    Some(c) => c.to_string()
                                };
                            }
                            // If any other char, add it to the string
                            Some(c) => {
                                s += &c.to_string();
                            }
                        }
                    }
                    tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::StringLiteral(s)));
                    i += 1;
                }

                // Number or BigInt literal
                digit if ('0'..='9').contains(&digit) => {
                    // Init base to decimal
                    let mut base = NumberLiteralBase::Decimal;
                    // Could be '0' literal, octal string e.g. '012' meaning 10, or start of '0x', '0b', etc.
                    if digit == '0' {
                        i += 1;
                        match p_ref.program.get(i) {
                            // If EOF here, generate `NumberLiteral(0)`
                            None => {
                                tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::NumberLiteral(0.0)));
                                break 'tokens;
                            },
                            Some(&c) => base = match c {
                                // Hex literal
                                'x' | 'X' => {i += 1; NumberLiteralBase::Hex},
                                // Octal literal
                                'o' | 'O' | '0' => {i += 1; NumberLiteralBase::Octal},
                                // Binary literal
                                'b' | 'B' => {i += 1; NumberLiteralBase::Binary},
                                // Bigint '0n' literal
                                'n' => {
                                    // Error if the next char is an identifier
                                    if let Some(&c) = p_ref.program.get(i + 1) {
                                        if is_identifier_start(c) {
                                            return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::IdentifierAfterNumber))
                                        }
                                    }
                                    // Generate `BigIntLiteral(0)` token
                                    tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::BigIntLiteral(BigInt::from(0))));
                                    i += 2;
                                    continue 'tokens;
                                },
                                // TODO: error here in strict mode
                                // Octal literal with no '0o' or '0O'
                                c if ('1'..='9').contains(&c) => NumberLiteralBase::Octal,
                                // Error if identifier encountered
                                c if is_identifier_start(c) => {
                                    return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::IdentifierAfterNumber))
                                }
                                _ => {
                                    tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::NumberLiteral(0.0)));
                                    continue 'tokens;
                                }
                            }
                        };
                    }
                    
                    // Start of the digits
                    let digits_start = i;
                    // Whether there has been a decimal point yet
                    let mut had_decimal = false;
                    // The number for the string
                    let mut number = String::new();

                    'digits: loop {
                        match p_ref.program.get(i) {
                            // Error on EOF
                            None => {
                                if digits_start == i {
                                    return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::MissingDigits(base)))
                                } 
                                break 'digits;
                            },
                            // Indicates a BigInt literal instead of a number
                            Some('n') => {
                                if had_decimal {return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::IdentifierAfterNumber))}
                                tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::BigIntLiteral(BigInt::from_str_radix(&number, base.get_radix()).expect("Should have been a valid bigint"))));
                                i += 1;
                                continue 'tokens;
                            },
                            // A digit
                            Some(digit) if base.get_chars().contains(&digit.to_string()) => {number += &digit.to_string()},
                            // Underscores are ignored in numeric literals
                            Some('_') => (),
                            // A newline
                            Some('\n') => {
                                // Decrement i so that the next loop iteration finds the newline
                                i -= 1;
                                break 'digits;
                            },
                            Some('.') if base == NumberLiteralBase::Decimal => {
                                had_decimal = true;
                            }
                            // Error if an identifier is found
                            Some(&id) if is_identifier_start(id) => {return Err(LexError::new(p.clone(), line, line_index, i, LexErrorType::IdentifierAfterNumber))},
                            // Any other character means the end of the number
                            _ => break 'digits,
                        }
                        i += 1;
                    }
                    i += 1;
                    if base == NumberLiteralBase::Decimal {
                        let n = number.parse::<f64>().expect("Should have been a valid float");
                        tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::NumberLiteral(n)))
                    }
                    else {
                        // Parse string to number
                        let n = num::BigInt::from_str_radix(&number, base.get_radix()).expect("Should have been a valid bigint");
                        let n = n.to_f64().unwrap_or(f64::INFINITY);

                        tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::NumberLiteral(n)))
                    }
                }

                // Newline
                '\n' => {
                    tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::NewLine));
                    i += 1;
                    line += 1;
                    line_index = i;
                }       
                
                // Ignore whitespace
                w if w.is_whitespace() => {i += 1}

                // Single line comments
                '/' if p_ref.program.get(i + 1) == Some(&'/') => {
                    i += 2;
                    // Find newline to end comment
                    'comment: loop {
                        match p_ref.program.get(i) {
                            None => break 'tokens,
                            Some('\n') => {
                                break 'comment
                            },
                            _ => i += 1,
                        }
                    }
                }

                // Multi-line comments
                '/' if p_ref.program.get(i + 1) == Some(&'*') => {
                    i += 2;
                    // Find '*/' to end comment
                    'comment: loop {
                        match p_ref.program.get(i) {
                            None => break 'tokens,
                            // Still track line / columns in a comment
                            Some('\n') => {
                                i += 1;
                                line += 1;
                                line_index = i;
                            }
                            Some('*') if p_ref.program.get(i + 1) == Some(&'/') => {
                                i += 2;
                                break 'comment
                            },
                            _ => i += 1,
                        }
                    }
                }

                // An identifier
                c if is_identifier_start(c) => {
                    'chars_in_identifer: loop {
                        match p_ref.program.get(i) {
                            None => {break 'chars_in_identifer},
                            Some(&c) if is_identifier_continue(c) => i += 1,
                            _ => break 'chars_in_identifer,
                        }
                    }

                    let ident: String = p_ref.program[token_start..i].iter().collect();

                    tokens.push(Token::new(p.clone(), line, line_index, token_start, TokenType::Identifier(ident)));
                }
            
                // Any other character: should be an operator
                c => {
                    for (operator, operator_token) in OPERATORS {
                        // Get operators
                        if p_ref.program[i..i+operator.len()].iter().map(char::to_owned).eq(operator.chars()) {
                            i += operator.len();
                            tokens.push(Token::new(p.clone(), line, line_index, token_start, operator_token));
                            continue 'tokens;
                        }
                    }
                    return Err(LexError::new(p.clone(), line, line_index, token_start, LexErrorType::InvalidChar(c)));
                }
            }
        }
        Ok(tokens)
    }
}