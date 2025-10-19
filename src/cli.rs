use clap::Parser;

/// perg - A fast text search tool similar to grep
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pattern to search for (supports regular expressions)
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    /// Files or directories to search in
    #[arg(value_name = "PATH")]
    pub paths: Vec<String>,

    /// Perform case insensitive matching
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Show line numbers
    #[arg(short = 'n', long)]
    pub line_number: bool,

    /// Show filenames
    #[arg(short = 'H', long)]
    pub with_filename: bool,

    /// Recursively search directories
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Suppress error messages about inaccessible files
    #[arg(short = 's', long)]
    pub no_messages: bool,

    /// Invert match: show lines that do NOT match the pattern
    #[arg(short = 'v', long)]
    pub invert_match: bool,

    /// Only show filenames that contain matches
    #[arg(short = 'l', long)]
    pub files_with_matches: bool,

    /// Only show filenames that do NOT contain matches
    #[arg(short = 'L', long)]
    pub files_without_match: bool,

    /// Print a count of matching lines for each input file
    #[arg(short = 'c', long)]
    pub count: bool,

    /// Print NUM lines of leading context before matching lines
    #[arg(short = 'B', long = "before-context", default_value_t = 0)]
    pub before_context: usize,

    /// Print NUM lines of trailing context after matching lines
    #[arg(short = 'A', long = "after-context", default_value_t = 0)]
    pub after_context: usize,

    /// Print NUM lines of output context
    #[arg(short = 'C', long = "context", default_value_t = 0)]
    pub context: usize,

    /// Stop reading a file after NUM matching lines
    #[arg(short = 'm', long = "max-count")]
    pub max_count: Option<usize>,

    /// Print only the matched (non-empty) parts of a matching line
    #[arg(short = 'o', long = "only-matching")]
    pub only_matching: bool,
}
