use clap::{App, Arg, ArgMatches, SubCommand};
use rand::Rng;
use webbrowser;

const PHRASES: [&str; 3] = ["幸運を", "よく学ぶ", "良い読書"];

struct CrateInfo {
    name: String,
    version: String,
    query: String,
}

impl CrateInfo {
    fn new(name: String, version: String, query: String) -> CrateInfo {
        CrateInfo {
            name,
            version,
            query,
        }
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
    let mut crt = CrateInfo::new(name, String::new(), String::new());
    check_config(&mut crt, matches);
}

/// Creates an object of type ArgMatches with the structure of the CLI.
fn get_config() -> ArgMatches<'static> {
    App::new("Sensei")
        .version("0.1.5")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Opens the documentation for any crate.")
        .arg(
            Arg::with_name("crate")
                .help("What crate do you need help with, 学生?")
                .required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("-v")
                .help("Flag used to specify a crate's version.")
                .arg(Arg::with_name("ver").help("ver").takes_value(true))
                .subcommand(
                SubCommand::with_name("-s")
                    .help("Used to specify a query.")
                    .arg(
                        Arg::with_name("query")
                            .help("Query to be used for search.")
                            .takes_value(true),
                    ),
            ),
        )
        .subcommand(
            SubCommand::with_name("-s")
                .help("Used to specify a query.")
                .arg(
                    Arg::with_name("query")
                        .help("Query to be used for search.")
                        .takes_value(true),
                ),
        )
        .get_matches()
}

/// Checks arguments and executes the required actions.
fn check_config(crt: &mut CrateInfo, matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("-v") {
        if matches.is_present("ver") {
            crt.version = matches.value_of("ver").unwrap().parse().unwrap();
            if let Some(matches) = matches.subcommand_matches("-s") {
                if matches.is_present("query") {
                    crt.query = matches.value_of("query").unwrap().parse().unwrap();
                    let uri = format!("https://docs.rs/{}/{}/{}/?search={}",
                                      crt.name, crt.version, crt.name, crt.query
                    );
                    open_url(uri, crt)
                } else {
                    println!("Please specify a query for this crate.");
                }
            } else {
                if matches.is_present("ver") {
                    crt.query = matches.value_of("ver").unwrap().parse().unwrap();
                    let uri = format!("https://docs.rs/{}/{}/{}", crt.name, crt.version, crt.name);
                    open_url(uri, crt)
                } else {
                    println!("Please specify a version for this crate.");
                }
            }
        } else {
            println!("Please specify a version for this crate.");
        }
    } else if let Some(matches) = matches.subcommand_matches("-s") {
        if matches.is_present("query") {
            crt.query = matches.value_of("query").unwrap().parse().unwrap();
            let uri = format!("https://docs.rs/{}/?search={}", crt.name, crt.query);
            open_url(uri, crt)
        } else {
            println!("Please specify a query for this crate.");
        }
    } else {
        open_url(format!("https://docs.rs/{}", crt.name), crt)
    }
}

/// Opens the requested crate's documentation in the web browser.
fn open_url(url: String, crt: &CrateInfo) {
    if !(webbrowser::open(&*url).is_ok()) {
        println!("Seems like you've lost your way, 学生, try again.");
    } else {
        let mut rng = rand::thread_rng();
        println!(
            "||| The Book Of {} {}|||\n\n{}",
            first_letter_to_uppercase(crt.name.clone()),
            format!("{} ", &crt.version),
            PHRASES[rng.gen_range(0, 2)]
        )
    }
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_uppercase(c: String) -> String {
    match c.chars().next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c[1..],
    }
}
