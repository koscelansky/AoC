mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
}

fn main() {
    let day = 3;
    let input = "input.txt";

    match day {
        1 => days::day1::day1(input),
        2 => days::day2::day2(input),
        3 => days::day3::day3(input),
        _ => std::process::exit(1)
    }
}
