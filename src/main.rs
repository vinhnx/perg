use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[derive(Debug)]
struct CLI {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl fmt::Display for CLI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLI: \n> args: {}\n> path: {:?}", 
            self.pattern, self.path
        )
    }
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn main() {
    let args = CLI::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    find_matches(&content, &args.pattern, &mut std::io::stdout());
}

#[test]
fn test_find_match() {
    let mut result = Vec::new();
    find_matches("hello world\nbye word", "hello", &mut result); // we give empty vector as "writer" in our test
    assert_eq!(result, b"hello world\n"); // b: is byte string literal
}
