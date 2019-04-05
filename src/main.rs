use structopt::StructOpt;

/// search for a pattern in a file and display the lines that contains it
#[derive(StructOpt)]
struct CLI {
    /// the pattern to look for
    pattern: String,
    // the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
}
