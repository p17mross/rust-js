pub(crate) mod token;

use std::fmt::Display;

pub(crate) use token::{Token, TokenType};

use num::{BigInt, Num, ToPrimitive};

use crate::engine::{Program, Gc};
use crate::engine::program::ProgramLocation;
use crate::util::is_identifier_continue;
use crate::util::{is_identifier_start, NumberLiteralBase};

use self::token::{OPERATORS, ValueLiteral};

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
    /// When the start of a numeric literal occurs with no digits following
    MissingDigits(NumberLiteralBase),
    /// When an invalid unicode occurs outside of a string
    InvalidChar(char),
    /// When brackets are incorrectly matched
    MismatchedBracket,
    /// Unclosed Bracket
    UnclosedBracket
}


impl Display for LexErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnclosedString(c) => f.write_fmt(format_args!("{c}{c} literal not terminated before end of script")),
            Self::NewlineInString(c) => f.write_fmt(format_args!("{c}{c} literal contains an unescaped line break")),
            Self::IdentifierAfterNumber => f.write_str("Identifier starts immediately after numeric literal"),
            Self::MissingDigits(n) => f.write_fmt(format_args!("Missing {} digits after '{}'", n.get_name(), n.get_start())),
            Self::InvalidChar(c) => f.write_fmt(format_args!("Illegal character U+{:x}", *c as u32)),
            Self::MismatchedBracket => f.write_str("Mismatched bracket"),
            Self::UnclosedBracket => f.write_str("Unclosed bracket")
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

#[derive(Debug)]
enum Bracket {
    Paren(usize),
    Brace(usize),
    SquareBracket(usize)
}

#[derive(Debug)]
/// Struct responsible for lexical analysis.
pub struct Lexer {
    /// The current index into `program.program`
    i: usize,
    /// The current line number
    line: usize,
    /// The index into `program` of the start of the current line
    line_index: usize,

    /// The self.tokens which are being produced
    tokens: Vec<Token>,
    /// The stack of bracket token types and locations. Used to detect bracket mismatches.
    brackets: Vec<Bracket>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            i: 0,
            line: 1,
            line_index: 0,

