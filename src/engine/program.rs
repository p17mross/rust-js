use std::{path::PathBuf, fs, fmt::{Display, Debug}};

use crate::{Lexer, Parser, parser::ast::ASTNodeProgram};

use super::{error::{SyntaxError, ProgramFromFileError}, Gc, garbage_collection::{GarbageCollectable, GarbageCollectionId}};

#[derive(Debug, Clone)]
/// An enum for the source of a [Program].
/// Holds the type and location, so that the source of error messages can be printed.
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
    pub source: ProgramSource,
    /// The text of the program.
    /// Stored as a [Vec<char>] rather than [String] for easier indexing.
    pub program: Vec<char>,
    pub ast: Option<ASTNodeProgram>,
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Program {{source: {}, ast: {}}}", 
            self.source, 
            if self.ast.is_some() {"Some"} else {"None"}
        )
    }
}

impl GarbageCollectable for Program {
    fn get_children(&self) -> Vec<GarbageCollectionId> {
        match &self.ast {
            None => vec![],
            Some(a) => a.get_children()
        }
    }
}

impl Program {
    /// Parses the ast and sets self.ast to Some(AstNode)
    fn load_ast(s: &Gc<Self>) -> Result<(), SyntaxError> {
        // Lex
        let lexer = Lexer::new();
        let tokens = lexer.lex(s)?;
        // Parse
        let ast = Parser::parse(s.clone(), tokens)?;

        // Set s.ast
        s.borrow_mut().ast = Some(ast);

        Ok(())
    }

    /// Create a [`Program`] from a string with a [`ProgramSource`] of [`ProgramSource::Console`]
    /// 
    /// ### Errors
    /// * Returns a [`SyntaxError`] if the given string is not valid javascript code
    pub fn from_console(s: &str) -> Result<Gc<Self>, SyntaxError> {
        let program = Gc::new(Self {
            source: ProgramSource::Console,
            program: s.chars().collect(),
            ast: None
        });
        Self::load_ast(&program)?;
        Ok(program)
    }

    /// Load a [`Program`] from a file
    /// 
    /// ### Errors
    /// * Returns an [io error][std::io::Error] if there is an error reading from the file
    /// * Returns a [`SyntaxError`] if the given file does not contain valid javascript code
    pub fn from_file(p: PathBuf) -> Result<Gc<Self>, ProgramFromFileError> {
        let program = fs::read_to_string(p.clone())?;
        let program = Gc::new(Self {
            source: ProgramSource::File(p),
            program: program.chars().collect(),
            ast: None
        });
        Self::load_ast(&program)?;
        Ok(program)
    }
}

#[derive(Clone)]
/// Represents a line:column position in a program
pub struct ProgramLocation {
    /// The source of the program
    pub program: Gc<Program>,
    /// The line number
    pub line: usize,
    /// The column number
    pub column: usize,
    /// The index into [Program]::program
    pub index: usize,
}

impl Debug for ProgramLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} in {}", self.line, self.column, self.program.borrow().source)
    }
}