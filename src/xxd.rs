use std::io;
use std::cmp;
use std::io::prelude::*;
use std::fs::File;
use std::io::IsTerminal;
use std::process::{self};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>,

    #[arg(long, default_value_t = String::from("auto"))] // "always", "auto", "never"
    color: String,
}

const HEX_LOOKUP: &[u8] = "0123456789abcdef".as_bytes();
const BYTES_PER_LINE: usize = 16;
const GROUP_BYTES: usize = 2;

fn color_of_byte(b: u8) -> String {
    if b == 0x00 {
        return "\x1b[1;37m".to_string();
    }

    if b == 0xff {
        return "\x1b[1;34m".to_string();
    }

    if b == 0x09 || b == 0x0a || b == 0x0d {
        return "\x1b[1;33m".to_string();
    }

    if b >= 0x7f || b < 0x20 {
        return "\x1b[1;31m".to_string();
    }

    if b == b' ' {
        return "\x1b[1;34m".to_string();
    }

    if b.is_ascii_whitespace() {
        return "\x1b[1;33m".to_string();
    }

    return "\x1b[1;32m".to_string();
}

fn should_print_dot(b: u8) -> bool {
    if b >= 0x7f {
        return true;
    }

    if b < 0x20 {
        return true;
    }

    return false;
}

fn handle_buf(buf: [u8; 512], size: usize, index: usize, color: &String) -> bool {
    if size == 0 {
        return false;
    }

    for line_index in (0..size).step_by(BYTES_PER_LINE) {
        print!("{}", format!("{:#010x}: ", index+line_index).to_string().strip_prefix("0x").unwrap());

        let mut i = 0;
        for b in &buf[line_index..cmp::min(size, line_index+BYTES_PER_LINE)] {
            if color != "never" {
                print!("{}", color_of_byte(*b));
            }

            print!("{}{}", HEX_LOOKUP[(b>>4) as usize] as char, HEX_LOOKUP[(b & 0xf) as usize] as char);

            if color != "never" {
                print!("\x1b[0m");
            }

            i += 1;
            if i % GROUP_BYTES == 0 {
                print!(" ")
            }
        }

        print!(" ");

        let n_expected = 2*BYTES_PER_LINE + ((BYTES_PER_LINE / GROUP_BYTES) as f64).ceil() as usize;
        let n_bytes_printed = 2*i + ((i / GROUP_BYTES) as f64).ceil() as usize;

        let whitespace = String::from(" ").repeat(n_expected - n_bytes_printed);
        print!("{}", whitespace);

        for b in &buf[line_index..cmp::min(size, line_index+BYTES_PER_LINE)] {
            if color != "never" {
                print!("{}", color_of_byte(*b));
            }

            if should_print_dot(*b) {
                print!(".");
            } else {
                print!("{}", *b as char);
            }

            if color != "never" {
                print!("\x1b[0m");
            }
        }

        println!();
    }

    return true;
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    // Don't output color if we're being piped into another program
    if args.color != "always" {
        if !std::io::stdout().is_terminal() {
            args.color = "never".to_string();
        }
    }

    // No input files? Read from STDIN
    if args.files.is_empty() {
        loop {
            let mut buf = [0; 512];
            let mut i = 0;
            match io::stdin().read(&mut buf) {
                Ok(len) => if len == 0 {
                    break;
                } else {
                    if !handle_buf(buf, len, i, &args.color) {
                        break;
                    }
                    i += len;
                }
                Err(_) => {
                    break;
                }
            }
        }
        process::exit(0);
    }

    for file in &args.files {
        if let Ok(mut f) = File::open(file) {
            let mut i = 0;
            let mut buf = [0; 512];
            loop {
                let bytes_read = f.read(&mut buf).unwrap();
                if !handle_buf(buf, bytes_read, i, &args.color) {
                    break;
                }
                i += bytes_read;
            }
        } else {
            if args.color != "never" {
                eprintln!("\x1b[1;31mNo such file or directory: {}\x1b[0m", file);
            } else {
                eprintln!("No such file or directory: {}", file);
            }
        }
    }

    Ok(())
}
