extern crate regex;

#[allow(unused_must_use)]
pub fn search(content: &str, pattern: &regex::Regex, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if pattern.is_match(line) {
            writeln!(writer, "{}", line);
        }
    }
}
