mod file;
mod proc;
mod doc;

fn main() {
	let filename: String = proc::get_filename();
	println!("{}", file::read_file(filename));
}
