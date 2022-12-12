use std::{fs, collections::HashSet, cmp::Ordering};

fn update_tail(head: (i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
        // check if touching
        if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
            return tail;
        }

        if head.0 == tail.0 { // same x coordinates
            tail.1 += match head.1.cmp(&tail.1) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0
            };
        }

        if head.1 == tail.1 { // same y coordinates
            tail.1 += match head.0.cmp(&tail.0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0
            };
        }

        // up right
        if head.0 > tail.0 && head.1 > tail.1 {
            tail.0 += 1;
            tail.1 += 1;
        } 

        // up left
        if head.0 < tail.0 && head.1 > tail.1 {
            tail.0 -= 1;
            tail.1 += 1;
        } 

        // down right
        if head.0 > tail.0 && head.1 < tail.1 {
            tail.0 += 1;
            tail.1 -= 1;
        } 

        // down left
        if head.0 < tail.0 && head.1 < tail.1 {
            tail.0 -= 1;
            tail.1 -= 1;
        }

        tail
}

fn first_half(commands: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    visited.insert(tail);
    for i in commands.chars() {
        head = match i {
            'U' => (head.0, head.1 + 1),
            'D' => (head.0, head.1 - 1),
            'L' => (head.0 - 1, head.1),
            'R' => (head.0 + 1, head.1),
            _ => panic!("invalid command")
        };

        tail = update_tail(head, tail);

        visited.insert(tail);
    } 

    visited.len()
}

fn second_half(commands: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut rope: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();

    visited.insert(*rope.last().unwrap());

    for i in commands.chars() {
        let head = rope.first_mut().unwrap();

        *head = match i {
            'U' => (head.0, head.1 + 1),
            'D' => (head.0, head.1 - 1),
            'L' => (head.0 - 1, head.1),
            'R' => (head.0 + 1, head.1),
            _ => panic!("invalid command")
        };

        for i in 1..rope.len() {
            rope[i] = update_tail(rope[i-1], rope[i]);
        }

        visited.insert(*rope.last().unwrap());
    } 

    visited.len()
}

pub fn day9(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines = content.split('\n').collect::<Vec<&str>>();

    let commands = lines.iter()
        .map(|w| w.split(' ').next().unwrap().chars().next().unwrap())
        .zip(lines.iter().map(|w| w.split(' ').nth(1).unwrap().parse::<usize>().unwrap()))
        .map(|(d, c)| (0..c).map(|_| d).collect::<String>())
        .fold(String::from(""), |a, b| a + &b );

    println!("First half is {}", first_half(&commands));

    println!("Second half is {}", second_half(&commands));
}