#[macro_use]
extern crate clap;

use std::collections::VecDeque;
use std::env;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
use clap::{Arg, ArgMatches};

fn main() {
    let cwd = env::current_dir()
        .ok()
        .and_then(|d| d.to_str().map(String::from))
        .unwrap_or_else(|| String::from("."));
    let matches = parse_args(&cwd);
    let input_dir = matches
        .value_of("input_directory")
        .expect("No value given for INPUT_DIRECTORY.");

    for entry in iter_tree(Path::new(&input_dir)) {
        let entry = entry.expect("Cannot read from input directory.");
        //     Run file
        //     Parse its output
        //     Copy the file to destination_dir with the appropriate extension
    }

    println!("from {:?}", &matches.value_of("input_directory"));
    println!("to   {:?}", &matches.value_of("destination_dir"));
}

struct TreeWalker {
    q: VecDeque<io::Result<DirEntry>>,
}

impl Iterator for TreeWalker {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

fn iter_tree(root: &Path) -> TreeWalker {
    unimplemented!()
}

fn parse_args(current_dir: &str) -> ArgMatches {
    app_from_crate!()
        .arg(
            Arg::with_name("destination_dir")
                .short("d")
                .long("dest")
                .default_value(&current_dir)
                .value_name("DIRECTORY")
                .takes_value(true)
                .required(true)
                .help("The destination directory."),
        )
        .arg(
            Arg::with_name("input_directory")
                .value_name("DIRECTORY")
                .default_value(&current_dir)
                .takes_value(true)
                .required(true)
                .help("The directory to walk looking for files to extend."),
        )
        .get_matches()
}
