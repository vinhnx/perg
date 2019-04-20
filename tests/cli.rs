extern crate regex;
use regex::Regex;

#[test]
fn test_search() {
    let mut result = Vec::new();
    let reg = Regex::new("h[ei]").unwrap();
    perg::search("hello world\nhi world\nbye world", &reg, &mut result);
    assert_eq!(result, b"hello world\nhi world\n");
}
