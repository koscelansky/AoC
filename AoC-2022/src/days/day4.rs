use std::fs;
use itertools::Itertools;


pub fn day4(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    let lines: Vec<&str> = content.split('\n').collect();
    for line in lines.iter() {
        let (a1, a2, b1, b2) = line.split([',', '-']).map(|x| x.parse::<u32>().unwrap()).tuples().next().unwrap();

        if b1 >= a1 && b2 <= a2 || a1 >= b1 && a2 <= b2 {
            part1 += 1;
            continue;
        }

        if b1 <= a1 && b2 >= a1 || b1 <= a2 && b2 >= a2 {
            part2 += 1;
        }
    }

    println!("Answer to 1. half is {part1}");

    println!("Answer to 1. half is {}", part1 + part2);
}