mod day1;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args.get(1).expect("Please provide a day to run");

    match day.as_str() {
        "day1" => day1::solve(),
        _ => panic!("Day {} not implemented", day),
    }
}
