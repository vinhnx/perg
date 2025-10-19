use perg::search::{search_file, SearchConfig};
use std::io::Write;

struct Sut {}
impl Sut {
    fn create_temp_file(content: &str) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }
}

#[test]
fn test_normal_search() {
    let content = "Title\nhello world\nhi world\nbye world\nend of file";
    let file = Sut::create_temp_file(content);
    let file_path = file.path().to_str().unwrap();

    let config = SearchConfig::new(
        "h".to_string(),
        false, // ignore_case
        false, // line_number
        false, // with_filename
        false, // invert_match
        false, // files_with_matches
        false, // files_without_match
        false, // count
        0,     // before_context
        0,     // after_context
        0,     // context
        None,  // max_count
        false, // only_matching
        false, // extended_regexp
        "auto".to_string(), // color
    );

    let mut result = Vec::new();
    let search_result = search_file(&config, file_path, &mut result);

    assert!(search_result.is_ok());
    assert!(search_result.unwrap()); // Should have matches
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("hello world"));
    assert!(output.contains("hi world"));
}

#[test]
fn test_case_insensitive() {
    let content = "Title\nhello world\nhi world\nbye world\nend of file";
    let file = Sut::create_temp_file(content);
    let file_path = file.path().to_str().unwrap();

    let config = SearchConfig::new(
        "t".to_string(),
        true, // ignore_case
        false, // line_number
        false, // with_filename
        false, // invert_match
        false, // files_with_matches
        false, // files_without_match
        false, // count
        0,     // before_context
        0,     // after_context
        0,     // context
        None,  // max_count
        false, // only_matching
        false, // extended_regexp
        "auto".to_string(), // color
    );

    let mut result = Vec::new();
    let search_result = search_file(&config, file_path, &mut result);

    assert!(search_result.is_ok());
    assert!(search_result.unwrap());
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Title"));
}

#[test]
fn test_regular_expression_either() {
    let content = "Title\nhello world\nhi world\nbye world\nend of file";
    let file = Sut::create_temp_file(content);
    let file_path = file.path().to_str().unwrap();

    let config = SearchConfig::new(
        "h[ei]".to_string(),
        false, // ignore_case
        false, // line_number
        false, // with_filename
        false, // invert_match
        false, // files_with_matches
        false, // files_without_match
        false, // count
        0,     // before_context
        0,     // after_context
        0,     // context
        None,  // max_count
        false, // only_matching
        false, // extended_regexp
        "auto".to_string(), // color
    );

    let mut result = Vec::new();
    let search_result = search_file(&config, file_path, &mut result);

    assert!(search_result.is_ok());
    assert!(search_result.unwrap());
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("hello world"));
    assert!(output.contains("hi world"));
}

#[test]
fn test_regular_expression_either_start_of_line() {
    let content = "Title\nhello world\nhi world\nbye world\nend of file";
    let file = Sut::create_temp_file(content);
    let file_path = file.path().to_str().unwrap();

    let config = SearchConfig::new(
        "^[be]".to_string(),
        false, // ignore_case
        false, // line_number
        false, // with_filename
        false, // invert_match
        false, // files_with_matches
        false, // files_without_match
        false, // count
        0,     // before_context
        0,     // after_context
        0,     // context
        None,  // max_count
        false, // only_matching
        false, // extended_regexp
        "auto".to_string(), // color
    );

    let mut result = Vec::new();
    let search_result = search_file(&config, file_path, &mut result);

    assert!(search_result.is_ok());
    assert!(search_result.unwrap());
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("bye world"));
    assert!(output.contains("end of file"));
}

#[test]
fn test_line_numbers() {
    let content = "Title\nhello world\nhi world\nbye world\nend of file";
    let file = Sut::create_temp_file(content);
    let file_path = file.path().to_str().unwrap();

    let config = SearchConfig::new(
        "world".to_string(),
        false, // ignore_case
        true,  // line_number
        false, // with_filename
        false, // invert_match
        false, // files_with_matches
        false, // files_without_match
        false, // count
        0,     // before_context
        0,     // after_context
        0,     // context
        None,  // max_count
        false, // only_matching
        false, // extended_regexp
        "auto".to_string(), // color
    );

    let mut result = Vec::new();
    let search_result = search_file(&config, file_path, &mut result);

    assert!(search_result.is_ok());
    assert!(search_result.unwrap());
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("2:hello world"));
    assert!(output.contains("3:hi world"));
    assert!(output.contains("4:bye world"));
}

#[test]
fn test_invert_match() {
    let content = "Title\nhello world\nhi world\nbye world\nend of file";
    let file = Sut::create_temp_file(content);
    let file_path = file.path().to_str().unwrap();

    let config = SearchConfig::new(
        "world".to_string(),
        false, // ignore_case
        false, // line_number
        false, // with_filename
        true,  // invert_match
        false, // files_with_matches
        false, // files_without_match
        false, // count
        0,     // before_context
        0,     // after_context
        0,     // context
        None,  // max_count
        false, // only_matching
        false, // extended_regexp
        "auto".to_string(), // color
    );

    let mut result = Vec::new();
    let search_result = search_file(&config, file_path, &mut result);

    assert!(search_result.is_ok());
    assert!(search_result.unwrap());
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Title"));
    assert!(output.contains("end of file"));
    assert!(!output.contains("world"));
}
