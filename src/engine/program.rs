//! Contains most of the public methods to do with loading programs

use std::{
    fmt::{Debug, Display},
    fs,
    path::PathBuf,
};

use crate::{
    lexer::Lexer,
    parser::ast::{ASTNodeProgram, ToTree},
    parser::Parser,
};

use super::{
    error::{ProgramFromFileError, SyntaxError},
    garbage_collection::{GarbageCollectable, GarbageCollectionId, Gc},
    Config,
};

/// Holds the type and location, so that the source of error messages can be printed.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ProgramSource {
    /// The program was typed in a console
    Console,
    /// The program was passed to eval()
    EvalString,
    /// The program was loaded from a file
    File(PathBuf),
}

impl Display for ProgramSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Console => f.write_str("console code"),
            Self::EvalString => f.write_str(""),
            Self::File(p) => write!(f, "{}", p.to_str().unwrap_or("Unprintable file")),
        }
    }
}

/// A struct representing a javascript program
pub struct Program {
    /// The program source
    pub(crate) source: ProgramSource,
    /// The text of the program.
    /// Stored as a [`Vec<char>`] rather than [`String`] for easier indexing.
    pub(crate) program: Vec<char>,
    /// The AST, if it's been parsed
    pub(crate) ast: Option<ASTNodeProgram>,

    /// The configuration of the engine
    pub(crate) config: Config,
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Program {{source: {}, ast: {}}}",
            self.source,
            if self.ast.is_some() { "Some" } else { "None" }
        )
    }
}

impl GarbageCollectable for Program {
    fn get_children(&self) -> Vec<GarbageCollectionId> {
        match &self.ast {
            None => vec![],
            Some(a) => a.get_children(),
        }
    }
}

impl Program {
    /// Parses the AST and sets `self.ast` to `Some(AstNode)`
    fn load_ast(s: &Gc<Self>) -> Result<(), SyntaxError> {
        // Lex
        let lexer = Lexer::new();
        let tokens = lexer.lex(s)?;

        if s.borrow().config.debug {
            println!("[");
            for token in &tokens {
                println!("    {token:?},");
            }
            println!("]");
        }

        // Parse
        let ast = Parser::parse(s.clone(), tokens)?;

        // Set s.ast
        s.borrow_mut().ast = Some(ast);

        Ok(())
    }

    /// Create a [`Program`] from a string
    ///
    /// ### Errors
    /// * Returns a [`SyntaxError`] if the given string is not valid javascript code
    pub fn from_console(s: &str, config: Config) -> Result<Gc<Self>, SyntaxError> {
        let program = Gc::new(Self {
            source: ProgramSource::Console,
            program: s.chars().collect(),
            ast: None,
            config,
        });
        Self::load_ast(&program)?;
        Ok(program)
    }

    /// Load a [`Program`] from a file
    ///
    /// ### Errors
    /// * Returns an [io error][std::io::Error] if there is an error reading from the file
    /// * Returns a [`SyntaxError`] if the given file does not contain valid javascript code
    pub fn from_file(p: PathBuf, config: Config) -> Result<Gc<Self>, ProgramFromFileError> {
        let program = fs::read_to_string(p.clone())?;
        let program = Gc::new(Self {
            source: ProgramSource::File(p),
            program: program.chars().collect(),
            ast: None,
            config,
        });
        Self::load_ast(&program)?;
        Ok(program)
    }

    /// Prints the program's AST as a tree structure. This is meant for debugging purposes only and should not be user-facing.
    ///
    /// ### Panics
    /// * If an AST has not been generated.
    ///     This should never occur if the provided [`from_file`][Program::from_file] or [`from_console`][Program::from_console] functions are used.
    pub fn debug_ast(&self) {
        println!("{}", self.ast.as_ref().unwrap().to_tree());
    }

    /// Gets the [`Program`]'s [`ProgramSource`]
    #[must_use]
    pub fn get_source(&self) -> ProgramSource {
        self.source.clone()
    }
}

#[derive(Clone)]
/// Represents a line:column position in a program
pub struct ProgramLocation {
    /// The source of the program
    pub(crate) program: Gc<Program>,
    /// The line number
    pub(crate) line: usize,
    /// The column number
    pub(crate) column: usize,
    /// The character index into [`Program::program`]
    pub(crate) index: usize,
}

impl ProgramLocation {
    /// Gets the [`Program`] this [`ProgramLocation`] is a location in
    #[must_use]
    pub fn get_program(&self) -> Gc<Program> {
        self.program.clone()
    }

    /// Gets the 1-based line number of this [`ProgramLocation`]
    #[must_use]
    pub fn get_line(&self) -> usize {
        self.line
    }

    /// Gets the 1-based column number of this [`ProgramLocation`], measured in code points from the start of the line
    #[must_use]
    pub fn get_column(&self) -> usize {
        self.column
    }

    /// Gets the 0-based offset of this [`ProgramLocation`] from the beginning of the code. Measured in code points.
    #[must_use]
    pub fn get_index(&self) -> usize {
        self.index
    }
}

impl Debug for ProgramLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} in {}",
            self.line,
            self.column,
            self.program.borrow().source
        )
    }
}
