mod lexer;
mod engine;
mod util;

use lexer::Lexer;
use engine::Engine;

use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let mut filepath: Option<String> = None;

    let mut args = args.iter();
    // First arg is path to binary, so ignore it
    args.next();

    while let Some(arg) = args.next() {
        let mut char_iter = arg.chars();
        if char_iter.next().unwrap() == '-' {
            match char_iter.as_str() {
                s => {
                    return Err(format!("Unknown flag '-{s}'"));
                }
            }
        }
        else {
            if filepath.is_some() {
                return Err("Found multiple file names".to_string());
            }
            filepath = Some(arg.clone());
        }
    }

    let Some(filepath) = filepath else {
        return Err("Expected file name".to_string());
    };

    let Ok(program) = fs::read_to_string(filepath.clone()) else {
        return Err(format!("File not found: '{filepath}'"));
    };

    let mut engine = Engine::new();
    engine.parse(&program);

    Ok(())
}
