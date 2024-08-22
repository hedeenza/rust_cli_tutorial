use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
# [derive(Parser)]
struct Cli {
    /// The pattern to look for 
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

}

/// pick back up at start of 1.3
/// https://rust-cli.github.io/book/tutorial/impl-draft.html
