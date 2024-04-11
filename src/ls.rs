use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;
use clap::Parser;

mod util;

#[derive(Parser, Debug)]
//#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    all: bool,

    #[arg(long, default_value_t = String::from("always"))] // "always", "never"
    color: String,

    files: Vec<String>,
}

fn print_entry(file: &PathBuf, args: &Args, working_directory: &PathBuf, _indent: &bool) {
//    let mut color_prefix: &str = "";
    let mut color_prefix = String::from("");

    if !args.all {
        if file.starts_with(".") {
            return;
        }
    }

    if args.color != "never" {
        if file.is_dir() {
            color_prefix = "\x1b[01;34m".to_string();
        } else {
            let f = fs::File::open(file.clone());
            if f.is_err() {
                return;
            }

            let metadata = f.unwrap().metadata().unwrap();
            if metadata.permissions().mode() & 0o111 != 0 {
                color_prefix = "\x1b[01;32m".to_string();
            } else {
                color_prefix = "".to_string();
            }
        }
    }

    if *_indent {
        color_prefix.insert_str(0, "    ");
    }

    print!("{}{}\x1b[0m\n", color_prefix, file.strip_prefix(&working_directory).unwrap().display());
}

fn main() {
    let mut args = Args::parse();

    if args.files.len() < 1 {
        args.files.push(util::working_directory().into_os_string().into_string().unwrap());
    }

    let show_dir_names = args.files.len() > 1;

    for file_arg in &args.files {
        let file_arg_canonicalized = PathBuf::from(file_arg).canonicalize().unwrap();
        let paths = match fs::read_dir(file_arg_canonicalized.clone()) {
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

        if show_dir_names {
            println!("\x1b[01;34m{}\x1b[0m/", util::path_without_slash_suffix(file_arg));
        }

        for dir in directories {
            print_entry(&dir, &args, &file_arg_canonicalized, &show_dir_names);
        }

        for file in files {
            print_entry(&file, &args, &file_arg_canonicalized, &show_dir_names);
        }
    }
}
