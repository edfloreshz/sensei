use crate::Result;
use clap::{App, Arg, ArgMatches};
use open;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

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
            if matches.is_present("manifest") {
                match get_manifest_version(crt.name.clone()) {
                    Ok(version) => crt.version = version,
                    Err(e) => eprintln!("{}", e),
                }
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
    pub fn open(&self) -> Result<()> {
        match open::that(&*self.url) {
            Ok(_) => {
                Ok(
                    if self.std {
                        println!(
                            "\x1B[32m\n||| The Standard Library {}||| \n{}\x1B[32m",
                            self.version, self.warning
                        )
                    } else {
                        println!(
                            "\x1B[32m\n||| The Book Of {} {}|||\n{}\x1B[32m",
                            first_letter_to_upper(self.name.as_str()),
                            self.version,
                            self.warning
                        )
                    }
                )
            }
            Err(e) => {
                if self.local && !self.is_locally_available() {
                    println!("The crate is not available locally");
                } else {
                    println!("Seems like you've lost your way, 学生, try again.");
                }
                Err(Box::new(e))
            }
        }
    }
}

/// Get manifest version from Cargo.toml
fn get_manifest_version(name: String) -> std::io::Result<String> {
    let toml = format!("{}/{}", std::env::current_dir()?.display(), "Cargo.toml");
    let version: String = read_to_string(toml)?
        .lines()
        .filter(|l| l.replace(" ", "").contains(format!("{}=", name).as_str()))
        .collect();
    Ok(version.trim_matches(|c: char| !c.is_numeric()).to_string())
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_upper(c: &str) -> String {
    if c.len() > 0 {
        format!("{}{}", &c[0..1].to_uppercase(), &c[1..])
    } else {
        c.to_string()
    }
}

#[test]
fn test_first_letter_to_upper() {
    assert_eq!(first_letter_to_upper("crate"), "Crate");
    assert_eq!(first_letter_to_upper("c"), "C");
    assert_eq!(first_letter_to_upper(""), "");
}

/// Creates an object of type ArgMatches with the structure of the CLI.
pub fn parse_args() -> ArgMatches<'static> {
    App::new("Sensei")
        .version("0.2.7")
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
        .get_matches()
}
