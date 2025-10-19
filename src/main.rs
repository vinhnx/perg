use clap::Parser;
use perg::cli::Args;
use perg::error::PergError;
use perg::search::{search_paths, search_stdin, SearchConfig};
use std::process;

/// Main entry point for the perg command-line tool.
/// 
/// The tool supports reading from files or stdin, with various search options
/// including regular expressions, context lines, counting, and more.
fn main() {
    let args = Args::parse();

    // Validate arguments
    if args.files_with_matches && args.files_without_match {
        eprintln!("perg: cannot specify both -l and -L");
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
        args.count,
        args.before_context,
        args.after_context,
        args.context,
        args.max_count,
        args.only_matching,
        args.extended_regexp,
        args.color,
    );

    // Perform search
    let mut stdout = std::io::stdout();
    let result = if args.paths.is_empty() {
        // Search stdin when no paths provided
        search_stdin(&config, &mut stdout)
    } else {
        // Perform search on paths
        search_paths(
            &config,
            &args.paths,
            args.recursive,
            args.no_messages,
            &mut stdout,
        )
    };

    match result {
        Ok(_) => process::exit(0),
        Err(PergError::FileNotFound(_)) => process::exit(1),
        Err(PergError::Regex(_)) => process::exit(2),
        Err(_) => process::exit(1),
    }
}
