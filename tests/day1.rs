use aoc2023::day1::{parse_line, part_one, part_two};
use aoc2023::read_file_to_vec_lines;

#[test]
fn test_overlaps() {
    assert_eq!(parse_line("sevenine"), Some((7, 9)));
}

#[test]
fn example1() {
    let file_lines = read_file_to_vec_lines("inputs/day1.example.txt");
    assert_eq!(file_lines.len(), 4);
    assert_eq!(part_one(&file_lines), 142);
}

#[test]
fn example2() {
    let file_lines = read_file_to_vec_lines("inputs/day1.example2.txt");
    assert_eq!(part_two(&file_lines), 281);
}

#[test]
fn using_input1() {
    let file_lines = read_file_to_vec_lines("inputs/day1.txt");
    assert_eq!(part_one(&file_lines), 55621);
}

#[test]
fn using_input2() {
    let file_lines = read_file_to_vec_lines("inputs/day1.txt");
    assert_eq!(part_two(&file_lines), 53592);
}
