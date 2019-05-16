extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufReader, Read};

/*
    TODO:
    + test CLI (https://docs.rs/assert_cli/0.6.3/assert_cli/)
    + line number
    + filename
    + highlight matches
    + error reporting (https://rust-lang-nursery.github.io/cli-wg/in-depth/human-communication.html)
    + mimic most of `grep` features, for education purposes
        > ($ tldr grep)

    + [done] case insensitive (-i)
*/

fn main() {
    let ignore_case_flag = Arg::with_name("ignore-case")
        .short("i")
        .long("ignore-case")
        .help("Perform case insensitive matching. Default is case sensitive.");
    let pattern = Arg::with_name("PATTERN")
        .help("pattern to search, can use regular expression")
        .required(true)
        .index(1);
    let file = Arg::with_name("FILE")
        .help("path to file")
        .required(true)
        .index(2);

    let command = App::new("perg")
        .about("perg is a small command-line tool to search for given string inside a file")
        .arg(ignore_case_flag)
        .arg(pattern)
        .arg(file)
        .get_matches();

    // parse CLI arguments
    let path = command.value_of("FILE").unwrap();
    let pattern = command.value_of("PATTERN").unwrap();
    let file = File::open(&path).expect("could not read file");

    // read content of file and appending to data
    let mut data = String::new();
    let mut reader = BufReader::new(file);
    reader
        .read_to_string(&mut data)
        .expect("unable to read string aaaa");

    let should_ignore_case = command.is_present("ignore-case");
    perg::search(should_ignore_case, &data, &pattern, &mut std::io::stdout());
}
