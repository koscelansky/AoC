use std::fs;
use itertools::Itertools;

fn priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 'A' as usize + 27
    } else {
        c as usize - 'a' as usize + 1
    }
}

fn badges(input: &[&str]) -> usize {
    let backpacks: Vec<&str> = input.iter().filter(|&x| !x.is_empty()).cloned().collect();
    if backpacks.len() % 3 != 0 {
        panic!("Elves should be in group of 3")
    }

    let mut sum = 0;
    for (a, b, c) in backpacks.iter().tuples() {
        for i in a.chars() {
            if b.contains(i) && c.contains(i) {
                sum += priority(i);
                break;
            }
        }
    }

    sum
}

pub fn day3(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let rucksacks: Vec<&str> = content.split('\n').collect();

    let mut sum: usize = 0;
    for rucksack in rucksacks.iter() {
        if rucksack.len() % 2 == 1 {
            panic!("Rucksacks should be even")
        }
    
        let prefix = &rucksack[..rucksack.len() / 2];
        let suffix = &rucksack[rucksack.len() / 2..];
    
        for i in prefix.chars() {
            if suffix.contains(i) {
                sum += priority(i);
                break;
            }
        }
    }

    println!("Answer to 1. half is {sum}");

    println!("Answer to 2. half is {}", badges(&rucksacks))
}