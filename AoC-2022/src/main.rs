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
    pub mod day10;
    pub mod day11;
    pub mod day12;
}

fn main() {
    let day = 12;
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
        10 => days::day10::day10(input),
        11 => days::day11::day11(input),
        12 => days::day12::day12(input),
        _ => std::process::exit(1)
    }
}
