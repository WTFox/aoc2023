use super::{read_file_to_vec_lines, Answers};

use regex::Regex;
use std::collections::HashMap;

pub fn solve() -> Answers {
    let file_lines = read_file_to_vec_lines("inputs/day1.txt");
    Answers {
        part1: solve_part1(file_lines.clone()).to_string(),
        part2: solve_part2(file_lines).to_string(),
    }
}

fn solve_part1(lines: Vec<String>) -> i64 {
    lines
        .iter()
        .map(|l| {
            let nums = l.chars().filter(|c| c.is_ascii_digit());
            let last = nums.clone().next().unwrap();
            let first = nums.clone().last().unwrap_or(last);
            format!("{}{}", last, first).parse::<i64>().unwrap()
        })
        .sum()
}

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let num_map = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    let re = Regex::new(r"\b(zero|one|two|three|four|five|six|seven|eight|nine|\d)\b").unwrap();
    let mut results = vec![];
    for end in 1..=line.len() {
        for start in 0..end {
            let substring = &line[start..end];
            if let Some(capture) = re.captures(substring) {
                if let Some(matched) = capture.get(0) {
                    let matched_str = matched.as_str();
                    if let Ok(digit) = matched_str.parse::<i32>() {
                        results.push(digit);
                    } else if let Some(&num) = num_map.get(matched_str) {
                        results.push(num);
                    }
                }
            }
        }
    }
    let mut numbers = results;

    if numbers.is_empty() {
        None
    } else {
        let last = numbers.pop().unwrap();
        let first = numbers.first().cloned().unwrap_or(last);
        Some((first, last))
    }
}

fn solve_part2(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .enumerate()
        .filter_map(|(_, line)| parse_line(line.trim()))
        .map(|(first, last)| first * 10 + last)
        .sum::<i32>()
}

mod tests {
    use super::{parse_line, read_file_to_vec_lines, solve_part1, solve_part2};

    #[test]
    fn test_overlaps() {
        assert_eq!(parse_line("sevenine"), Some((7, 9)));
    }

    #[test]
    fn example1() {
        let file_lines = read_file_to_vec_lines("inputs/day1.example.txt");
        assert_eq!(file_lines.len(), 4);
        assert_eq!(solve_part1(file_lines), 142);
    }

    #[test]
    fn example2() {
        let file_lines = read_file_to_vec_lines("inputs/day1.example2.txt");
        assert_eq!(solve_part2(file_lines), 281);
    }

    #[test]
    fn using_input1() {
        let file_lines = read_file_to_vec_lines("inputs/day1.txt");
        assert_eq!(solve_part1(file_lines), 55621);
    }

    #[test]
    fn using_input2() {
        let file_lines = read_file_to_vec_lines("inputs/day1.txt");
        assert_eq!(solve_part2(file_lines), 53592);
    }
}
