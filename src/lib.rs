extern crate regex;
use regex::Regex;

#[allow(unused_must_use)]
pub fn search(
    is_case_insensitive: bool,
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) {
    let pattern = if is_case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let regex = Regex::new(&pattern).unwrap();
    for line in content.lines() {
        if regex.is_match(line) {
            writeln!(writer, "{}", line);
        }
    }
}
