use clap::{Arg, ArgMatches, Command};
use rust_analyzer::analyze_exercise;
use std::process;

/// Initializes the clap application.
fn init_app() -> ArgMatches {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("slug")
                .short('s')
                .long("slug")
                .help("The slug of the exercise to be analyzed (e.g. 'reverse-string').")
                .required(true),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("A path to a directory containing the submitted file(s).")
                .required(true),
        )
        .get_matches()
}

fn main() {
    let matches = init_app();
    if let Err(error) = analyze_exercise(
        matches.get_one::<String>("slug").unwrap(),
        matches.get_one::<String>("path").unwrap(),
    ) {
        eprintln!("[error] {}", error);
        process::exit(1);
    }
}
