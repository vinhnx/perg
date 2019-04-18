extern crate structopt;
use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[derive(Debug)]
struct CLI {
    pattern: String,
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
    let args = CLI::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    perg::find_matches(&content, &args.pattern, &mut std::io::stdout());
}
