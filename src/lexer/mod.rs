pub(crate) mod token;
pub(crate) mod error;
mod literals;

pub(crate) use token::{Token, TokenType};

pub use error::{LexError, LexErrorType};

use crate::engine::{Gc, Program};
use crate::util::{is_identifier_continue, is_identifier_start};

use self::token::OPERATORS;

#[derive(Debug, Clone, Copy)]
enum Bracket {
    Paren(usize),
    Brace(usize),
    SquareBracket(usize),
}

#[derive(Debug)]
/// Struct responsible for lexical analysis.
pub(super) struct Lexer {
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
    /// Creates a new [`Lexer`] with default values
    pub(super) const fn new() -> Self {
        Self {
            i: 0,
            line: 1,
            line_index: 0,

            tokens: Vec::new(),
            brackets: Vec::new(),
        }
    }

    fn get_char(&mut self, program_text: &[char]) -> Option<char> {
        match program_text.get(self.i) {
            Some(c) => {
                self.i += 1;
                Some(*c)
            }
            None => None
        }
    }

    /// Constructs a list of tokens from a string.
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

            // Break on EOF
            if program_text.get(self.i).is_none() {
                break 'tokens;
            };

            self.lex_token(program, program_text)?;
        }

        // After this loop, there should be no more items on the bracket stack
        // Return an error if self.brackets.pop() returns Some
        if let Some(b) = self.brackets.pop() {
            // Get the index of the opening bracket
            let open_index = match b {
                Bracket::Paren(i) | Bracket::Brace(i) | Bracket::SquareBracket(i) => i,
            };
            let location = self.tokens[open_index].location.clone();
            return Err(LexError {
                location,
                error_type: LexErrorType::UnclosedBracket,
            });
        }

        Ok(self.tokens)
    }

    /// Lexes one token (in this context, a newline counts as a token)
    ///
    /// ### Params
    /// * `token_start_char`: the character which the token starts with
    /// * `program`: the program which the tokens are being lexed from
    /// * `program_text`: the source to parse a token from
    fn lex_token(
        &mut self,
        program: &Gc<Program>,
        program_text: &[char],
    ) -> Result<(), LexError> {
        let token_start = self.i;
        // This should never fail as it will be checked before the function is called
        let token_start_char = self.get_char(program_text).unwrap();

        match token_start_char {
            // String literal
            quote if quote == '"' || quote == '\'' || quote == '`' => {
                self.lex_string_literal(program, program_text, quote, token_start)?;
            }

            // Number or BigInt literal
            digit if ('0'..='9').contains(&digit) => {
                self.i -= 1;
                self.lex_numeric_literal(program, program_text, token_start)?;
            }
            // Number with leading decimal point e.g. '.25'
            '.' if program_text.get(self.i).map(char::is_ascii_digit) == Some(true) => {
                self.i -= 1;
                self.lex_numeric_literal(program, program_text, token_start)?;
            }

            // Newline
            '\n' => {
                if let Some(t) = self.tokens.last_mut() {
                    t.newline_after = true;
                }
                self.line += 1;
                self.line_index = self.i;
            }

            // Ignore whitespace
            w if w.is_whitespace() => (),

            // Single line comments
            '/' if program_text.get(self.i) == Some(&'/') => {
                self.lex_single_line_comment(program_text);
            }

            // Multi-line comments
            '/' if program_text.get(self.i) == Some(&'*') => {
                self.lex_multi_line_comment(program, program_text, token_start)?;
            }

            // Open brackets
            '(' => {
                self.produce_bracket(program, token_start, Bracket::Paren(self.tokens.len()));
            }

            '{' => {
                self.produce_bracket(program, token_start, Bracket::Brace(self.tokens.len()));
            }
            '[' => {
                self.produce_bracket(
                    program,
                    token_start,
                    Bracket::SquareBracket(self.tokens.len()),
                );
            }

            // Close brackets
            ')' => {
                self.match_brackets(program, Bracket::Paren(0), token_start)?;
            }
            '}' => {
                self.match_brackets(program, Bracket::Brace(0), token_start)?;
            }
            ']' => {
                self.match_brackets(program, Bracket::SquareBracket(0), token_start)?;
            }

            // An identifier
            c if is_identifier_start(c) => {
                self.i -= 1;
                self.lex_identifier(program_text, token_start, program);
            }

            // Any other character: should be an operator
            c => {
                self.i -= 1;
                self.lex_operator(program_text, program, token_start, c)?;
            }
        }

        Ok(())
    }

    /// Pushes the given bracket type to the [bracket stack][Lexer::brackets] and produces a token of the corresponding open bracket type
    fn produce_bracket(
        &mut self,
        program: &Gc<Program>,
        token_start: usize,
        bracket_type: Bracket,
    ) {
        self.brackets.push(bracket_type);

        let token_type = match bracket_type {
            Bracket::Paren(_) => TokenType::OpenParen(0),
            Bracket::Brace(_) => TokenType::OpenBrace(0),
            Bracket::SquareBracket(_) => TokenType::OpenSquareBracket(0),
        };

        self.tokens.push(Token::new(
            program.clone(),
            self.line,
            self.line_index,
            token_start,
            token_type,
        ));
    }

    /// Lex a line comment by consuming characters until a newline. 
    /// The newline character will not be consumed, which allows line numbers to be properly tracked and semicolon insertion to function correctly
    fn lex_single_line_comment(&mut self, program_text: &[char]) {
        while let Some(c) = self.get_char(program_text) {
            if c == '\n' {
                // Make sure the newline is lexed
                self.i -= 1;
                return;
            }
        }
    }

    /// Lex a multi-line comment by consuming characters until a '*/' is encountered, which will be consumed.
    /// Line and column numbers will be tracked inside the comment.
    fn lex_multi_line_comment(&mut self, program: &Gc<Program>, program_text: &[char], token_start: usize) -> Result<(), LexError> {
        loop {
            match self.get_char(program_text) {
                None => return Err(LexError::new(program.clone(), self.line, self.line_index, token_start, LexErrorType::UnclosedComment)),
                // Still track line / columns in a comment
                Some('\n') => {
                    self.line_index = self.i;
                }
                Some('*') if self.get_char(program_text) == Some('/') => {
                    return Ok(());
                }
                _ => (),
            }
        }
    }

    /// Lexes an operator based on the string to operator mapping in [`OPERATORS`].
    fn lex_operator(
        &mut self,
        program_text: &[char],
        program: &Gc<Program>,
        token_start: usize,
        c: char,
    ) -> Result<(), LexError> {
        // Loop over all possible operators
        'test_operators: for (operator, operator_token) in OPERATORS {
            // Get a [char] slice of the same length as the operator
            let Some(slice) = program_text.get(self.i..self.i+operator.len()) else {continue 'test_operators};
            
            // If the operator matches the current text, produce that operator
            if slice.iter().map(char::to_owned).eq(operator.chars()) {
                self.i += operator.len();
                self.tokens.push(Token::new(
                    program.clone(),
                    self.line,
                    self.line_index,
                    token_start,
                    operator_token,
                ));
                return Ok(());
            }
        }

        // If no operators were found, return an error
        Err(LexError::new(
            program.clone(),
            self.line,
            self.line_index,
            token_start,
            LexErrorType::InvalidChar(c),
        ))
    }

    /// Lexes an identifier by consuming characters until a character which is not [an identifier][is_identifier_continue] is encountered
    fn lex_identifier(&mut self, program_text: &[char], token_start: usize, program: &Gc<Program>) {
        'chars_in_identifier: loop {
            match self.get_char(program_text) {
                Some(c) if is_identifier_continue(c) => (),
                _ => {
                    self.i -= 1;
                    break 'chars_in_identifier
                },
            }
        }
        let ident: String = program_text[token_start..self.i].iter().collect();
        self.tokens.push(Token::new(
            program.clone(),
            self.line,
            self.line_index,
            token_start,
            TokenType::Identifier(ident),
        ));
    }

    /// Checks that the given [`Bracket`] matched the top of the bracket stack, and sets the index of the corresponding open bracket token.
    /// Returns the index of the open bracket token
    fn match_brackets(
        &mut self,
        program: &Gc<Program>,
        close_bracket: Bracket,
        token_start: usize,
    ) -> Result<(), LexError> {
        let Some(open_bracket) = self.brackets.pop() else {return Err(LexError::new(program.clone(), self.line, self.line_index, token_start, LexErrorType::MismatchedBracket))};
        let close_index = self.tokens.len();

        let token_type = match (open_bracket, close_bracket) {
            (Bracket::Paren(open_index), Bracket::Paren(_)) => {
                let TokenType::OpenParen(ref mut stored_close_index) = self.tokens[open_index].token_type else {panic!("Brackets got out of sync with tokens")};
                *stored_close_index = close_index;
                TokenType::CloseParen(open_index)
            }
            (Bracket::Brace(open_index), Bracket::Brace(_)) => {
                let TokenType::OpenBrace(ref mut stored_close_index) = self.tokens[open_index].token_type else {panic!("Brackets got out of sync with tokens")};
                *stored_close_index = close_index;
                TokenType::CloseBrace(open_index)
            }
            (Bracket::SquareBracket(open_index), Bracket::SquareBracket(_)) => {
                let TokenType::OpenSquareBracket(ref mut stored_close_index) = self.tokens[open_index].token_type else {panic!("Brackets got out of sync with tokens")};
                *stored_close_index = close_index;
                TokenType::CloseSquareBracket(open_index)
            }
            _ => {
                return Err(LexError::new(
                    program.clone(),
                    self.line,
                    self.line_index,
                    token_start,
                    LexErrorType::MismatchedBracket,
                ))
            }
        };

        self.tokens.push(Token::new(
            program.clone(),
            self.line,
            self.line_index,
            token_start,
            token_type,
        ));

        Ok(())
    }
}
