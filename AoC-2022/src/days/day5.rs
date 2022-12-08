use std::fs;
use regex::Regex;

fn follow_instructions(instructions: &[&str], mut crates: Vec<String>, model: u32) -> Vec<String> {
    let parse = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();

    for instruction in instructions {
        let captures = parse.captures(instruction).unwrap();

        let count: usize = captures[1].parse().unwrap();
        let from: usize = captures[2].parse::<usize>().unwrap() - 1;
        let to: usize = captures[3].parse::<usize>().unwrap() - 1;

        let from_str = &mut crates[from];
    
        let drained = from_str.drain(from_str.len() - count..);
        let carry = if model == 9001 {
            drained.collect::<String>()
        } else {
            drained.rev().collect()
        };
        crates[to].push_str(&carry);
    }

    crates
}

pub fn day5(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let split = lines.iter().position(|w| w.is_empty()).unwrap();

    let input: Vec<&str> = lines.iter().take(split).cloned().collect();
    let instructions: Vec<&str> = lines.iter().skip(split + 1).cloned().collect();

    // transpose crates
    let crates: Vec<String> = (0..input[0].len())
        .map(|i| input.iter().rev().map(|inner| inner.chars().nth(i).unwrap()).collect::<String>()) // transpose matrix
        .filter(|w| w.chars().next().unwrap().is_ascii_digit()) // filter out any lines we do not like
        .map(|w| String::from(w.trim_end())) // map the result so that trailing whitespace is removed
        .collect();

    // now creates are in form of "1RGHQSBTN"

    let crates_after1 = follow_instructions(&instructions, crates.clone(), 9000);

    let answer1: String = crates_after1.iter().map(|w| w.chars().last().unwrap()).collect();

    println!("Answer to first half is {}", answer1);

    let crates_after2 = follow_instructions(&instructions, crates, 9001);

    let answer2: String = crates_after2.iter().map(|w| w.chars().last().unwrap()).collect();

    println!("Answer to second half is {}", answer2);
}