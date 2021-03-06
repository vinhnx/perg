# perg

[![](https://img.shields.io/crates/v/perg.svg?colorB=225382&style=flat-square)](https://crates.io/crates/perg)


https://crates.io/crates/perg

A micro lightweight implementation of [`grep`](http://man7.org/linux/man-pages/man1/grep.1.html), written in Rust.

It's mainly for my journey to learn [Rust programming language and its fascinated ecosystem](https://www.rust-lang.org), but feel free to use it. :smile:

```bash
❯ perg --help
perg x.x.x
Vinh Nguyen <>
perg is a small command-line tool to search for given string inside a file

USAGE:
    perg [FLAGS] <PATTERN> <FILE>

FLAGS:
    -h, --help           Prints help information
    -i, --ignore-case    Perform case insensitive matching. Default is case sensitive.
    -V, --version        Prints version information

ARGS:
    <PATTERN>    pattern to search, can use regular expression
    <FILE>       path to file
```

### Usage


a `test.md` is included in this repo:

```bash
$ cat test.md
hello world hi world bye world end of file Title
```

to try out `perg` on the included `test.md` file:

```bash
$ perg h test.md
    # hello world
    # hi world
```

`perg` also support regular expression search, like `grep`:
```bash
$ perg "h[ei]" test.md
    # hello world
    # hi world

$ perg "hello|bye" test.md
    # hello world
    # bye world

$ perg "^(be)" test.md
    # bye world
    # end of file
```

case insensitive search:
```bash
$ perg -i I test.md
    # hi world
    # end of file
```

### Installation

Using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) via `rustup`:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

then install `perg` binary:

```bash
$ cargo install perg
```

NOTE: if you see error "cargo command not found", it is because `cargo` executable is not yet added to your .bashrc/.zshrc yet. To mitigate this, just restart your Terminal/iTerm and it run `cargo install perg` again, should it fine.

## Rust Books 🦀

+ https://doc.rust-lang.org/book/
+ https://doc.rust-lang.org/rust-by-example/

[As of now, Rust is one the most favorited programming according to StackOverflow](https://insights.stackoverflow.com/survey/2019). :gift:

## Help, feedback or suggestions?

Feel free to contact me on [Twitter](https://twitter.com/vinhnx) for discussions, news & announcements & other projects. :rocket:

## What's with the name? 

Glad you asked, `perg` is just the reversed of `grep`.
