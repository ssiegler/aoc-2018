use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

pub fn file_lines() -> impl Iterator<Item=String> {
    let filename = env::args()
        .nth(1)
        .expect("missing filename argument");
    let file = File::open(filename)
        .expect("could not open file");
    return BufReader::new(file)
        .lines()
        .map(|line| line.expect("error reading file"));
}
