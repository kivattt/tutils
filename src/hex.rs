use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    nonewline: bool,

    files: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let hex_lookup = "0123456789abcdef".as_bytes();

    // No input files? Read from STDIN
    // FIXME: A little repetitive
    if args.files.is_empty() {
        loop {
            let mut buf = [0; 512];
            match io::stdin().read(&mut buf) {
                Ok(len) => if len == 0 {
                    break;
                } else {
                    for b in &buf[..len] {
                        print!("{}{}", hex_lookup[(b>>4) as usize] as char, hex_lookup[(b & 0xf) as usize] as char);
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
                if bytes_read == 0 {
                    if !args.nonewline {
                        println!();
                    }
                    break;
                }

                for b in &buf[..bytes_read] {
                    print!("{}{}", hex_lookup[(b>>4) as usize] as char, hex_lookup[(b & 0xf) as usize] as char);
                }
            }
        } else {
            eprintln!("\x1b[01;31mNo such file or directory: {}\x1b[0m", file);
        }
    }

    Ok(())
}
