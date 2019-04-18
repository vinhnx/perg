extern crate structopt;

use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
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
    // parse CLI arguments
    let args = CLI::from_args();
    let file = File::open(&args.path)
        .expect("could not read file");
    
    // read content of file and appending to data
    let mut data = String::new();
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut data)
        .expect("unable to read string");

    // find matching patterns  
    perg::find_matches(&data, &args.pattern, &mut std::io::stdout());
}
