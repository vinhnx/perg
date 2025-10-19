use crate::error::{PergError, Result};
use console::style;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, stdin};
use std::path::Path;
use walkdir::WalkDir;

/// Helper function to determine if we should use colors
fn use_colors(color_option: &str) -> bool {
    match color_option {
        "always" => true,
        "never" => false,
        "auto" => console::colors_enabled(),
        _ => console::colors_enabled(),
    }
}

/// Helper function to colorize matches in a line
fn colorize_matches(line: &str, regex: &Regex, color_option: &str) -> String {
    if !use_colors(color_option) {
        return line.to_string();
    }
    
    // Use the regex to find all matches and replace them with colored versions
    regex.replace_all(line, |caps: &regex::Captures| {
        style(&caps[0]).red().bold().to_string()
    }).to_string()
}

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
    pub count: bool,
    pub before_context: usize,
    pub after_context: usize,
    pub context: usize,
    pub max_count: Option<usize>,
    pub only_matching: bool,
    pub extended_regexp: bool,
    pub color: String,
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
        count: bool,
        before_context: usize,
        after_context: usize,
        context: usize,
        max_count: Option<usize>,
        only_matching: bool,
        extended_regexp: bool,
        color: String,
    ) -> Self {
        Self {
            pattern,
            ignore_case,
            line_number,
            with_filename,
            invert_match,
            files_with_matches,
            files_without_match,
            count,
            before_context,
            after_context,
            context,
            max_count,
            only_matching,
            extended_regexp,
            color,
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
    let mut match_count = 0;
    let mut _line_number = 0;
    let mut lines: Vec<String> = Vec::new();
    let mut matching_line_indices = Vec::new();
    
    // Read all lines to enable context functionality
    for line in reader.lines() {
        lines.push(line?);
        _line_number += 1;
    }

    // Use context: if -C is specified, it overrides -A and -B
    let before_context = if config.context > 0 { config.context } else { config.before_context };
    let after_context = if config.context > 0 { config.context } else { config.after_context };

    // First pass: find all matching lines
    for (idx, line) in lines.iter().enumerate() {
        let matches = regex.is_match(line);

        // Apply invert match logic
        let should_include = if config.invert_match { !matches } else { matches };

        if should_include {
            has_matches = true;
            match_count += 1;
            matching_line_indices.push(idx);

            // For files_with_matches/files_without_match modes, we just need to know if there are matches
            if config.files_with_matches || config.files_without_match {
                continue;
            }
        }
    }

    // Handle count-only mode
    if config.count {
        writeln!(writer, "{}:{}", file_path, match_count)?;
        return Ok(match_count > 0);
    }

    // Handle files_with_matches/files_without_match output
    if config.files_with_matches && has_matches {
        writeln!(writer, "{}", file_path)?;
        return Ok(has_matches);
    } else if config.files_without_match && !has_matches {
        writeln!(writer, "{}", file_path)?;
        return Ok(has_matches);
    }

    // Output results with context
    let mut output_lines = std::collections::BTreeSet::new(); // Use BTreeSet to keep lines in order
    for &match_idx in &matching_line_indices {
        let start_idx = if match_idx >= before_context { match_idx - before_context } else { 0 };
        let end_idx = std::cmp::min(match_idx + after_context, lines.len() - 1);

        // Add this matching line and its context
        for idx in start_idx..=end_idx {
            output_lines.insert((idx, match_idx == idx)); // (line_idx, is_match)
        }
    }

    let mut output_count = 0;
    for (line_idx, is_match) in output_lines {
        // Check max count limit
        if is_match {
            if let Some(max) = config.max_count {
                if output_count >= max {
                    break;
                }
                output_count += 1;
            }
        }

        if is_match {
            // This is a matching line
            if config.only_matching {
                // Extract only the matching parts
                for mat in regex.find_iter(&lines[line_idx]) {
                    writeln!(writer, "{}", mat.as_str())?;
                }
            } else {
                // Output the full line with proper formatting
                let original_line = &lines[line_idx];
                let line_to_output = if use_colors(&config.color) && !config.only_matching {
                    colorize_matches(original_line, &regex, &config.color)
                } else {
                    original_line.clone()
                };
                
                let output = format_match_with_content(config, file_path, line_idx + 1, &line_to_output);
                writeln!(writer, "{}", output)?;
            }
        } else {
            // This is just context, output with dashes to separate
            let output = format_context_line(config, file_path, line_idx + 1, &lines[line_idx]);
            writeln!(writer, "{}", output)?;
        }
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

    for (i, file_path) in all_files.iter().enumerate() {
        if let Err(err) = search_file(&effective_config, file_path, writer) {
            if !no_messages {
                eprintln!("perg: {}: {}", file_path, err);
            }
            // Return error for critical failures like file not found or regex errors
            match err {
                PergError::FileNotFound(_) | PergError::Regex(_) => return Err(err),
                _ => {} // Continue for other errors like I/O errors
            }
        }
        
        // Add separator between files if context is enabled and there are multiple files
        if i < all_files.len() - 1 && (config.before_context > 0 || config.after_context > 0 || config.context > 0) {
            writeln!(writer, "--")?;
        }
    }

    Ok(())
}

/// Search stdin for the pattern
pub fn search_stdin(config: &SearchConfig, writer: &mut impl Write) -> Result<()> {
    let stdin = stdin();
    let reader = stdin.lock();

    let pattern = if config.ignore_case {
        format!("(?i){}", config.pattern)
    } else {
        config.pattern.clone()
    };

    let regex = Regex::new(&pattern)?;

    let mut match_count = 0;
    let mut _line_number = 0;
    let mut lines: Vec<String> = Vec::new();
    let mut matching_line_indices = Vec::new();
    
    // Read all lines to enable context functionality
    for line_result in reader.lines() {
        lines.push(line_result?);
        _line_number += 1;
    }

    // Use context: if -C is specified, it overrides -A and -B
    let before_context = if config.context > 0 { config.context } else { config.before_context };
    let after_context = if config.context > 0 { config.context } else { config.after_context };

    // First pass: find all matching lines
    for (idx, line) in lines.iter().enumerate() {
        let matches = regex.is_match(line);

        // Apply invert match logic
        let should_include = if config.invert_match { !matches } else { matches };

        if should_include {
            match_count += 1;
            matching_line_indices.push(idx);

            // For files_with_matches/files_without_match modes, we can't use stdin
            if config.files_with_matches || config.files_without_match {
                // For stdin, these modes don't make sense, so we just continue
                continue;
            }
        }
    }

    // Handle count-only mode
    if config.count {
        writeln!(writer, "{}", match_count)?;
        return Ok(());
    }

    // Handle files_with_matches/files_without_match modes (they don't make sense with stdin)
    if config.files_with_matches || config.files_without_match {
        // These modes don't apply to stdin
        return Ok(());
    }

    // Output results with context
    let mut output_lines = std::collections::BTreeSet::new(); // Use BTreeSet to keep lines in order
    for &match_idx in &matching_line_indices {
        let start_idx = if match_idx >= before_context { match_idx - before_context } else { 0 };
        let end_idx = std::cmp::min(match_idx + after_context, lines.len() - 1);

        // Add this matching line and its context
        for idx in start_idx..=end_idx {
            output_lines.insert((idx, match_idx == idx)); // (line_idx, is_match)
        }
    }

    let mut output_count = 0;
    for (line_idx, is_match) in output_lines {
        // Check max count limit
        if is_match {
            if let Some(max) = config.max_count {
                if output_count >= max {
                    break;
                }
                output_count += 1;
            }
        }

        if is_match {
            // This is a matching line
            if config.only_matching {
                // Extract only the matching parts
                for mat in regex.find_iter(&lines[line_idx]) {
                    writeln!(writer, "{}", mat.as_str())?;
                }
            } else {
                // Output the full line with proper formatting
                let original_line = &lines[line_idx];
                let line_to_output = if use_colors(&config.color) && !config.only_matching {
                    colorize_matches(original_line, &regex, &config.color)
                } else {
                    original_line.clone()
                };
                
                let output = format_line_with_content(config, line_idx + 1, &line_to_output);
                writeln!(writer, "{}", output)?;
            }
        } else {
            // This is just context, output with dashes to separate
            let output = format_context_line_stdin(config, line_idx + 1, &lines[line_idx]);
            writeln!(writer, "{}", output)?;
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

/// Format a single match result for output with custom content
fn format_match_with_content(config: &SearchConfig, file_path: &str, line_number: usize, line: &str) -> String {
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

/// Format a line from stdin (without filename prefix)
fn format_line(config: &SearchConfig, line_number: usize, line: &str) -> String {
    let mut output = String::new();

    if config.line_number {
        output.push_str(&line_number.to_string());
        output.push_str(":");
    }

    output.push_str(line);
    output
}

/// Format a line from stdin with custom content
fn format_line_with_content(config: &SearchConfig, line_number: usize, line: &str) -> String {
    let mut output = String::new();

    if config.line_number {
        output.push_str(&line_number.to_string());
        output.push_str(":");
    }

    output.push_str(line);
    output
}

/// Format a context line for file output
fn format_context_line(config: &SearchConfig, file_path: &str, line_number: usize, line: &str) -> String {
    let mut output = String::new();

    if config.with_filename {
        output.push_str(file_path);
        output.push_str("-");
    }

    if config.line_number {
        output.push_str(&line_number.to_string());
        output.push_str("-");
    }

    output.push_str(line);
    output
}

/// Format a context line from stdin (without filename prefix)
fn format_context_line_stdin(config: &SearchConfig, line_number: usize, line: &str) -> String {
    let mut output = String::new();

    if config.line_number {
        output.push_str(&line_number.to_string());
        output.push_str("-");
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
            false,        // count
            0,            // before_context
            0,            // after_context
            0,            // context
            None,         // max_count
            false,        // only_matching
            false,        // extended_regexp
            "auto".to_string(), // color
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
            false,        // count
            0,            // before_context
            0,            // after_context
            0,            // context
            None,         // max_count
            false,        // only_matching
            false,        // extended_regexp
            "auto".to_string(), // color
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
            false,        // count
            0,            // before_context
            0,            // after_context
            0,            // context
            None,         // max_count
            false,        // only_matching
            false,        // extended_regexp
            "auto".to_string(), // color
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
