use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;
use clap::Parser;

mod working_directory;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    all: bool,

    #[arg(long, default_value_t = String::from("always"))] // "always", "never"
    color: String
}

fn print_entry(file: &PathBuf, args: &Args, working_directory: &PathBuf) {
    let mut color_prefix: &str = "";

    if !args.all {
        if file.starts_with(".") {
            return;
        }
    }

    if args.color != "never" {
        if file.is_dir() {
            color_prefix = "\x1b[01;34m";
        } else {
            let f = fs::File::open(file.clone());
            let metadata = f.unwrap().metadata().unwrap();
            if metadata.permissions().mode() & 0o111 != 0 {
                color_prefix = "\x1b[01;32m";
            } else {
                color_prefix = "";
            }
        }
    }

    print!("{}{}\x1b[0m\n", color_prefix, file.strip_prefix(&working_directory).unwrap().display());
}

fn main() {
    let args = Args::parse();

    let working_directory = working_directory::working_directory();
    let paths = match fs::read_dir(&working_directory) {
        Err(_) => process::exit(0),
        Ok(paths) => paths
    };

    let mut directories: Vec<PathBuf> = vec![];
    let mut files: Vec<PathBuf> = vec![];
    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            directories.push(path.as_ref().unwrap().path());
        } else {
            files.push(path.as_ref().unwrap().path());
        }
    }

    for dir in directories {
        print_entry(&dir, &args, &working_directory);
    }

    for file in files {
        print_entry(&file, &args, &working_directory);
    }
}
