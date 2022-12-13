use crate::doc::{USAGE, panic};

enum Word {
	L(u8),
	O(u8),
	N(u8)
}

struct Channel {
	title: String,
	words: Vec<Word>
}

struct Song {
	title: String,
	author: String,
	scale: Vec<&'static str>,
	channels: Vec<Channel>,
	bpm: u16
}

pub fn get_filename() -> String {
	let args:Vec<String> = std::env::args().collect();
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
	let chars = &mut data.chars();
	let mut character = chars.next();

	let mut song = Song {
		title: String::new(),
		author: String::new(),
		scale: Vec::from(["c","C","d","D","e","f","F","g","G","a","A","b"]),
		channels: Vec::new(),
		bpm: 60
	};

	while !character.is_none() {
		
		match
			match character.unwrap() {
				't'	=>	{
					let character = &chars.next();
					if character.is_none() {break;}
					match character.unwrap() {
						'i'	=>	{ // title
							let mut chars = chars.as_str().strip_prefix("tle:").expect("Expected title header.").trim_start().chars();
							let mut character = chars.nth(0);
							if character.unwrap()=='\n' {panic(0u8);}
							while character.unwrap()!='\n' {
								if character.is_none() {break;}
								song.title.push(match character.unwrap() {
									'\\'	=>	{
										match chars.next().unwrap() {
											'n'			=>	'\n',
											other	=>	other
										}
									}
									anything	=>	anything
								});
								character = chars.next();
							} println!("Modified title: '{}'", song.title);
						},
						_	=>	()
					}
				},
				_	=>	()
			}

			{
				_ => ()
			};
		
		character = chars.next();

	};
}