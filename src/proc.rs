use std::env;
use crate::doc::USAGE;

pub fn get_filename() -> String {
	let args:Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("{}", USAGE);
		panic!("parsing arguments: invalid number of arguments.");
	}
	if args[1] == "-h" || args[1] == "--help" {
		println!("{}", USAGE);
		panic!();
	}
	let filename: String = String::from(&args[1]);
	filename
}

pub fn serialize(data: String) {
	let mut chars = data.chars();
	let mut item = chars.next();
	let mut character: char;
	while item!=None {
		character = match item {
			Some(c) => c,
			None => '!' // this will never occur
		};
		println!("{:?}", character);
		item = chars.next();
	}
}