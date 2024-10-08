use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
# [derive(Parser)]
struct Cli {
    /// The pattern to look for 
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error:Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}


//fn main() -> Result<(), Box<dyn std::error:Error>> {
    //let result = std::fs::read_to_string("test.txt");
    //let content = match result {
        //Ok(content) => { content },
        //Err(error) => { return Err(error.into()); }
    //};
    //println!("file content: {}", content);
    //Ok(())
//}

//fn main() {
    //let args = Cli::parse();
//
    //let content = std::fs::read_to_string(&args.path).expect("could not read file");
//
    //for line in content.lines() {
        //if line.contains(&args.pattern) {
            //println!("{}", line);
        //}
    //}
//
//}

// pick back up at start of 1.3
// https://rust-cli.github.io/book/tutorial/impl-draft.html
/ /   P i c k   u p   a t   1 . 4   -   ' P r o v i d i n g   C o n t e x t '  
 