use std::{fs, io::{self, Read, Write}, process};
use std::io::IsTerminal;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>,

    #[arg(long, default_value_t = String::from("auto"))] // "always", "auto", "never"
    color: String,
}

fn main() {
    let mut args = Args::parse();

    // Don't output color if we're being piped into another program
    if !std::io::stdout().is_terminal() && args.color != "always" {
        args.color = "never".to_string();
    }

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
        let file_str = fs::read(file);
        if file_str.is_err() {
            if args.color != "never" {
                eprintln!("\x1b[01;31mNo such file: {}\x1b[0m", file);
            } else {
                eprintln!("No such file: {}", file);
            }
            process::exit(1);
        }

        let _ = std::io::stdout().write(&file_str.unwrap());
    }
}
