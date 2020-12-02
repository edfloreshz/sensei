extern crate clap;
use clap::{Arg, App};
use webbrowser;

fn main() {
    let matches = App::new("Sensei")
        .version("0.1.0")
        .author("Eduardo F. <edfloreshz@gmail.com>")
        .about("Opens the documentation for any crate.")
        .arg(Arg::with_name("crate")
            .help("What crate do you need help with, gakusei?")
            .required(true)
            .index(1))
        .get_matches();
    let crt = matches.value_of("crate").unwrap();
    if webbrowser::open(&*format!("https://docs.rs/{}", crt)).is_ok() {
        println!("Here it is... the book of {}. よく学ぶ", crt)
    } else {
        println!("Seems like you've lost your way, gakusei, try again.");
    }
}
