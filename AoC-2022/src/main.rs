mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}

fn main() {
    let day = 9;
    let input = "input.txt";

    match day {
        1 => days::day1::day1(input),
        2 => days::day2::day2(input),
        3 => days::day3::day3(input),
        4 => days::day4::day4(input),
        5 => days::day5::day5(input),
        6 => days::day6::day6(input),
        7 => days::day7::day7(input),
        8 => days::day8::day8(input),
        9 => days::day9::day9(input),
        _ => std::process::exit(1)
    }
}
