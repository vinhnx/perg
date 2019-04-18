#[test]
fn test_find_matches() {
    let mut result = Vec::new();
    perg::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
