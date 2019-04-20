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

a `test.md` is included in this repo, to try out `perg`:
```bash
❯ perg crate test.md
    # hello world
    # hi world
```

###### Regular Expression

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

## Help, feedback or suggestions?

Feel free to contact me on [Twitter](https://twitter.com/vinhnx) for discussions, news & announcements & other projects. :rocket:
