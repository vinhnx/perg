extern crate regex;
extern crate structopt;

use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use structopt::StructOpt;
use std::env;

/*
    TODO:
    + case insensitive (-i)
    + highlight matches
    + mimic most of `grep` features, for education purposes
        > https://www.digitalocean.com/community/tutorials/using-grep-regular-expressions-to-search-for-text-patterns-in-linux#regular-expressions
*/

#[derive(StructOpt, Debug)]
struct CLI {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl fmt::Display for CLI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CLI: \n> args: {}\n> path: {:?}",
            self.pattern, self.path
        )
    }
}

fn main() {
    let vargs: Vec<String> = env::args().collect();
    println!("{:?}", vargs);

    // parse CLI arguments
    // let args = CLI::from_args();
    let file = File::open(&args.path).expect("could not read file");

    // // read content of file and appending to data
    // let mut data = String::new();
    // let mut reader = BufReader::new(file);
    // reader
    //     .read_to_string(&mut data)
    //     .expect("unable to read string");

    // // convert pattern into regular expression
    // let re = Regex::new(&args.pattern).unwrap();
    // perg::search(&data, &re, &mut std::io::stdout());
}
