# perg

[![](https://img.shields.io/crates/v/perg.svg?colorB=225382&style=flat-square)](https://crates.io/crates/perg)

<https://crates.io/crates/perg>

A fast, feature-rich text search tool similar to [`grep`](http://man7.org/linux/man-pages/man1/grep.1.html), written in Rust.

perg is a modern implementation of the classic grep utility, designed for speed and ease of use. It supports regular expressions, recursive directory searching, and various output formatting options.

## Features

- **Fast text searching** with regular expression support
- **Recursive directory search** with the `-r` flag
- **Case-insensitive matching** with the `-i` flag
- **Line number display** with the `-n` flag
- **Filename display** with the `-H` flag
- **Invert match** (show non-matching lines) with the `-v` flag
- **Files with/without matches** listing with `-l`/`-L` flags
- **Multiple file/directory support**
- **Proper error handling** and exit codes

```bash
❯ perg --help
perg 0.5.1
Vinh Nguyen <vinhnguyen2308@gmail.com>
perg is a small command-line tool to search for given string inside a file

Usage: perg [OPTIONS] <PATTERN> [PATH]...

Arguments:
  <PATTERN>  Pattern to search for (supports regular expressions)
  [PATH]...  Files or directories to search in

Options:
  -i, --ignore-case          Perform case insensitive matching
  -n, --line-number          Show line numbers
  -H, --with-filename        Show filenames
  -r, --recursive            Recursively search directories
  -s, --no-messages          Suppress error messages about inaccessible files
  -v, --invert-match         Invert match: show lines that do NOT match the pattern
  -l, --files-with-matches   Only show filenames that contain matches
  -L, --files-without-match  Only show filenames that do NOT contain matches
  -h, --help                 Print help
  -V, --version              Print version information
```

## Usage

A `test.md` file is included in this repo for testing:

```bash
$ cat test.md
hello world hi world bye world end of file Title
```

### Basic Usage

Search for a pattern in a file:

```bash
$ perg hello test.md
hello world
```

### Advanced Features

**Line numbers:**

```bash
$ perg -n world test.md
1:hello world
2:hi world
3:bye world
```

**Case insensitive search:**

```bash
$ perg -i title test.md
Title
```

**Invert match (show lines that DON'T match):**

```bash
$ perg -v world test.md
end of file
Title
```

**Recursive directory search:**

```bash
perg -r "pattern" /path/to/directory
```

**Show filenames with matches:**

```bash
$ perg -l world test.md
test.md
```

**Show filenames without matches:**

```bash
$ perg -L world test.md
# (shows files that don't contain "world")
```

**Multiple files:**

```bash
$ perg -H pattern file1.txt file2.txt
file1.txt:matching line
file2.txt:another match
```

**Regular expressions:**

```bash
$ perg "h[ei]" test.md
hello world
hi world

$ perg "hello|bye" test.md
hello world
bye world

$ perg "^(be)" test.md
bye world
end of file
```

**Combined options:**

```bash
perg -r -i -n "error" /var/log/
```

## Installation

### From Crates.io

Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) via `rustup`:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then install `perg`:

```bash
cargo install perg
```

Note: If you see "cargo command not found", restart your terminal and run the install command again.

### From Source

Clone the repository and build:

```bash
git clone https://github.com/vinhnx/perg.git
cd perg
cargo build --release
./target/release/perg --help
```

## Exit Codes

- `0`: Success, matches found (or no matches when using `-L`)
- `1`: File not found or other I/O errors
- `2`: Invalid regular expression

## Project Structure

This project is organized as follows:

```
src/
├── main.rs      # CLI entry point
├── lib.rs       # Library exports
├── cli.rs       # Command-line argument parsing
├── search.rs    # Core search functionality
├── error.rs     # Error types and handling
└── ...
```

## Development

This project was created as a learning exercise for Rust programming. It demonstrates:

- Modern Rust development practices
- CLI application development with clap
- Error handling patterns
- Unit and integration testing
- Documentation with rustdoc

### Running Tests

```bash
cargo test                    # Run all tests
cargo test --test integration_test  # Run integration tests only
cargo test search::tests      # Run specific test module
```

### Benchmarks

The tool is optimized for performance and can handle large files and directories efficiently.

## Contributing

Contributions are welcome! Please feel free to:

- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## What's with the name?

Glad you asked, `perg` is just the reversed spelling of `grep`! 🦀

## Rust Resources 🦀

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Great for learning Rust

[Rust continues to be one of the most loved programming languages according to Stack Overflow surveys](https://insights.stackoverflow.com/survey/2023). :gift:
