# perg

A micro lightweight implementation of [`grep`](http://man7.org/linux/man-pages/man1/grep.1.html), written in Rust.

```bash
❯ perg --help
perg x.x.x
Vinh Nguyen <vinhnguyen2308@gmail.com>
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

a `test.md` is included in this repo, to try out `perg`:
```bash
❯ perg crate test.md
    # hello world
    # hi world
```

`perg` also support regular expression search, like `grep`:
```bash
❯ perg "h[ei]" test.md
    # hello world
    # hi world

❯ perg "hello|bye" test.md
    # hello world
    # bye world

❯ perg "^(be)" test.md
    # bye world
    # end of file
```

case insensitive search:
```bash
❯ perg -i I test.md
    # hi world
    # end of file
```

## Help, feedback or suggestions?

Feel free to contact me on [Twitter](https://twitter.com/vinhnx) for discussions, news & announcements & other projects. :rocket:
