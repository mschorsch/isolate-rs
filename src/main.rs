extern crate nix;
extern crate toml;
extern crate clap;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

mod errors;
mod mount;

use clap::{Arg, App, ArgMatches};

use errors::*;
use errors::Result; // ide hint

use std::fs;
use std::io::Read;

use std::ffi::CString;

#[derive(Deserialize, Debug)]
pub struct Config {
    initial_dir: String,
    command: Vec<CString>,
    readonly_dirs: Vec<String>,
    tmpfs_dirs: Vec<String>
}

// TODO clone stack: heap or stack?
// TODO chroot?
// TODO setUsername, setuid, setgid?

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }
    }
}

fn run() -> Result<()> {
    // 1. read configs
    let args = read_cmd_args();
    let config_filename = args.value_of("CONFIG_FILE")
        .ok_or_else(|| "Config file not found")?;
    let config = from_filename(config_filename)?;

    // 2. start clone
    mount::do_clone(&config)
}

fn read_cmd_args<'a>() -> ArgMatches<'a> {
    App::new("isolate process")
        .version("0.1")
        .author("Matthias Schorsch")
        .about("Experimental container isolation")
        .arg(Arg::with_name("CONFIG_FILE")
            .help("Sets the config file to use")
            .default_value("config.toml")
            .required(true)
            .index(1))
        .get_matches()
}

fn from_filename(filename: &str) -> Result<Config> {
    let mut file: fs::File = fs::File::open(filename)
        .chain_err(|| format!("Could not open {}", filename))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .chain_err(|| format!("Could not read {}", filename))?;

    toml::from_str(&buf)
        .chain_err(|| format!("Could not deserialize {}", filename))
}
