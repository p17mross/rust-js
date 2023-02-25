#![warn(
    clippy::all,
    clippy::pedantic,
    //clippy::nursery
)]

use std::{env, path::PathBuf};

use js::engine::Program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut filepath: Option<String> = None;

    let mut args = args.iter();
    // First arg is path to binary, so ignore it
    args.next();

    // This is so that items can be manually skipped
    #[allow(clippy::while_let_on_iterator)]
    while let Some(arg) = args.next() {
        let mut char_iter = arg.chars();
        if char_iter.next().unwrap() == '-' {
            // This is so that command line flags can be implemented in the future
            #[allow(clippy::match_single_binding)]
            match char_iter.as_str() {
                s => {
                    return Err(format!("Unknown flag '-{s}'").into());
                }
            }
        }

        if filepath.is_some() {
            return Err("Found multiple file names".to_string().into());
        }
        filepath = Some(arg.clone());
    }

    let Some(filepath) = filepath else {
        return Err("Expected file name".to_string().into());
    };

    let program = Program::from_file(PathBuf::from(filepath))?;

    let program_ref = program.borrow();

    program_ref.debug_ast();

    // TODO: run the code

    Ok(())
}
