use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in &args.files {
        print!("{}", fs::read_to_string(file).unwrap());
    }
}
