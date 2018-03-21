use clap::{Arg, ArgMatches};

pub fn parse_args(current_dir: &str) -> ArgMatches {
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
