use std::fs::read_to_string;

mod json;

use ::json::JsonValue;
use clap::Parser;
#[derive(Debug, Parser)]
#[clap(version, about = "An advanced MML interpreter")]
struct Arguments {
    json_path: String,
}

fn main() {
    let args = Arguments::parse();
    let content = read_to_string(args.json_path).unwrap();
    println!("{}", JsonValue::from(content));
}
