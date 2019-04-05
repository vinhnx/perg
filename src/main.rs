use std::fmt;
use structopt::StructOpt;

/// search for a pattern in a file and display the lines that contains it
#[derive(StructOpt)]
#[derive(Debug)]
struct CLI {
    /// the pattern to look for
    pattern: String,
    // the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl fmt::Display for CLI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLI: \n> args: {}\n> path: {:?}", 
            self.pattern, self.path
        )
    }
}

fn main() {
    // Gets the struct from the command line arguments.
    let args = CLI::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
