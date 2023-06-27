use std::fs::read_to_string;

mod json;

use clap::Parser;
#[derive(Debug, Parser)]
#[clap(version, about = "An advanced MML interpreter")]
struct Arguments {
    input_filename: String,
}

fn main() {
    let args = Arguments::parse();
    let content = read_to_string(args.input_filename).unwrap();
    dbg!(content);
}
