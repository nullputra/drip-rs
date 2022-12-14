# 💦 drip-rs

[![Crates.io](https://img.shields.io/crates/v/drip-rs)](https://crates.io/crates/drip-rs)
[![Crates.io](https://img.shields.io/crates/d/drip-rs)](https://crates.io/crates/drip-rs)
[![versions](https://img.shields.io/repology/repositories/drip-rs)](https://repology.org/project/drip-rs/versions)
[![Build status](https://github.com/nullputra/drip-rs/workflows/min-version/badge.svg)](https://github.com/nullputra/drip-rs/actions/workflows/min-version.yml)
[![Build status](https://github.com/nullputra/drip-rs/workflows/release/badge.svg)](https://github.com/nullputra/drip-rs/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-CC0_1.0-blue)](https://github.com/nullputra/drip-rs/blob/master/LICENSE)

## Synopsis

```bat
drip-rs exec FILE_PATH [ARGS...]
drip-rs misc COMMAND [ARGS...]
drip-rs {-h|--help|help}
drip-rs {-V|--version}
```

## Supported platforms

Windows and Linux

## Install

### Install with `cargo install`

```bat
$ cargo install drip-rs
```

### Build from source

```bat
$ git clone https://github.com/nullputra/drip-rs
$ cd drip-rs
$ cargo build --release
```

### Show version information

```bat
$ cargo install drip-rs
$ drip-rs -V
drip-rs 1.0.0
```

## Examples

### `drip-rs -h`

```bat
$ drip-rs -h
drip-rs 1.0.0
Nullputra <stdnlptr@gmail.com>
`drip-rs` executes code and runs tasks

USAGE:
    drip-rs.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    exec    Execute code according to extension
    help    Print this message or the help of the given subcommand(s)
    misc    Run task(s)
```

### `drip-rs exec FILE_PATH [ARGS...]`

```bat
$ cat drip.toml
symbol = ">"

[exec]
cpp = [
    "g++ -g -O2 -std=c++17 -W $FILE_PATH -o $FILE_PATH_WITHOUT_EXT",
    "$FILE_PATH_WITHOUT_EXT",
]
py = { cmd = "py -3 $FILE_PATH", sh = "python3 $FILE_PATH" }

$ drip-rs exec tests/data/fft.py
[INFO] cmd: "python3 tests/data/fft.py"
[INFO] args: []
> python3 tests/data/fft.py
1 4 11 26 36 40 32

$ drip-rs exec tests/data/z-algorithm.cpp
[INFO] cmd: ["g++ -g -O2 -std=c++17 -W tests/data/z-algorithm.cpp -o tests/data/z-algorithm", "tests/data/z-algorithm"]
[INFO] args: []
> g++ -g -O2 -std=c++17 -W tests/data/z-algorithm.cpp -o tests/data/z-algorithm
> tests/data/z-algorithm
z-algorithm works
```

### `drip-rs misc COMMAND [ARGS...]`

```bat
$ cat drip.toml
symbol = ">"

[env_var]
COMMIT_MSG = "Fix src/main.rs"

[misc]
echo = "echo $0"
# When the contents of "cmd" and "sh" match,
# they can be specified together by "common".
echo_confirm = { common = "echo $0", confirm = true }
# This is equivalent to the following:
# echo_confirm = { cmd = "echo $0", sh = "echo $0", confirm = true }
echo2 = ["echo $0", "echo ${1}_$2"]
push = { common = [
    "git add --all",
    'git commit -m "$COMMIT_MSG"',
    "git push origin master",
], confirm = true }

$ drip-rs misc echo arg0
scrs: "echo arg0"
args: ["arg0"]
> echo arg0
arg0

$ drip-rs misc echo_confirm arg0
[INFO] cmd: "echo arg0"
[INFO] args: ["arg0"]
Continue? y
> echo arg0
arg0

$ drip-rs misc echo2 arg0 arg1 arg2
[INFO] cmd: ["echo arg0", "echo arg1_arg2"]
[INFO] args: ["arg0", "arg1", "arg2"]
> echo arg0
arg0
> echo arg1_arg2
arg1_arg2

$ drip-rs misc push
[INFO] cmd: ["git add --all", "git commit -m "Fix src/main.rs"", "git push origin master"]
[INFO] args: []
Continue? n
Error: Aborted
```

## Debug

```bat
$ git clone https://github.com/nullputra/drip-rs
$ cd drip-rs
$ cargo run -- exec tests/data/fft.py
[INFO] cmd: "python3 tests/data/fft.py"
[INFO] args: []
> python3 tests/data/fft.py
1 4 11 26 36 40 32
```

## Future outlook

- [x] I'd like to support linux as well.

## References

- [Command Line Apps in Rust](https://rust-cli.github.io/book)
- [clap::\_derive::\_tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [TOML](https://toml.io/ja/)
- [Rust/Anyhow の Tips](https://zenn.dev/yukinarit/articles/b39cd42820f29e)
- [`sharkdp/fd`](https://github.com/sharkdp/fd)
- [`Peltoche/lsd`](https://github.com/Peltoche/lsd)
- [`BurntSushi/ripgrep`](https://github.com/BurntSushi/ripgrep)

## License

Licensed under [Creative Commons CC0 1.0 Universal Public Domain Dedication](https://creativecommons.org/publicdomain/zero/1.0/) (no credit needed).
