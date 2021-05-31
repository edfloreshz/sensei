mod crate_info;
use crate_info::{parse_args, CrateInfo};

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
/// -h, --help     Prints help information
/// -l, --local    Tries to open local documentation.
/// ```
///
/// # Examples
///
/// ```
/// $ sensei serde
/// $ sensei serde -v 0.8.8
/// $ sensei serde -q Serializer
/// $ sensei serde -v 0.8.8 - q Serializer
/// ```
fn main() {
    let crt = CrateInfo::new(parse_args());
    crt.open();
}
