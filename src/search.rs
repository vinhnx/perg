use crate::error::{PergError, Result};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use walkdir::WalkDir;

/// Search configuration
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub pattern: String,
    pub ignore_case: bool,
    pub line_number: bool,
    pub with_filename: bool,
    pub invert_match: bool,
    pub files_with_matches: bool,
    pub files_without_match: bool,
}

impl SearchConfig {
    pub fn new(
        pattern: String,
        ignore_case: bool,
        line_number: bool,
        with_filename: bool,
        invert_match: bool,
        files_with_matches: bool,
        files_without_match: bool,
    ) -> Self {
        Self {
            pattern,
            ignore_case,
            line_number,
            with_filename,
            invert_match,
            files_with_matches,
            files_without_match,
        }
    }
}

/// Search result for a single match
#[derive(Debug)]
pub struct MatchResult {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
}

/// Search for pattern in a single file
pub fn search_file(
    config: &SearchConfig,
    file_path: &str,
    writer: &mut impl Write,
) -> Result<bool> {
    let path = Path::new(file_path);

    // Handle directory case for files_with_matches/files_without_match
    if path.is_dir() && (config.files_with_matches || config.files_without_match) {
        // For directories in these modes, we consider them as having no matches
        // since directories themselves don't contain searchable text
        if config.files_without_match {
            writeln!(writer, "{}", file_path)?;
            return Ok(false);
        }
        return Ok(false);
    }

    let file = File::open(path).map_err(|_| PergError::FileNotFound(file_path.to_string()))?;
    let reader = BufReader::new(file);

    let pattern = if config.ignore_case {
        format!("(?i){}", config.pattern)
    } else {
        config.pattern.clone()
    };

    let regex = Regex::new(&pattern)?;

    let mut has_matches = false;
    let mut line_number = 0;

    for line in reader.lines() {
        line_number += 1;
        let line = line?;
        let matches = regex.is_match(&line);

        // Apply invert match logic
        let should_include = if config.invert_match { !matches } else { matches };

        if should_include {
            has_matches = true;

            // For files_with_matches/files_without_match modes, we just need to know if there are matches
            if config.files_with_matches || config.files_without_match {
                continue;
            }

            // Format and write the match
            let output = format_match(config, file_path, line_number, &line);
            writeln!(writer, "{}", output)?;
        }
    }

    // Handle files_with_matches/files_without_match output
    if config.files_with_matches && has_matches {
        writeln!(writer, "{}", file_path)?;
    } else if config.files_without_match && !has_matches {
        writeln!(writer, "{}", file_path)?;
    }

    Ok(has_matches)
}

/// Search for pattern in multiple files/directories
pub fn search_paths(
    config: &SearchConfig,
    paths: &[String],
    recursive: bool,
    no_messages: bool,
    writer: &mut impl Write,
) -> Result<()> {
    let mut all_files = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);

        if path.is_file() {
            all_files.push(path_str.clone());
        } else if path.is_dir() {
            if recursive {
                // Use walkdir for recursive directory traversal
                for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Some(path_str) = entry.path().to_str() {
                            all_files.push(path_str.to_string());
                        }
                    }
                }
            } else {
                // For files_with_matches/files_without_match, we should list the directory itself
                // but only if it doesn't exist as a file (which we're checking here)
                if config.files_with_matches || config.files_without_match {
                    // For these modes, we should still report the directory
                    all_files.push(path_str.clone());
                }
                if !no_messages {
                    eprintln!("{}: Is a directory", path_str);
                }
            }
        } else {
            if !no_messages {
                eprintln!("{}: No such file or directory", path_str);
            }
            // Return error for non-existent files
            return Err(PergError::FileNotFound(path_str.to_string()));
        }
    }

    // If only one file and filename display is not forced, don't show filenames
    let should_show_filename = config.with_filename || all_files.len() > 1;
    let mut effective_config = config.clone();
    effective_config.with_filename = should_show_filename;

    for file_path in all_files {
        if let Err(err) = search_file(&effective_config, &file_path, writer) {
            if !no_messages {
                eprintln!("perg: {}: {}", file_path, err);
            }
            // Return error for critical failures like file not found or regex errors
            match err {
                PergError::FileNotFound(_) | PergError::Regex(_) => return Err(err),
                _ => {} // Continue for other errors like I/O errors
            }
        }
    }

    Ok(())
}

/// Format a single match result for output
fn format_match(config: &SearchConfig, file_path: &str, line_number: usize, line: &str) -> String {
    let mut output = String::new();

    if config.with_filename {
        output.push_str(file_path);
        output.push_str(":");
    }

    if config.line_number {
        output.push_str(&line_number.to_string());
        output.push_str(":");
    }

    output.push_str(line);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_file_basic() {
        let content = "line 1\ntest line 2\nline 3";
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();

        let config = SearchConfig::new(
            "test".to_string(),
            false,
            false,
            false,
            false,
            false,
            false,
        );

        let mut output = Vec::new();
        let result = search_file(&config, file.path().to_str().unwrap(), &mut output);

        assert!(result.is_ok());
        assert!(result.unwrap()); // Should have matches
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("test line 2"));
    }

    #[test]
    fn test_search_file_case_insensitive() {
        let content = "Line 1\nTEST line 2\nline 3";
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();

        let config = SearchConfig::new(
            "test".to_string(),
            true, // ignore_case
            false,
            false,
            false,
            false,
            false,
        );

        let mut output = Vec::new();
        let result = search_file(&config, file.path().to_str().unwrap(), &mut output);

        assert!(result.is_ok());
        assert!(result.unwrap());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("TEST line 2"));
    }

    #[test]
    fn test_search_file_invert_match() {
        let content = "line 1\ntest line 2\nline 3";
        let mut file = tempfile::NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();

        let config = SearchConfig::new(
            "test".to_string(),
            false,
            false,
            false,
            true, // invert_match
            false,
            false,
        );

        let mut output = Vec::new();
        let result = search_file(&config, file.path().to_str().unwrap(), &mut output);

        assert!(result.is_ok());
        assert!(result.unwrap());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("line 1"));
        assert!(output_str.contains("line 3"));
        assert!(!output_str.contains("test line 2"));
    }
}
