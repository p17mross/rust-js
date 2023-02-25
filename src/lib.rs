#![warn(
    clippy::all,
    clippy::pedantic,
    //clippy::nursery
)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]

pub mod engine;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod util;

pub use engine::Engine;
pub use engine::Program;
pub use lexer::LexError;
pub use parser::ParseError;
