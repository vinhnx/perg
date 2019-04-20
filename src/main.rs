extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufReader, Read};

/*
    TODO:
    + [done] case insensitive (-i)
    + highlight matches
    + error reporting
    + mimic most of `grep` features, for education purposes
        > https://www.digitalocean.com/community/tutorials/using-grep-regular-expressions-to-search-for-text-patterns-in-linux#regular-expressions
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
        .expect("unable to read string");

    let should_ignore_case = command.is_present("ignore-case");
    if should_ignore_case {
        perg::search_case_insensitive(&data, &pattern, &mut std::io::stdout());
    } else {
        perg::search(&data, &pattern, &mut std::io::stdout());
    }
}
