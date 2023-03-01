//! A javascript engine.
//! Not yet feature complete - see readme for progress

#![warn(
    clippy::all,
    clippy::pedantic,
    //clippy::nursery
    missing_docs,
    clippy::missing_docs_in_private_items
)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]

mod engine;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod util;

pub use engine::{Config, Engine, Program};
pub use engine::{
    GarbageCollectable, GarbageCollectionBorrowError, GarbageCollectionBorrowMutError,
    GarbageCollectionId, Gc,
};
pub use lexer::LexError;
pub use parser::ParseError;

mod tests;
