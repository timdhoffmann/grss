use std::{error::Error, io::{BufReader, BufRead}};

use clap::Parser;

/// Search for a string in a file and display all matching lines.
#[derive(Parser)]
struct Cli {
    /// The pattern to search for.
    pattern: String,
    /// The path to the file in which to search.
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    println!("{:?}, {:?}", args.pattern, args.path);

    let file = std::fs::File::open(&args.path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        if line.as_ref().unwrap().contains(&args.pattern) {
            println!("{}", line.unwrap());
        }
    }

    Ok(())
}
