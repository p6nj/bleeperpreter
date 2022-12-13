// TODO: use anyhow for error handling https://crates.io/crates/anyhow
mod doc;
mod file;
mod proc;

fn main() {
    let filename = proc::get_filename();
    let data = file::read_file(filename);
    proc::serialize(data);
}
