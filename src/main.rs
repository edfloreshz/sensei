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
///  -v, --version <version>    Opens documentation for a specific version.
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
    let matches = make_config();
    let mut crt = CrateInfo::new(matches.value_of("crate").unwrap().into());
    check_config(&mut crt, matches);
}

/// Creates an object of type ArgMatches with the structure of the CLI.
fn make_config() -> ArgMatches<'static> {
    App::new("Sensei")
        .version("0.1.12")
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
    crt.get_args(matches);
    crt.construct_url();
    crt.open();
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_uppercase(c: String) -> String {
    match c.chars().next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c[1..],
    }
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
    /// Assigns arguments to the structure.
    fn get_args(&mut self, matches: ArgMatches) {
        if matches.is_present("local") {
            if matches.is_present("version") || matches.is_present("query") {
                self.warning = "Versioning and querying is not available with local crates.".into();
            }
            if self.is_local() {
                self.open_locally()
            }
        }
        if matches.is_present("version") {
            self.version = matches.value_of("version").unwrap().parse().unwrap();
        }
        if matches.is_present("query") {
            self.query = matches.value_of("query").unwrap().parse().unwrap();
        }
    }
    /// Constructs the url.
    fn construct_url(&mut self) {
        if self.query.is_empty() && self.version.is_empty() {
            self.url = format!("https://docs.rs/{}", self.name);
        } else {
            self.url = if self.is_std() {
                self.url = "https://doc.rust-lang.org".into();
                self.format_url("", "std", "index.html")
            } else {
                self.url = "https://docs.rs".into();
                self.format_url(&*format!("{}/", &*self.name), "", "")
            }
        }
    }
    /// Formats the url.
    fn format_url(&self, crate_name: &str, stdlib: &str, index_file: &str) -> String {
        if !self.version.is_empty() && !self.query.is_empty() {
            format!(
                "{}/{}{}/{}/{}?search={}",
                self.url, crate_name, self.version, self.name, index_file, self.query
            )
        } else if !self.version.is_empty() {
            format!(
                "{}/{}{}/{}/{}",
                self.url, crate_name, self.version, stdlib, index_file
            )
        } else {
            format!(
                "{}/{}{}/{}?search={}",
                self.url, crate_name, stdlib, index_file, self.query
            )
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
