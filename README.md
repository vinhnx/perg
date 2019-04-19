# perg

A micro lightweight alternative to [`grep`](http://man7.org/linux/man-pages/man1/grep.1.html), without highlighting, written in Rust. Currently it only support local path.

```bash
❯ perg --help

perg 0.1.0

USAGE:
    perg <pattern> <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <pattern>
    <path>
```

### Installation

Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) via `rustup`:

```bash
❯ curl https://sh.rustup.rs -sSf | sh
```

then install `perg` binary:

```bash
❯ cargo install perg
```

### Usage

###### Quick example

on current repo:
```bash
❯ perg v src/main.rs
    #[derive(StructOpt)]
    #[derive(Debug)]
```

###### Another Example

on a test file, then search for a pattern:
```bash
❯ echo "hello world\nhi world\nby world" >> test.txt
❯ cat test.txt
    # hello world
    # hi world
    # by world
❯ perg e test.txt
    # hello world
    # bye world
```

## Help, feedback or suggestions?

Feel free to contact me on [Twitter](https://twitter.com/vinhnx) for discussions, news & announcements & other projects. :rocket:
