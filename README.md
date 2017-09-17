# isolate-rs
Experimental process isolation written in Rust (__Linux only__).

# Installation

Install [Rust](https://www.rust-lang.org/en-US/downloads.html)
```bash
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Clone this repository
```git
git clone [REPO]
```

Build
```bash
cargo build --release
```

Run
```bash
./target/release/isolate-rs
```

# Command line options
```
USAGE:
    isolate-rs <CONFIG_FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <CONFIG_FILE>    Sets the config file to use [default: config.toml]
```

# License
MIT
