use crate::Result;
use clap::ArgMatches;
use std::fs::read_to_string;
use std::path::Path;
use std::borrow::Cow;
use std::env;

enum CrateSource {
    Std,
    Local(String),
    DocsRs(String),
}

impl CrateSource {
    fn is_local(&self) -> bool {
        match self {
            &CrateSource::Local(_) => true,
            _ => false,
        }
    }
}

/// Structure with information about the crate.
pub struct CrateInfo<'a> {
    source: CrateSource,
    version: Option<Cow<'a, str>>,
    query: Option<&'a str>,
    warning: Option<String>,
}

pub fn parse_args<'a>(matches: &'a ArgMatches) -> CrateInfo<'a> {
    let name = matches.value_of("crate").unwrap().to_lowercase();
    let query = matches.value_of("query");

    let version = if matches.is_present("manifest") {
        match get_manifest_version(&name) {
            Ok(version) => Some(Cow::Owned(version)),
            Err(e) => {
                eprintln!("{}", e);

                None
            },
        }
    } else {
        matches.value_of("version").map(|v| Cow::Borrowed(v))
    };


    let (source, warning) = if name == "std" {
        (CrateSource::Std, None)
    } else if matches.is_present("local") {
        (CrateSource::Local(name), if version.is_some() || query.is_some() {
            Some("Versioning and querying is not available with local crates.".to_owned())
        } else { None })
    } else {
        (CrateSource::DocsRs(name), None)
    };

    CrateInfo {
        source, version, query, warning,
    }
}

/// Checks if the crate is available locally.
fn is_locally_available(path: &str) -> bool {
    Path::new(path).exists()
}

fn make_url(crinfo: &CrateInfo) -> String {
    match &crinfo.source {
        CrateSource::Std => {
            let base = String::from("https://docs.rs/std/index.html");

            if let Some(query) = crinfo.query {
                base + &format!("?search={}", query)
            } else {
                base
            }
        },
        CrateSource::Local(name) => {
            format!("{}/target/doc/{}/index.html", env::current_dir().unwrap().to_str().unwrap(), name)
        },
        CrateSource::DocsRs(name) => {
            let base = if let Some(version) = crinfo.version.as_ref() {
                format!("https://docs.rs/{}/{}", name, version)
            } else {
                format!("https://docs.rs/{}", name)
            };

            if let Some(query) = crinfo.query {
                base + &format!("?search={}", query)
            } else {
                base
            }
        }
    }
}

/// Opens the crate's documentation.
pub fn open(crinfo: CrateInfo) -> Result<()> {
    let url = make_url(&crinfo);

    match open::that(&url) {
        Ok(_) => {
            match crinfo.source {
                CrateSource::Std => {
                    println!("\x1B[32m\n||| The Standard Library ||| \n\x1B[32m");
                },
                CrateSource::Local(name) | CrateSource::DocsRs(name) => {
                    println!(
                        "\x1B[32m\n||| The Book Of {} {}|||\n{}\x1B[32m",
                        first_letter_to_upper(&name),
                        crinfo.version.unwrap_or_else(|| "".into()),
                        crinfo.warning.unwrap_or_else(|| "".into()),
                    );
                },
            }

            Ok(())
        }
        Err(e) => {
            if crinfo.source.is_local() && !is_locally_available(&url) {
                println!("The crate is not available locally");
            } else {
                println!("Seems like you've lost your way, 学生, try again.");
            }

            Err(Box::new(e))
        }
    }
}

/// Get manifest version from Cargo.toml
fn get_manifest_version(name: &str) -> std::io::Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_letter_to_upper() {
        assert_eq!(first_letter_to_upper("crate"), "Crate");
        assert_eq!(first_letter_to_upper("c"), "C");
        assert_eq!(first_letter_to_upper(""), "");
    }
}
