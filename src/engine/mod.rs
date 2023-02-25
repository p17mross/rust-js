pub(crate) mod garbage_collection;
pub(crate) mod program;
pub(crate) mod error;

pub use garbage_collection::Gc;
pub use program::Program;

#[derive(Debug)]
#[allow(dead_code)]
/// A class for executing parsed [Program]s.
/// Currently has no functionality.
pub struct Engine {
    program: Program,
    // TODO: runtime state
}

impl Engine {
    #[must_use]
    pub const fn new(program: Program) -> Self {
        Self { program }
    }
}
