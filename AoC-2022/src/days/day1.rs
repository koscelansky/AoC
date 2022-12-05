use std::fs;

pub fn day1(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let mut elves: Vec<u32> = Vec::new();
    let mut curr: u32 = 0;
    for line in lines.into_iter() {
        if line.is_empty() {
            elves.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<u32>().unwrap();
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!("1. half answer is {}", elves[0]);

    let top = &elves[..3];
    println!("2. half answer is {}", top.iter().sum::<u32>());
}