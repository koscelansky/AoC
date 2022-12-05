mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
}

fn main() {
    let day = 4;
    let input = "input.txt";

    match day {
        1 => days::day1::day1(input),
        2 => days::day2::day2(input),
        3 => days::day3::day3(input),
        4 => days::day4::day4(input),
        _ => std::process::exit(1)
    }
}
