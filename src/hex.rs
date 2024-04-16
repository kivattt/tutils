use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process::{self};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    nonewline: bool,

    #[arg(short, long, default_value_t = false, help="Decode")]
    decode: bool,

    #[arg(long, default_value_t = false, help="If non-hex character found during decode, exit with 1")]
    noignore: bool,

    files: Vec<String>,
}

const HEX_LOOKUP: &[u8] = "0123456789abcdef".as_bytes();

fn handle_buf(buf: [u8; 512], size: usize, nonewline: bool, decode: bool, noignore: bool) -> bool {
    if size == 0 {
        if !nonewline {
            println!();
        }
        return false;
    }

    if decode {
        for i in (0..size).step_by(2) {
            if i+1 >= size {
                break;
            }

            let b1 = HEX_LOOKUP.iter().position(|&e| e == buf[i]);
            let b2 = HEX_LOOKUP.iter().position(|&e| e == buf[i+1]);
            if b1.is_none() || b2.is_none() {
                if noignore {
                    process::exit(1);
                } else {
                    continue; // FIXME: Only step by 1 here to ignore non-hex characters
                }
            }
            print!("{}", ((b1.unwrap() << 4 | b2.unwrap()) as u8) as char);
        }
    } else {
        for b in &buf[..size] {
            print!("{}{}", HEX_LOOKUP[(b>>4) as usize] as char, HEX_LOOKUP[(b & 0xf) as usize] as char);
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // No input files? Read from STDIN
    if args.files.is_empty() {
        loop {
            let mut buf = [0; 512];
            match io::stdin().read(&mut buf) {
                Ok(len) => if len == 0 {
                    break;
                } else {
                    if !handle_buf(buf, len, args.nonewline, args.decode, args.noignore) {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        if !args.nonewline {
            println!();
        }
        process::exit(0);
    }

    for file in &args.files {
        if let Ok(mut f) = File::open(file) {
            let mut buf = [0; 512];
            loop {
                let bytes_read = f.read(&mut buf).unwrap();
                if !handle_buf(buf, bytes_read, args.nonewline, args.decode, args.noignore) {
                    break;
                }
            }
        } else {
            eprintln!("\x1b[01;31mNo such file or directory: {}\x1b[0m", file);
        }
    }

    Ok(())
}
