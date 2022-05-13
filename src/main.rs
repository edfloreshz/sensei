mod crate_info;
use clap::{App, Arg};
use crate_info::{open, parse_args};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Receives input from the user to process their request.
///
/// # Arguments
///
/// * `crate` - A string with the crate.
///
/// # Options
///
/// ```
/// -q, --query <query>        Specifies query to search documentation.
/// -v, --version <version>    Opens documentation for a specific version.
/// ```
///
/// # Flags
///
/// ```
/// -h, --help      Prints help information
/// -l, --local     Tries to open local documentation.
/// -m, --manifest  Looks up the version in Cargo.toml
/// ```
///
/// # Examples
///
/// ```
/// $ sensei serde
/// $ sensei serde -v 0.8.8
/// $ sensei serde -q Serializer
/// $ sensei serde -v 0.8.8 - q Serializer
/// $ sensei serde -m
/// ```
fn main() -> Result<()> {
    let matches = App::new("Sensei")
        .version("0.2.9")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Opens the documentation for any crate.")
        .arg(
            Arg::with_name("crate")
                .help("What crate do you need help with, 学生?")
                .short("c")
                .long("crate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("local")
                .help("Tries to open local documentation.")
                .short("l")
                .long("local"),
        )
        .arg(
            Arg::with_name("version")
                .help("Opens documentation for a specific version.")
                .short("v")
                .long("version")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("query")
                .help("Specifies query to search documentation.")
                .short("q")
                .long("query")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("manifest")
                .help("Looks up the version in Cargo.toml")
                .short("m")
                .long("manifest"),
        )
        .get_matches();

    let crate_info = parse_args(&matches);

    open(crate_info)
}
