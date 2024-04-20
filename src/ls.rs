use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;
use std::io::IsTerminal;
use clap::Parser;

mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false, help="Do not ignore entries starting with .")]
    all: bool,

    #[arg(long, default_value_t = String::from("auto"))] // "always", "auto", "never"
    color: String,

    #[arg(long, default_value_t = false, help="Folder stats")]
    summary: bool,

    files: Vec<String>,
}

fn print_entry(file: &PathBuf, args: &Args, working_directory: &PathBuf, _indent: &bool) {
    let mut print_prefix = String::from("");

    if !args.all {
        if PathBuf::from(file).file_name().unwrap().to_str().unwrap().starts_with(".") {
            return;
        }
    }

    if args.color != "never" {
        if file.is_dir() {
            print_prefix = "\x1b[01;34m".to_string();
        } else {
            let f = fs::File::open(file.clone());
            if f.is_err() {
                return;
            }

            let metadata = f.unwrap().metadata().unwrap();
            if metadata.permissions().mode() & 0o111 != 0 {
                print_prefix = "\x1b[01;32m".to_string();
            } else {
                print_prefix = "".to_string();
            }

            let ansi_color = util::ansi_color_from_file_extension(file.to_str().unwrap());
            if ansi_color != "" {
                print_prefix = ansi_color.to_string();
            }
        }
    }

    if *_indent {
        print_prefix.insert_str(0, "    ");
    }

    println!("{}{}\x1b[0m", print_prefix, file.strip_prefix(&working_directory).unwrap().display());
}

fn main() {
    let mut args = Args::parse();

    // Don't output color if we're being piped into another program
    if args.color != "always" {
        if !std::io::stdout().is_terminal() {
            args.color = "never".to_string();
        }
    }

    if args.files.len() < 1 {
        args.files.push(util::working_directory().into_os_string().into_string().unwrap());
    }

    let show_dir_names = args.files.len() > 1;

    let mut dir_count = 0;
    let mut file_count = 0;

    for file_arg in &args.files {
        let file_arg_canonicalized = match PathBuf::from(file_arg).canonicalize() {
            Ok(x) => x,
            Err(_) => {
                println!("\x1b[01;31mNo such file or directory: {}", file_arg);
                continue;
            }
        };

        let paths = match fs::read_dir(&file_arg_canonicalized) {
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

        if args.summary {
            dir_count += directories.len();
            file_count += files.len();
            continue;
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

    if args.summary {
        println!("{} folders", dir_count);
        println!("{} files", file_count);
        println!("{} total", dir_count+file_count);
    }
}
