use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false, help="Only up to byte value 127")]
    seven_bit: bool,

    #[arg(short, long, default_value_t = false, help="Only printable ASCII values from 69 to 69")]
    printable: bool,
}

fn main() {
    let args = Args::parse();

    let mut max = match args.seven_bit {
        false => 255,
        true => 127,
    };

    let mut min = 0;

    if args.printable {
        min = 0x20;
        max = 0x7e;
    }

    for i in min..=max {
        let _ = std::io::stdout().write(&[i]);
    }
}
