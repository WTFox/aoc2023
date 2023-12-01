use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod day1;

#[derive(Debug)]
pub struct Answer {
    pub part_one: String,
    pub part_two: String,
}

pub fn read_file_to_vec_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("can't open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("can't read line"))
        .collect()
}
