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
	let mut chars = data.chars();
	let mut characters = vec![&chars.next()];

	let mut song = Song {
		title: String::new(),
		author: String::new(),
		scale: Vec::from(["c","C","d","D","e","f","F","g","G","a","A","b"]),
		channels: Vec::new(),
		bpm: 60
	};

	while !characters[0].is_none() {
		
		match
			match characters[0].unwrap() {
				't'	=>	{
					characters.push(&chars.next());
					match characters[1].unwrap() {
						'i'	=>	{ // title
							chars.skip_while(|x| x!=&':');
							chars.skip_while(|x| x==&' ');
							characters.push(&chars.next());
							if characters[2].unwrap()=='\n' {panic(0u8);}
							while characters[2].unwrap()!='\n' {
								match chars.next().unwrap() {
									' '	=>	continue,
									not_a_space	=>	{
										song.title.push(match not_a_space {
											'\\'	=>	{
												match chars.next().unwrap() {
													'n'			=>	'\n',
													other	=>	other
												}
											}
											anything	=>	anything
										});
									}
								};
							};
							"Title: {song.title}"
						},
						_	=>	""
					}
				},
				_	=>	""
			}

			{
				str	=>	println!("{:?}", str),
				_ => continue
			};
		
			characters = vec![&chars.next()];

		};
	}