            tokens: Vec::new(),
            brackets: Vec::new()
        }
    }

    /// Constructs a list of self.tokens from a string.
    pub(crate) fn lex(mut self, program: &Gc<Program>) -> Result<Vec<Token>, LexError> {
        // The value of `self.i` from the start of the previous loop
        // Used to detect if `self.i` has not changed since the last loop, to detect infinite loops
        let mut prev_i = 0;
        let mut is_first_loop = true;

        let program_text = &program.borrow().program;

        'tokens: loop {
            if !is_first_loop {
                // Panic if an infinite loop is detected
                assert_ne!(prev_i, self.i, "Infinite loop detected while parsing");
                prev_i = self.i;
            }
            is_first_loop = false;

            // Store the current `self.i` to calculate the token's self.line and column
            let token_start = self.i;

            // Get char or break on EOF
            let Some(&c) = program_text.get(self.i) else {
                break 'tokens;
            };

            match c {
                // String literal
                quote if quote == '"' || quote == '\'' || quote == '`' => self.lex_string_literal(program, program_text, quote, token_start)?,

                // Number or BigInt literal
                digit if ('0'..='9').contains(&digit) => self.lex_numeric_literal(program, program_text, token_start)?,

                // Newline
                '\n' => {
                    if let Some(t) = self.tokens.last_mut() {
                        t.newline_after = true;
                    }
                    self.i += 1;
                    self.line += 1;
                    self.line_index = self.i;
                }       
                
                // Ignore whitespace
                w if w.is_whitespace() => {self.i += 1}

                // Single line comments
                '/' if program_text.get(self.i + 1) == Some(&'/') => self.lex_single_line_comment(program_text),

                // Multi-line comments
                '/' if program_text.get(self.i + 1) == Some(&'*') => self.lex_multi_line_comment(program_text),

                // Open brackets
                '(' => {
                    self.brackets.push(Bracket::Paren(self.tokens.len()));
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::OpenParen(0)));
                    self.i += 1;
                }
                '{' => {
                    self.brackets.push(Bracket::Brace(self.tokens.len()));
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::OpenBrace(0)));
                    self.i += 1;
                }
                '[' => {
                    self.brackets.push(Bracket::SquareBracket(self.tokens.len()));
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::OpenSquareBracket(0)));
                    self.i += 1;
                }

                // Close brackets
                ')' => {
                    let open_index = self.match_brackets(program, Bracket::Paren(0), token_start)?;
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::CloseParen(open_index)));
                    self.i += 1;
                }
                '}' => {
                    let open_index = self.match_brackets(program, Bracket::Brace(0), token_start)?;
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::CloseBrace(open_index)));
                    self.i += 1;
                }
                ']' => {
                    let open_index = self.match_brackets(program, Bracket::SquareBracket(0), token_start)?;
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::CloseSquareBracket(open_index)));
                    self.i += 1;
                }

                // An identifier
                c if is_identifier_start(c) => self.lex_identifier(program_text, token_start, program),

                // Any other character: should be an operator
                c => self.lex_operator(program_text, program, token_start, c)?,
            }
        }

        // After this loop, there should be no more items on the bracket stack
        // Return an error if self.brackets.pop() returns Some
        if let Some(b) = self.brackets.pop() {
            // Get the index of the opening bracket
            let open_index = match b {
                Bracket::Paren(i)
                | Bracket::Brace(i)
                | Bracket::SquareBracket(i) => i,
            };
            let location = self.tokens[open_index].location.clone();
            return Err(LexError { location, error_type: LexErrorType::UnclosedBracket });
        }

        Ok(self.tokens)
    }

    fn lex_single_line_comment(&mut self, program_text: &[char]) {
        self.i += 2;
        while let Some(c) = program_text.get(self.i) {
            if *c == '\n' {return}
            self.i += 1;
        } 
    }

    fn lex_multi_line_comment(&mut self, program_text: &[char]) {
        self.i += 2;
        loop {
            match program_text.get(self.i) {
                None => return,
                // Still track line / columns in a comment
                Some('\n') => {
                    self.i += 1;
                    self.line += 1;
                    self.line_index = self.i;
                }
                Some('*') if program_text.get(self.i + 1) == Some(&'/') => {
                    self.i += 2;
                    return
                },
                _ => self.i += 1,
            }
        }
    }

    fn lex_operator(&mut self, program_text: &[char], program: &Gc<Program>, token_start: usize, c: char) -> Result<(), LexError> {
        'test_operators: for (operator, operator_token) in OPERATORS {
            let Some(slice) = program_text.get(self.i..self.i+operator.len()) else {continue 'test_operators};
            // Get operators
            if slice.iter().map(char::to_owned).eq(operator.chars()) {
                self.i += operator.len();
                self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, operator_token));
                return Ok(());
            }
        }
        
        Err(LexError::new(program.clone(), self.line, self.line_index, token_start, LexErrorType::InvalidChar(c)))
    }

    fn lex_identifier(&mut self, program_text: &[char], token_start: usize, program: &Gc<Program>) {
        'chars_in_identifier: loop {
            match program_text.get(self.i) {
                None => {break 'chars_in_identifier},
                Some(&c) if is_identifier_continue(c) => self.i += 1,
                _ => break 'chars_in_identifier,
            }
        }
        let ident: String = program_text[token_start..self.i].iter().collect();
        self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::Identifier(ident)));
    }

    /// Checks that the given [`Bracket`] matched the top of the bracket stack, and sets the index of the corresponding open bracket token.
    /// Returns the index of the open bracket token
    fn match_brackets(&mut self, program: &Gc<Program>, close_bracket: Bracket, token_start: usize) -> Result<usize, LexError> {
        let Some(open_bracket) = self.brackets.pop() else {return Err(LexError::new(program.clone(), self.line, self.line_index, token_start, LexErrorType::MismatchedBracket))};
        let close_index = self.tokens.len();

        let open_index = match (open_bracket, close_bracket) {
            (Bracket::Paren(open_index), Bracket::Paren(_)) => {
                let TokenType::OpenParen(ref mut stored_close_index) = self.tokens[open_index].token_type else {panic!("Brackets got out of sync with tokens")};
                *stored_close_index = close_index;
                open_index
            },
            (Bracket::Brace(open_index), Bracket::Brace(_)) => {
                let TokenType::OpenBrace(ref mut stored_close_index) = self.tokens[open_index].token_type else {panic!("Brackets got out of sync with tokens")};
                *stored_close_index = close_index;
                open_index
            },
            (Bracket::SquareBracket(open_index), Bracket::SquareBracket(_)) => {
                let TokenType::OpenSquareBracket(ref mut stored_close_index) = self.tokens[open_index].token_type else {panic!("Brackets got out of sync with tokens")};
                *stored_close_index = close_index;
                open_index
            }
            _ => {
                return Err(LexError::new(program.clone(), self.line, self.line_index, token_start, LexErrorType::MismatchedBracket))
            }
        };

        Ok(open_index)
    }

    fn lex_string_literal(&mut self, program: &Gc<Program>, program_text: &[char], quote: char, token_start: usize) -> Result<(), LexError> {

        let mut s = String::new();
        'string: loop {
            self.i += 1;
            match program_text.get(self.i) {
                // Error on EOF
                None => return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::UnclosedString(quote))),
                // Error on newlines in the string
                // Does not error for backtick enclosed strings
                Some('\n') if quote != '`' => return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::NewlineInString(quote))),
                // If in a backtick string, update self.line on newline
                Some('\n') => {
                    self.line += 1;
                    self.line_index = self.i;
                }
                // Detect the end of the string
                Some(&c) if c == quote => break 'string,
                // Parse escape sequences
                Some('\\') => {
                    self.i += 1;
                    // Add character to string
                    s += &match program_text.get(self.i) {
                        None => return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::UnclosedString(quote))),
                        // self.line continuation
                        Some('\n') => String::new(),
                        // Newline
                        Some('n') => "\n".to_string(),
                        // Carriage return
                        Some('r') => "\r".to_string(),
                        // Tab
                        Some('t') => "\t".to_string(),
                        // Backspace
                        Some('b') => "\u{0008}".to_string(),
                        // Form feed
                        Some('f' | 'v') => "\u{000C}".to_string(),
                        // TODO: unicode strings
                        Some('u' | 'x') => todo!(),
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
        self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::String(s))));
        self.i += 1;
        
        Ok(())
    }

    fn lex_numeric_literal(&mut self, program: &Gc<Program>, program_text: &[char], token_start: usize) -> Result<(), LexError> {
        let digit = program_text[self.i];

        // Parse the number's prefix, for instance '0x'
        let base = if digit == '0' {
            self.i += 1;
            match self.parse_number_prefix(program_text, program, token_start)? {
                Some(base) => base,
                None => return Ok(()),
            }
        } else {
            NumberLiteralBase::Decimal
        };
        
        // Start of the digits
        let digits_start = self.i;
        // Whether there has been a decimal point yet
        let mut had_decimal = false;
        // The number for the string
        let mut number = String::new();

        'digits: loop {
            match program_text.get(self.i) {
                // Error on EOF
                None => {
                    if digits_start == self.i {
                        return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::MissingDigits(base)))
                    } 
                    break 'digits;
                },
                // Indicates a BigInt literal instead of a number
                Some('n') => {
                    if had_decimal {return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::IdentifierAfterNumber))}
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::BigInt(BigInt::from_str_radix(&number, base.get_radix()).expect("Should have been a valid bigint")))));
                    self.i += 1;
                    return Ok(());
                },
                // A digit
                Some(digit) if base.get_chars().contains(&digit.to_string()) => {number += &digit.to_string()},
                // Underscores are ignored in numeric literals
                Some('_') => (),
                Some('.') if base == NumberLiteralBase::Decimal => {
                    had_decimal = true;
                }
                // Error if an identifier is found
                Some(&id) if is_identifier_start(id) => {return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::IdentifierAfterNumber))},
                // Any other character means the end of the number
                _ => {
                    self.i -=1;
                    break 'digits;
                }
            }
            self.i += 1;
        }

        self.i += 1;

        if base == NumberLiteralBase::Decimal {
            let n = number.parse::<f64>().expect("Should have been a valid float");
            self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::Number(n))));
        }
        else {
            // Parse string to number
            let n = num::BigInt::from_str_radix(&number, base.get_radix()).expect("Should have been a valid bigint");
            let n = n.to_f64().unwrap_or(f64::INFINITY);

            self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::Number(n))));
        }

        Ok(())
    }

    fn parse_number_prefix(&mut self, program_text: &[char], program: &Gc<Program>, token_start: usize) -> Result<Option<NumberLiteralBase>, LexError> {
        let base  = match program_text.get(self.i) {
            // If EOF here, generate `NumberLiteral(0)`
            None => {
                self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::Number(0.0))));
                return Ok(None);
            },
            Some(&c) => match c {
                // Hex literal
                'x' | 'X' => {self.i += 1; NumberLiteralBase::Hex},
                // Octal literal
                'o' | 'O' | '0' => {self.i += 1; NumberLiteralBase::Octal},
                // Binary literal
                'b' | 'B' => {self.i += 1; NumberLiteralBase::Binary},
                // Bigint '0n' literal
                'n' => {
                    // Error if the next char is an identifier
                    if let Some(&c) = program_text.get(self.i + 1) {
                        if is_identifier_start(c) {
                            return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::IdentifierAfterNumber))
                        }
                    }
                    // Generate `BigIntLiteral(0)` token
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::BigInt(BigInt::from(0)))));
                    self.i += 2;
                    return Ok(None);
                },
                // TODO: error here in strict mode
                // Octal literal with no '0o' or '0O'
                c if ('1'..='9').contains(&c) => NumberLiteralBase::Octal,
                // Error if identifier encountered
                c if is_identifier_start(c) => {
                    return Err(LexError::new(program.clone(), self.line, self.line_index, self.i, LexErrorType::IdentifierAfterNumber))
                }
                _ => {
                    self.tokens.push(Token::new(program.clone(), self.line, self.line_index, token_start, TokenType::ValueLiteral(ValueLiteral::Number(0.0))));
                    return Ok(None);
                }
            }
        };

        Ok(Some(base))
    }
}