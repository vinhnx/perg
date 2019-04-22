struct Sut {}
impl Sut {
    fn read() -> String {
        format!("hello world\nhi world\nbye world\nend of file")
    }
}

#[test]
fn test_normal_search() {
    let mut result = Vec::new();
    let sut = Sut::read();
    let pattern = "h";
    perg::search(&sut, &pattern, &mut result);
    assert_eq!(result, b"hello world\nhi world\n");
}

#[test]
fn test_regular_expression_either() {
    let mut result = Vec::new();
    let sut = Sut::read();
    let pattern = "h[ei]";
    perg::search(&sut, &pattern, &mut result);
    assert_eq!(result, b"hello world\nhi world\n");
}

#[test]
fn test_regular_expression_either_start_of_line() {
    let mut result = Vec::new();
    let sut = Sut::read();
    let pattern = "^[be]";
    perg::search(&sut, &pattern, &mut result);
    assert_eq!(result, b"bye world\nend of file\n");
}
