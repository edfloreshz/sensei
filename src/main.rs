mod crate_info;
use clap::Parser;
use crate_info::{open, parse_args};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Tries to open local documentation
    #[clap(short, long)]
    local: bool,

    /// Looks up the version in Cargo.toml
    #[clap(short, long)]
    manifest: bool,

    /// What crate do you need help with, 学生?
    #[clap(value_name = "NAME")]
    name: String,

    /// Opens documentation for a specific version
    #[clap(short, long)]
    version: Option<String>,

    /// Specifies query to search documentation
    #[clap(short, long)]
    query: Option<String>,
}

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
    let args = Args::parse();

    let crate_info = parse_args(args);

    open(crate_info)
}
