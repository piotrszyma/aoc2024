use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_lines_from_file(path: &str) -> impl Iterator<Item = io::Result<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}
