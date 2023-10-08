use crate::{Args, Result};

use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;

enum CrateSource {
    Std,
    Local(String),
    DocsRs(String),
}

impl CrateSource {
    fn is_local(&self) -> bool {
        matches!(self, &CrateSource::Local(_))
    }
}

/// Structure with information about the crate.
pub struct CrateInfo {
    source: CrateSource,
    version: Option<String>,
    query: Option<String>,
    warning: Option<String>,
}

pub fn parse_args(matches: Args) -> CrateInfo {
    let name = matches.name.to_lowercase();
    let query = matches.query;

    let version = if matches.manifest {
        match get_manifest_version(&name) {
            Ok(version) => Some(version),
            Err(e) => {
                eprintln!("{}", e);

                None
            }
        }
    } else {
        matches.version
    };

    let (source, warning) = if name == "std" {
        (CrateSource::Std, None)
    } else if matches.local {
        (
            CrateSource::Local(name),
            if version.is_some() || query.is_some() {
                Some("Versioning and querying is not available with local crates.".to_owned())
            } else {
                None
            },
        )
    } else {
        (CrateSource::DocsRs(name), None)
    };

    CrateInfo {
        source,
        version,
        query,
        warning,
    }
}

/// Checks if the crate is available locally.
fn is_locally_available(path: &str) -> bool {
    Path::new(path).exists()
}

fn make_url(crate_info: &CrateInfo) -> String {
    match &crate_info.source {
        CrateSource::Std => {
            let base = if let Some(version) = &crate_info.version {
                format!("https://doc.rust-lang.org/{}/std/", version)
            } else {
                "https://doc.rust-lang.org/stable/std/".to_string()
            };

            if let Some(ref query) = crate_info.query {
                base + &format!("?search={}", query)
            } else {
                base
            }
        }
        CrateSource::Local(name) => {
            format!(
                "{}/target/doc/{}/index.html",
                env::current_dir().unwrap().to_str().unwrap(),
                name
            )
        }
        CrateSource::DocsRs(name) => {
            let base = if let Some(version) = crate_info.version.as_ref() {
                format!("https://docs.rs/{}/{}", name, version)
            } else {
                format!("https://docs.rs/{}", name)
            };

            if let Some(ref query) = crate_info.query {
                base + &format!("?search={}", query)
            } else {
                base
            }
        }
    }
}

/// Opens the crate's documentation.
pub fn open(crate_info: CrateInfo) -> Result<()> {
    let url = make_url(&crate_info);

    match open::that(&url) {
        Ok(_) => {
            match crate_info.source {
                CrateSource::Std => {
                    if let Some(version) = &crate_info.version {
                        println!(
                            "\n\x1B[32m||| The Standard Library {} ||| \x1b[0m\n",
                            version
                        )
                    } else {
                        println!("\n\x1B[32m||| The Standard Library ||| \x1b[0m\n")
                    }
                }
                CrateSource::Local(name) | CrateSource::DocsRs(name) => {
                    println!(
                        "\n\x1B[32m||| The Book Of {} {}|||{}\x1b[0m\n",
                        first_letter_to_upper(&name),
                        crate_info.version.unwrap_or_else(|| "".into()),
                        crate_info.warning.unwrap_or_else(|| "".into()),
                    );
                }
            }

            Ok(())
        }
        Err(e) => {
            if crate_info.source.is_local() && !is_locally_available(&url) {
                println!("Trying to build the documentation locally...");
                let build_local_doc = Command::new("cargo").arg("doc").status();
                match build_local_doc {
                    Ok(status) => {
                        if status.success() && is_locally_available(&url) {
                            return open(crate_info);
                        } else {
                            println!("The crate is not available locally")
                        }
                    }
                    Err(build_error) => println!("Oh no {}!", build_error),
                }
            } else {
                println!("Seems like you've lost your way, 学生, try again.");
            }

            Err(Box::new(e))
        }
    }
}

/// Get manifest version from Cargo.toml
fn get_manifest_version(name: &str) -> std::io::Result<String> {
    let toml = std::env::current_dir()?.join("Cargo.toml");
    let version: String = read_to_string(toml)?
        .lines()
        .filter(|l| l.replace(' ', "").contains(format!("{}=", name).as_str()))
        .collect();
    Ok(version.trim_matches(|c: char| !c.is_numeric()).to_string())
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_upper(c: &str) -> String {
    if !c.is_empty() {
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
