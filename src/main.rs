use clap::{Arg, App};
use webbrowser;

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
            .help("What crate do you need help with, gakusei?")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .value_name("version")
            .help("Specify version for a crate")
            .takes_value(true))
        .get_matches();
    let crt = matches.value_of("crate").unwrap();
    match matches.value_of("v") {
        Some(v) => open_url(format!("https://docs.rs/{}/{}/{}", crt, v, crt), crt),
        None => open_url(format!("https://docs.rs/{}", crt), crt)
    }
}

/// Opens the requested crate's documentation in the web browser.
fn open_url(url: String, crt: &str) {
    if webbrowser::open(&*url).is_ok() {
        println!("Here it is... the book of {}. よく学ぶ", crt)
    } else {
        println!("Seems like you've lost your way, gakusei, try again.");
    }
}
