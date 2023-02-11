pub(crate) mod garbage_collection;
pub(crate) use garbage_collection::Gc;

pub mod program;
pub use program::Program;
pub mod error;

#[derive(Debug)]
#[allow(dead_code)]
/// A class for executing parsed [Program]s.
/// Currently has no functionality.
pub struct Engine {
    program: Program,
    // TODO: runtime state
}

impl Engine {
    pub fn new(program: Program) -> Self {
        Engine {
            program,
        }
    }
}