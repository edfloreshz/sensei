use clap::{App, Arg, ArgMatches};
use fastrand::usize;
use open;
use std::env;
use std::path::Path;

/// Array with phrases to print at the end of the program.
const PHRASES: [&str; 4] = ["幸運を", "よく学ぶ", "良い読書", "良書"];

/// Structure with information about the crate.
pub struct CrateInfo {
    name: String,
    version: String,
    query: String,
    url: String,
    warning: String,
    local: bool,
    std: bool,
}

/// Stores info about a crate.
impl CrateInfo {
    /// Creates an object of type CrateInfo.
    pub fn new(matches: ArgMatches) -> CrateInfo {
        let name = matches.value_of("crate").unwrap().to_lowercase().clone();
        let mut crt = CrateInfo {
            name: name.clone(),
            version: String::new(),
            query: String::new(),
            url: String::new(),
            warning: String::new(),
            local: matches.is_present("local"),
            std: "std".eq_ignore_ascii_case(&name),
        };
        if crt.local {
            crt.warning = "Versioning and querying is not available with local crates.".into();
            crt.url = format!(
                "{}/target/doc/{}/index.html",
                env::current_dir().unwrap().to_str().unwrap(),
                name
            );
        } else {
            if crt.std {
                crt.url = format!("https://doc.rust-lang.org")
            } else {
                crt.url = format!("https://docs.rs")
            }
            if matches.is_present("version") {
                crt.version = matches.value_of("version").unwrap().parse().unwrap();
            }
            if matches.is_present("query") {
                crt.query = matches.value_of("query").unwrap().parse().unwrap();
            }
            crt.url = match (crt.std, crt.query.is_empty(), crt.version.is_empty()) {
                (true, true, true) => format!("{}/std/index.html", crt.url),
                (true, true, false) => format!("{}/{}/std/index.html", crt.url, crt.version),
                (true, false, true) => {
                    format!("{}/std/index.html?search={}", crt.url, crt.query)
                }
                (true, false, false) => format!(
                    "{}/{}/std/index.html?search={}",
                    crt.url, crt.version, crt.query
                ),
                (false, true, true) => format!("{}/{}", crt.url, crt.name),
                (false, true, false) => format!("{}/{}/{}", crt.url, crt.name, crt.version),
                (false, false, true) => {
                    format!("{}/{}/?search={}", crt.url, crt.name, crt.query)
                }
                (false, false, false) => format!(
                    "{}/{}/{}/{}/?search={}",
                    crt.url, crt.name, crt.version, crt.name, crt.query
                ),
            }
        }
        crt
    }
    /// Checks if the crate is available locally.
    fn is_locally_available(&self) -> bool {
        Path::new(self.url.as_str()).exists()
    }
    /// Opens the crate's documentation.
    pub fn open(&self) {
        if open::that(&*self.url).is_ok() {
            if self.std {
                println!(
                    "\x1B[32m\n{} ||| The Standard Library {}||| {}\n{}\x1B[32m",
                    PHRASES[usize(0..PHRASES.len() - 1)],
                    format!("{}", self.version),
                    PHRASES[usize(0..PHRASES.len() - 1)],
                    self.warning
                )
            } else {
                println!(
                    "\x1B[32m\n{} ||| The Book Of {} {}||| {}\n{}\x1B[32m",
                    PHRASES[usize(0..PHRASES.len() - 1)],
                    first_letter_to_upper(self.name.clone()),
                    format!("{} ", self.version),
                    PHRASES[usize(0..PHRASES.len() - 1)],
                    self.warning
                )
            }
        } else {
            if self.local && !self.is_locally_available() {
                println!("The crate is not available locally");
            } else {
                println!("Seems like you've lost your way, 学生, try again.");
            }
        }
    }
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_upper(c: String) -> String {
    match c.chars().next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c[1..],
    }
}

/// Creates an object of type ArgMatches with the structure of the CLI.
pub fn parse_args() -> ArgMatches<'static> {
    App::new("Sensei")
        .version("0.2.4")
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
