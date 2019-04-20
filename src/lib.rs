extern crate regex;
use regex::Regex;

// TODO: unify these with borrowing `to_lowercase()`

#[allow(unused_must_use)]
pub fn search(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let regex = Regex::new(&pattern).unwrap();
    for line in content.lines() {
        if regex.is_match(line) {
            writeln!(writer, "{}", line);
        }
    }
}

#[allow(unused_must_use)]
pub fn search_case_insensitive(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let regex = Regex::new(&pattern.to_lowercase()).unwrap();
    for line in content.lines() {
        if regex.is_match(line) {
            writeln!(writer, "{}", line);
        }
    }
}
