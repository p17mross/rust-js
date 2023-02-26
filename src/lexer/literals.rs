use num::{BigInt, Num, ToPrimitive};

use crate::util::NumberLiteralBase;

use super::{token::ValueLiteral, *};

impl Lexer {
    pub(super) fn lex_string_literal(
        &mut self,
        program: &Gc<Program>,
        program_text: &[char],
        quote: char,
        token_start: usize,
    ) -> Result<(), LexError> {
        // Save the line and line_index as they might change while parsing the string
        let start_line = self.line;
        let start_line_index = self.line_index;

        let mut s = String::new();
        'string: loop {
            self.i += 1;
            match program_text.get(self.i) {
                // Error on EOF
                None => {
                    return Err(LexError::new(
                        program.clone(),
                        self.line,
                        self.line_index,
                        self.i,
                        LexErrorType::UnclosedString(quote),
                    ))
                }
                // Error on newlines in the string
                // Does not error for backtick enclosed strings
                Some('\n') if quote != '`' => {
                    return Err(LexError::new(
                        program.clone(),
                        self.line,
                        self.line_index,
                        self.i,
                        LexErrorType::NewlineInString(quote),
                    ))
                }
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
                        None => {
                            return Err(LexError::new(
                                program.clone(),
                                self.line,
                                self.line_index,
                                self.i,
                                LexErrorType::UnclosedString(quote),
                            ))
                        }
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
                        Some(c) => c.to_string(),
                    };
                }
                // If any other char, add it to the string
                Some(c) => {
                    s += &c.to_string();
                }
            }
        }
        self.tokens.push(Token::new(
            program.clone(),
            start_line,
            start_line_index,
            token_start,
            TokenType::ValueLiteral(ValueLiteral::String(s)),
        ));
        self.i += 1;

        Ok(())
    }

    pub(super) fn lex_numeric_literal(
        &mut self,
        program: &Gc<Program>,
        program_text: &[char],
        token_start: usize,
    ) -> Result<(), LexError> {
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
                        return Err(LexError::new(
                            program.clone(),
                            self.line,
                            self.line_index,
                            self.i,
                            LexErrorType::MissingDigits { base },
                        ));
                    }
                    break 'digits;
                }
                // Indicates a BigInt literal instead of a number
                Some('n') => return self.check_bigint(
                    had_decimal,
                    program,
                    base,
                    program_text,
                    token_start,
                    &number,
                ),
                // A digit
                Some(digit) if base.get_chars().contains(&digit.to_string()) => {
                    number += &digit.to_string();
                }
                // Underscores are ignored in numeric literals
                Some('_') => (),
                Some('.') if base == NumberLiteralBase::Decimal => {
                    had_decimal = true;
                }
                // Error if an identifier is found
                Some(&id) if is_identifier_start(id) => {
                    return Err(LexError::new(
                        program.clone(),
                        self.line,
                        self.line_index,
                        self.i,
                        LexErrorType::IdentifierAfterNumber,
                    ))
                }
                // Any other character means the end of the number
                _ => {
                    self.i -= 1;
                    break 'digits;
                }
            }
            self.i += 1;
        }

        self.i += 1;

        // Parse string to number
        let n = num::BigInt::from_str_radix(&number, base.get_radix())
            .expect("Should have been a valid bigint");
        let n = n.to_f64().unwrap_or(f64::INFINITY);

        self.tokens.push(Token::new(
            program.clone(),
            self.line,
            self.line_index,
            token_start,
            TokenType::ValueLiteral(ValueLiteral::Number(n)),
        ));

        Ok(())
    }

    /// Checks whether a `BigInt` literal is valid and produces it if it is
    fn check_bigint(
        &mut self,
        had_decimal: bool,
        program: &Gc<Program>,
        base: NumberLiteralBase,
        program_text: &[char],
        token_start: usize,
        number: &str,
    ) -> Result<(), LexError> {
        // BigInt literals can't have decimal points
        if had_decimal {
            return Err(LexError::new(
                program.clone(),
                self.line,
                self.line_index,
                self.i,
                LexErrorType::IdentifierAfterNumber,
            ));
        }

        // BigInt literals can't be implicitly octal e.g. '012n' is not allowed
        if base == NumberLiteralBase::OctalImplicit {
            return Err(LexError::new(
                program.clone(),
                self.line,
                self.line_index,
                self.i,
                LexErrorType::InvalidBigInt,
            ));
        }

        // Identifiers can't come straight after a bigint literal e.g. '10na' is not allowed
        if let Some(c) = program_text.get(self.i + 1) {
            if is_identifier_start(*c) {
                return Err(LexError::new(
                    program.clone(),
                    self.line,
                    self.line_index,
                    self.i,
                    LexErrorType::IdentifierAfterNumber,
                ));
            }
        }

        self.tokens.push(Token::new(
            program.clone(),
            self.line,
            self.line_index,
            token_start,
            TokenType::ValueLiteral(ValueLiteral::BigInt(
                BigInt::from_str_radix(number, base.get_radix())
                    .expect("Should have been a valid bigint"),
            )),
        ));
        self.i += 1;

        Ok(())
    }

    /// Parses the letter which comes after the first `0` in a number literal which specifies the base, e.g. the `x` in `0x10`.
    ///
    /// ### Returns
    /// The base if one was found, or `Ok(None)` if no base was found and the number literal is already parsed by this function.
    /// E.g. in the case of ` 0 `, this function will produce a [number literal][TokenType::ValueLiteral] token and return `Ok(None)`
    fn parse_number_prefix(
        &mut self,
        program_text: &[char],
        program: &Gc<Program>,
        token_start: usize,
    ) -> Result<Option<NumberLiteralBase>, LexError> {
        let base = match program_text.get(self.i) {
            // If EOF here, generate `NumberLiteral(0)`
            None => {
                self.tokens.push(Token::new(
                    program.clone(),
                    self.line,
                    self.line_index,
                    token_start,
                    TokenType::ValueLiteral(ValueLiteral::Number(0.0)),
                ));
                return Ok(None);
            }
            Some(&c) => match c {
                // Hex literal
                'x' | 'X' => {
                    self.i += 1;
                    NumberLiteralBase::Hex
                }
                // Octal literal
                'o' | 'O' | '0' => {
                    self.i += 1;
                    NumberLiteralBase::Octal
                }
                // Binary literal
                'b' | 'B' => {
                    self.i += 1;
                    NumberLiteralBase::Binary
                }
                // Bigint '0n' literal
                'n' => {
                    // Error if the next char is an identifier
                    if let Some(&c) = program_text.get(self.i + 1) {
                        if is_identifier_start(c) {
                            return Err(LexError::new(
                                program.clone(),
                                self.line,
                                self.line_index,
                                self.i,
                                LexErrorType::IdentifierAfterNumber,
                            ));
                        }
                    }
                    // Generate `BigIntLiteral(0)` token
                    self.tokens.push(Token::new(
                        program.clone(),
                        self.line,
                        self.line_index,
                        token_start,
                        TokenType::ValueLiteral(ValueLiteral::BigInt(BigInt::from(0))),
                    ));
                    self.i += 2;
                    return Ok(None);
                }
                // TODO: error here in strict mode
                // Octal literal with no '0o' or '0O'
                c if ('1'..='9').contains(&c) => NumberLiteralBase::OctalImplicit,
                // Error if identifier encountered
                c if is_identifier_start(c) => {
                    return Err(LexError::new(
                        program.clone(),
                        self.line,
                        self.line_index,
                        self.i,
                        LexErrorType::IdentifierAfterNumber,
                    ))
                }
                _ => {
                    self.tokens.push(Token::new(
                        program.clone(),
                        self.line,
                        self.line_index,
                        token_start,
                        TokenType::ValueLiteral(ValueLiteral::Number(0.0)),
                    ));
                    return Ok(None);
                }
            },
        };

        Ok(Some(base))
    }
}
