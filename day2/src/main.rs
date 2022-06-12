use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

fn read_file() -> String {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    return content;
}