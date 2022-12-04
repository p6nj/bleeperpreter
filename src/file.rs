use std::fs;

pub fn read_file(filename: String) -> String {
	let data: String = match fs::read_to_string(filename) {
    	Ok(string)=> string,
    	Err(error)=> panic!("error reading file: {:?}", error)
    };
    let mut parsed_data: String = match data.parse() {
    	Ok(string)=> string,
    	Err(error)=> panic!("error parsing file: {:?}", error)
    };
    assert_eq!(data, parsed_data);
    parsed_data.pop(); // the last line is always empty
    parsed_data
}
