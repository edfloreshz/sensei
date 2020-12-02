use clap::{Arg, App, SubCommand};
use webbrowser;
use rand::Rng;

struct CrateInfo {
    name: String,
    version: String
}

const PHRASES: [&str; 3] = ["幸運を","よく学ぶ","良い読書"];

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
    /// $ sensei serde 0.8.8
    /// ```
fn main() {
    let matches = App::new("Sensei")
        .version("0.1.0")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Opens the documentation for any crate.")
        .arg(Arg::with_name("crate")
            .help("What crate do you need help with, 学生?")
            .required(true)
            .index(1))
        .subcommand(SubCommand::with_name("-v")
            .help("Flag used to specify a crate's version.")
            .arg(Arg::with_name("ver")
                .help("ver")
                .takes_value(true)))
        .get_matches();

    let mut crt = CrateInfo {
        name: matches.value_of("crate").unwrap().into(),
        version: "Latest version".into()
    };
    if let Some(matches) = matches.subcommand_matches("-v") {
        if matches.is_present("ver") {
            crt.version = matches.value_of("ver").unwrap().parse().unwrap();
            open_url(
                format!("https://docs.rs/{}/{}/{}", crt.name, crt.version, crt.name),
                crt
            )
        } else {
            println!("Please specify a version for this crate.");
        }
    } else {
        open_url(format!("https://docs.rs/{}", crt.name), crt)
    }
}

/// Opens the requested crate's documentation in the web browser.
fn open_url(url: String, crt: CrateInfo) {
    if webbrowser::open(&*url).is_ok() {
        let mut rng = rand::thread_rng();
        println!(
            "||| The Book Of {} |||\n{}\n{}", first_letter_to_uppercase(crt.name), crt.version,
            PHRASES[rng.gen_range(0,2)]
        )
    } else {
        println!("Seems like you've lost your way, 学生, try again.");
    }
}

/// Converts the first letter of a crate's name to upper case.
fn first_letter_to_uppercase(c: String) -> String {
    match c.chars().next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c[1..],
    }
}