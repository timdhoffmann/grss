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

    let content = std::fs::read_to_string(&args.path).expect("Could not read file.");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{line}");
        }
    }
}
