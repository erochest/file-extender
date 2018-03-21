#[macro_use]
extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::env;
use std::path::Path;

mod walker;
mod cli;

use walker::TreeWalker;

pub fn run() {
    let cwd = env::current_dir()
        .ok()
        .and_then(|d| d.to_str().map(String::from))
        .unwrap_or_else(|| String::from("."));
    let matches = cli::parse_args(&cwd);
    let input_dir = matches
        .value_of("input_directory")
        .expect("No value given for INPUT_DIRECTORY.");

    for _entry in TreeWalker::new(Path::new(&input_dir)) {
        //     Run file
        //     Parse its output
        //     Copy the file to destination_dir with the appropriate extension
    }

    println!("from {:?}", &matches.value_of("input_directory"));
    println!("to   {:?}", &matches.value_of("destination_dir"));
}
