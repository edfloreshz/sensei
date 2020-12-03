use clap::{App, Arg, ArgMatches};
use rand::Rng;
use std::env;
use std::path::Path;
use std::process::exit;
use webbrowser;

/// Array with phrases to print at the end of the program.
const PHRASES: [&str; 3] = ["幸運を", "よく学ぶ", "良い読書"];

/// Structure with information about the crate.
struct CrateInfo {
    name: String,
    version: String,
    query: String,
    url: String,
    warning: String,
}

impl CrateInfo {
    /// Creates an object of type CrateInfo.
    fn new(name: String) -> CrateInfo {
        CrateInfo {
            name,
            version: String::new(),
            query: String::new(),
            url: String::new(),
            warning: String::new(),
        }
    }
    /// Checks if the crate is The Standard Library.
    fn is_std(&self) -> bool {
        self.name == "std"
    }
    /// Checks if the crate is available locally.
    fn is_local(&self) -> bool {
        Path::exists(
            format!(
                "{}/target/doc/{}/index.html",
                env::current_dir().unwrap().to_str().unwrap(),
                self.name
            )
            .as_ref(),
        )
    }
    /// Opens the crate's documentation.
    fn open(&self) {
        if !(webbrowser::open(&*self.url).is_ok()) {
            println!("Seems like you've lost your way, 学生, try again.");
        } else {
            let mut rng = rand::thread_rng();
            if self.is_std() {
                println!(
                    "||| The Standard Library {}|||\n{}\n{}",
                    format!("{}", self.version),
                    self.warning,
                    PHRASES[rng.gen_range(0, 2)]
                )
            } else {
                println!(
                    "||| The Book Of {} {}|||\n{}\n{}",
                    first_letter_to_uppercase(self.name.clone()),
                    format!("{} ", self.version),
                    self.warning,
                    PHRASES[rng.gen_range(0, 2)]
                )
            }
        }
    }
    /// Opens the crate's documentation locally.
    fn open_locally(&self) {
        if webbrowser::open(&*format!(
            "{}/target/doc/{}/index.html",
            env::current_dir().unwrap().to_str().unwrap(),
            self.name
        ))
        .is_ok()
        {
            let mut rng = rand::thread_rng();
            println!(
                "||| The Book Of {} {}|||\n{}\n{}",
                first_letter_to_uppercase(self.name.clone()),
                format!("{} ", self.version),
                self.warning,
                PHRASES[rng.gen_range(0, 2)]
            );
            exit(0)
        }
    }
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
//  -v, --version <version>    Opens documentation for a specific version.
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
    let matches = get_config();
    let name: String = matches.value_of("crate").unwrap().into();
    let mut crt = CrateInfo::new(name);
    check_config(&mut crt, matches);
}

/// Creates an object of type ArgMatches with the structure of the CLI.
fn get_config() -> ArgMatches<'static> {
    App::new("Sensei")
        .version("0.1.11")
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
        .get_matches()
}

/// Checks arguments and executes the required actions.
fn check_config(crt: &mut CrateInfo, matches: ArgMatches) {
    if matches.is_present("local") {
        if matches.is_present("version") || matches.is_present("query") {
            crt.warning = "Versioning and querying is not available with local crates.".into();
        }
        if crt.is_local() {
            crt.open_locally()
        }
    }
    if matches.is_present("version") {
        crt.version = matches.value_of("version").unwrap().parse().unwrap();
    }
    if matches.is_present("query") {
        crt.query = matches.value_of("query").unwrap().parse().unwrap();
    }
    if crt.query.is_empty() && crt.version.is_empty() {
        crt.url = format!("https://docs.rs/{}", crt.name);
    } else {
        if !crt.version.is_empty() && !crt.query.is_empty() {
            crt.url = if crt.is_std() {
                format!(
                    "https://doc.rust-lang.org/{}/std/index.html?search={}",
                    crt.version, crt.query
                )
            } else {
                format!(
                    "https://docs.rs/{}/{}/{}/?search={}",
                    crt.name, crt.version, crt.name, crt.query
                )
            };
        } else if !crt.version.is_empty() {
            crt.url = if crt.is_std() {
                format!("https://doc.rust-lang.org/{}/std/index.html", crt.version)
            } else {
                format!("https://docs.rs/{}/{}/{}", crt.name, crt.version, crt.name)
            };
        } else {
            crt.url = if crt.is_std() {
                format!(
                    "https://doc.rust-lang.org/std/index.html?search={}",
                    crt.query
                )
            } else {
                format!("https://docs.rs/{}/?search={}", crt.name, crt.query)
            };
        }
    }
    crt.open();
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_uppercase(c: String) -> String {
    match c.chars().next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c[1..],
    }
}
