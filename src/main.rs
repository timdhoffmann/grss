use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

/// Search for a string in a file and display all matching lines.
#[derive(Parser)]
struct Cli {
    /// The pattern to search for.
    pattern: String,
    /// The path to the file in which to search.
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("{:?}, {:?}", args.pattern, args.path);

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;
    let reader = BufReader::new(file);
    // let lines = reader.lines();

    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    for mut line in reader.lines() {
        if line.as_mut().unwrap().contains(pattern) {
            writeln!(writer, "{}", line.unwrap());
        }
        // if line.as_ref().unwrap().contains(&args.pattern) {
        // println!("{}", line.unwrap());
        // }
    }
}

// #[test]
// fn find_match() {
//     let content = "test".to_string();
//     let reader = BufReader::new(content);
// }