use aoc2023::day1;
use aoc2023::day2;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1).expect("Please provide a day to run");

    println!("Running day {}", day);
    match day.as_str() {
        "day1" => println!("{:?}", day1::solve()),
        "day2" => println!("{:?}", day2::solve()),
        _ => println!("Day {} not implemented yet", day),
    }
}
