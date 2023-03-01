//! Functionality to do with the execution of programs.
//! Currently very small as no execution has been implemented.

pub(crate) mod error;
pub(crate) mod garbage_collection;
pub(crate) mod program;

pub use error::{ProgramFromFileError, SyntaxError};
pub use garbage_collection::{
    GarbageCollectable, GarbageCollectionBorrowError, GarbageCollectionBorrowMutError,
    GarbageCollectionId, Gc,
};
pub use program::{Program, ProgramLocation, ProgramSource};

#[derive(Debug)]
#[allow(dead_code)]
/// A class for executing parsed [`Program`]s.
/// Currently has no functionality.
pub struct Engine {
    /// The program to run
    program: Program,
    // TODO: runtime state
}

impl Engine {
    /// Construct a new [`Engine`] from the given [`Program`]
    #[must_use]
    pub const fn new(program: Program) -> Self {
        Self { program }
    }
}

/// A configuration of the engine
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Whether to print debug output during execution
    pub debug: bool,
}

#[allow(clippy::derivable_impls)] // More properties may be added which may be incorrect if derived
impl Default for Config {
    fn default() -> Self {
        Self { debug: false }
    }
}
