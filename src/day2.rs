use crate::{read_file_to_vec_lines, Answer};

pub fn solve() -> Answer {
    let file_lines = read_file_to_vec_lines("inputs/day2.txt");
    Answer {
        part_one: part_one(&file_lines).to_string(),
        part_two: part_two(&file_lines).to_string(),
    }
}

#[allow(unused_variables)]
pub fn part_one(lines: &[String]) -> i32 {
    42
}

#[allow(unused_variables)]
pub fn part_two(lines: &[String]) -> i32 {
    42
}
