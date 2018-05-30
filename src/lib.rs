#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate magic;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use failure::Error;
use magic::flags;
use magic::Cookie;

mod cli;
mod walker;

use walker::TreeWalker;

type Result<R> = std::result::Result<R, Error>;

pub fn run() -> Result<()> {
    let default_magic = "/usr/local/share/misc/magic.mgc".to_string();
    let cwd = env::current_dir()
        .ok()
        .and_then(|d| d.to_str().map(String::from))
        .unwrap_or_else(|| String::from("."));
    let matches = cli::parse_args(&cwd, &default_magic);

    let input_dir = matches
        .value_of("input_directory")
        .ok_or_else(|| format_err!("No value given for INPUT_DIRECTORY."))?;
    let destination_dir = matches
        .value_of("destination_dir")
        .map(PathBuf::from)
        .ok_or_else(|| format_err!("No value given for DIRECTORY."))?;
    let magic_file = matches
        .value_of("magic_file")
        .ok_or_else(|| format_err!("No value given for MAGIC_FILE."))?;

    let cookie = Cookie::open(flags::MIME_TYPE)
        .map_err(|err| format_err!("Unable to open file cookie: {}", &err))?;
    cookie
        .load(&[magic_file])
        .map_err(|err| format_err!("Unable to read magic file {:?}: {}", &magic_file, &err))?;

    let entries = TreeWalker::new(Path::new(&input_dir))
        .filter(|entry| entry.is_file())
        .filter_map(|entry| cookie.file(&entry).ok().map(|mime| (entry, mime)))
        .filter_map(|(entry, mime_type)| {
            get_dest(&entry, &destination_dir, &mime_type).map(|dest| (entry, dest))
        });

    for (src, dest) in entries {
        println!("cp {:?} => {:?}", &src, &dest);
        fs::copy(&src, &dest)
            .map_err(|err| format_err!("Unable to copy {:?} => {:?}.: {}", &src, &dest, &err))?;
    }

    Ok(())
}

fn get_dest(src_file: &Path, destination_dir: &Path, mime_type: &str) -> Option<PathBuf> {
    src_file.file_name().and_then(|file_name| {
        get_extension(&mime_type).map(|ext| destination_dir.join(file_name).with_extension(ext))
    })
}

fn get_extension(mime_type: &str) -> Option<String> {
    match mime_type {
        "application/octet-stream" => Some(String::from("bin")),
        "image/jpeg" => Some(String::from("jpeg")),
        "inode/x-empty" => None,
        "text/plain" => Some(String::from("txt")),
        "text/x-shellscript" => Some(String::from("sh")),
        "video/3gpp" => Some(String::from("3gp")),
        "video/x-msvideo" => Some(String::from("avi")),
        _ => None,
    }
}
