use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;

#[derive(Debug)]
pub struct Answers {
    part1: String,
    part2: String,
}

fn read_file_to_vec_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("can't open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("can't read line"))
        .collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1).expect("Please provide a day to run");

    println!("Running day {}", day);
    match day.as_str() {
        "day1" => println!("{:?}", day1::solve()),
        _ => println!("Day {} not implemented yet", day),
    }
}
