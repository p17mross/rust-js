use std::{env, path::PathBuf};

use js::engine::Program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                    return Err(format!("Unknown flag '-{s}'").into());
                }
            }
        }
        else {
            if filepath.is_some() {
                return Err("Found multiple file names".to_string().into());
            }
            filepath = Some(arg.clone());
        }
    }

    let Some(filepath) = filepath else {
        return Err("Expected file name".to_string().into());
    };

    let program = Program::from_file(PathBuf::from(filepath))?;

    println!("{program:?}");

    // TODO: run the code

    Ok(())
}
