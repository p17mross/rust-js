#![cfg(test)]

use crate::engine::{Gc, ProgramSource, Config};
use crate::lexer::Lexer;
use crate::{parser::Parser, Program};

mod parsing;