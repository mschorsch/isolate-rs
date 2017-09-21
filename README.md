# isolate-rs
Experimental process isolation written in Rust (__Linux only__).

# Installation

1. Install [Rust](https://www.rust-lang.org/en-US/downloads.html)

2. Clone this repository
```git
git clone [REPO]
```

3. Build
```bash
cargo build --release
```

4. Run (default)
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