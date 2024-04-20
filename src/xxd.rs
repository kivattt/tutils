use std::io;
use std::cmp;
use std::io::prelude::*;
use std::fs::File;
use std::process::{self};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>,

    #[arg(long, default_value_t = String::from("always"))] // "always", "never"
    color: String,
}

const HEX_LOOKUP: &[u8] = "0123456789abcdef".as_bytes();
const BYTES_PER_LINE: usize = 16;
const GROUP_BYTES: usize = 2;

fn color_of_byte(b: u8) -> String {
    if !b.is_ascii() {
        return "\x1b[0;91m".to_string();
    }

    if b.is_ascii_whitespace() {
        return "\x1b[0;93m".to_string();
    }

    if b.is_ascii() {
        return "\x1b[0;92m".to_string();
    }

    return "".to_string();
}

fn should_print_dot(b: u8) -> bool {
    if !b.is_ascii() {
        return true;
    }

    if b == b'\n' {
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

        if i % GROUP_BYTES != 0 {
            print!(" ");
        }

//        let whitespace = String::from(" ").repeat(2*BYTES_PER_LINE + BYTES_PER_LINE / GROUP_BYTES);
//        let whitespace = String::from(" ").repeat(2*(BYTES_PER_LINE - i) + (i - i/GROUP_BYTES));
//        let whitespace = String::from(" ").repeat(2*(BYTES_PER_LINE - i) * (i/GROUP_BYTES));
//        let whitespace = String::from(" ").repeat(2*BYTES_PER_LINE - n_bytes_printed);
//        let whitespace = String::from(" ").repeat((BYTES_PER_LINE - i) * (i / GROUP_BYTES) * 2);
        let whitespace = String::from(" ").repeat((BYTES_PER_LINE - i) * 2);
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
/*
    let mut i = index;
    print!("{}", format!("{:#010X}: ", i).to_string().strip_prefix("0x").unwrap());
    for b in &buf[..size] {
        print!("{}{}", HEX_LOOKUP[(b>>4) as usize] as char, HEX_LOOKUP[(b & 0xf) as usize] as char);
        i += 1;
        if i % 2 == 0 {
            print!(" ");
        }

        if i % BYTES_PER_LINE == 0 {
            println!();
            print!("{}", format!("{:#010X}: ", i).to_string().strip_prefix("0x").unwrap());
        }
    }

    if i > index {
        println!();
    }*/

    return true;
}

fn main() -> io::Result<()> {
    let args = Args::parse();

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
                eprintln!("\x1b[01;31mNo such file or directory: {}\x1b[0m", file);
            } else {
                eprintln!("No such file or directory: {}", file);
            }
        }
    }

    Ok(())
}
