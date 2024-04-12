use std::{fs, io::{self, Read}, process};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    // No input files? Read from STDIN
    if args.files.is_empty() {
        loop {
            let mut buf = [0; 512];
            match io::stdin().read(&mut buf) {
                Ok(len) => if len == 0 {
                    break;
                } else {
                    for b in &buf[..len] {
                        print!("{}", *b as char);
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        process::exit(0);
    }

    for file in &args.files {
        let file_str = fs::read_to_string(file);
        if file_str.is_err() {
            eprintln!("\x1b[01;31mNo such file: {}\x1b[0m", file);
            continue;
        }
        print!("{}", file_str.unwrap());
    }
}
