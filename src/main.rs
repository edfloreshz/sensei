use clap::{App, Arg, ArgMatches};
use rand::Rng;
use webbrowser;

const PHRASES: [&str; 3] = ["幸運を", "よく学ぶ", "良い読書"];

struct CrateInfo {
    name: String,
    version: String,
    query: String,
    url: String
}

impl CrateInfo {
    fn new(name: String, version: String, query: String, url: String) -> CrateInfo {
        CrateInfo {
            name,
            version,
            query,
            url,
        }
    }
    fn is_std(&self) -> bool {
        self.name == "std"
    }
}

/// Receives input from the user to process their request.
///
/// # Arguments
///
/// * `crate` - A string with the crate.
/// * `version` - Crate version you want to open (optional).
/// # Examples
///
/// ```
/// $ sensei serde
/// $ sensei serde -v 0.8.8
/// $ sensei serde -s Serializer
/// ```
fn main() {
    let matches = get_config();
    let name = matches.value_of("crate").unwrap().into();
    let mut crt = CrateInfo::new(name, String::new(), String::new(), String::new());
    check_config(&mut crt, matches);
}

/// Creates an object of type ArgMatches with the structure of the CLI.
fn get_config() -> ArgMatches<'static> {
    App::new("Sensei")
        .version("0.1.8")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Opens the documentation for any crate.")
        .arg(Arg::with_name("crate")
                 .help("What crate do you need help with, 学生?")
                 .short("c")
                 .long("crate")
                .required(true)
                .index(1),
        )
        .arg(Arg::with_name("version")
            .help("Opens documentation for a specific version.")
            .short("v")
            .long("version")
            .takes_value(true)
        )
        .arg(Arg::with_name("query")
                 .help("Specifies query to search documentation.")
                 .short("q")
                 .long("query")
                 .takes_value(true),
        )
        .get_matches()
}

/// Checks arguments and executes the required actions.
fn check_config(crt: &mut CrateInfo, matches: ArgMatches) {
    if matches.is_present("version") {
        crt.version = matches.value_of("version").unwrap().parse().unwrap();
    }
    if matches.is_present("query") {
        crt.query = matches.value_of("query").unwrap().parse().unwrap();
    }
    if crt.query.is_empty() && crt.version.is_empty() {
        crt.url = format!("https://docs.rs/{}", crt.name);
        open_url(crt.url.clone(), crt)
    } else {
        if !crt.version.is_empty() && !crt.query.is_empty() {
            crt.url = if crt.is_std() {
                format!("https://doc.rust-lang.org/{}/std/index.html?search={}", crt.version, crt.query)
            } else {
                format!("https://docs.rs/{}/{}/{}/?search={}",
                    crt.name, crt.version, crt.name, crt.query)
            };
            open_url(crt.url.clone(), crt)
        } else if !crt.version.is_empty() {
            crt.url = if crt.is_std() {
                format!("https://doc.rust-lang.org/{}/std/index.html", crt.version)
            } else {
                format!("https://docs.rs/{}/{}/{}", crt.name, crt.version, crt.name)
            };
            open_url(crt.url.clone(), crt)
        } else {
            crt.url = if crt.is_std() {
                format!("https://doc.rust-lang.org/std/index.html?search={}", crt.query)
            } else {
                format!("https://docs.rs/{}/?search={}", crt.name, crt.query)
            };
            open_url(crt.url.clone(), crt)
        }
    }
}

/// Opens the requested crate's documentation in the web browser.
fn open_url(url: String, crt: &CrateInfo) {
    if !(webbrowser::open(&*url).is_ok()) {
        println!("Seems like you've lost your way, 学生, try again.");
    } else {
        let mut rng = rand::thread_rng();
        if crt.is_std() {
            println!(
                "||| The Standard Library {}|||\n\n{}",
                format!("{} ", &crt.version),
                PHRASES[rng.gen_range(0, 2)]
            )
        } else {
            println!(
                "||| The Book Of {} {}|||\n\n{}",
                first_letter_to_uppercase(crt.name.clone()),
                format!("{} ", &crt.version),
                PHRASES[rng.gen_range(0, 2)]
            )
        }
    }
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_uppercase(c: String) -> String {
    match c.chars().next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c[1..],
    }
}
