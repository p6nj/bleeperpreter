// Doc lib, library containing all the text of the project.

pub static USAGE: &'static str = "Usage: interbeeper FILENAME";

// TODO: Error descriptions
pub fn panic(err: u8) {
    panic!("{err}");
}