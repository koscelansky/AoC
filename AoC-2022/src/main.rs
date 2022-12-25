use std::time::Instant;

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
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
}

fn main() {
    let start = Instant::now();

    let day = 24;
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
        13 => days::day13::day13(input),
        14 => days::day14::day14(input),
        15 => days::day15::day15(input),
        16 => days::day16::day16(input), // this one better run in release :)
        17 => days::day17::day17(input),
        18 => days::day18::day18(input),
        19 => days::day19::day19(input),
        20 => days::day20::day20(input),
        21 => days::day21::day21(input),
        22 => days::day22::day22(input),
        23 => days::day23::day23(input),
        24 => days::day24::day24(input),
        _ => std::process::exit(1)
    }

    let duration = start.elapsed();

    println!("Time for day {} is: {:?}", day, duration);
}
