use clap::Parser;

/// Search for a string in a file and display all matching lines.
#[derive(Parser)]
struct Cli {
    /// The pattern to search for.
    pattern: String,
    /// The path to the file in which to search.
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}, {:?}", args.pattern, args.path);
}
