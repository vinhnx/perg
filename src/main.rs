use clap::Parser;
use perg::cli::Args;
use perg::error::PergError;
use perg::search::{search_paths, SearchConfig};
use std::process;

fn main() {
    let args = Args::parse();

    // Validate arguments
    if args.files_with_matches && args.files_without_match {
        eprintln!("perg: cannot specify both -l and -L");
        process::exit(1);
    }

    if args.paths.is_empty() {
        eprintln!("perg: no path specified");
        process::exit(1);
    }

    // Create search configuration
    let config = SearchConfig::new(
        args.pattern,
        args.ignore_case,
        args.line_number,
        args.with_filename,
        args.invert_match,
        args.files_with_matches,
        args.files_without_match,
    );

    // Perform search
    let mut stdout = std::io::stdout();
    match search_paths(
        &config,
        &args.paths,
        args.recursive,
        args.no_messages,
        &mut stdout,
    ) {
        Ok(_) => process::exit(0),
        Err(PergError::FileNotFound(_)) => process::exit(1),
        Err(PergError::Regex(_)) => process::exit(2),
        Err(_) => process::exit(1),
    }
}
