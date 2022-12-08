mod file;
mod proc;
mod doc;

fn main() {
	let filename = proc::get_filename();
	let data = file::read_file(filename);
	proc::serialize(data);
}