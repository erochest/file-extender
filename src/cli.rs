use clap::{Arg, ArgMatches};

pub fn parse_args<'a>(current_dir: &'a str, default_magic: &'a str) -> ArgMatches<'a> {
    app_from_crate!()
        .arg(
            Arg::with_name("magic_file")
                .short("m")
                .long("magic")
                .default_value(&default_magic)
                .value_name("MAGIC_FILE")
                .takes_value(true)
                .required(false)
                .help("The location of the magic file."),
        )
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
