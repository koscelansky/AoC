use std::fs;

pub fn day20(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let vec: Vec<(usize, i64)> = content.split('\n').map(|x| x.trim().parse::<i64>().unwrap()).enumerate().collect();

    let mut answer = vec.clone();

    let length = answer.len();
    for (i, x) in vec.iter() {
        let value = x % (length - 1) as i64;
        let mut pos = answer.iter().position(|(idx, _)| idx == i).unwrap();

        match value.cmp(&0) {
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Less => {
                for _ in 0..value.abs() {
                    if pos == 0 {
                        answer.swap(pos, length - 1);
                        pos = length - 1;
                    } else {
                        answer.swap(pos, pos - 1);
                        pos -= 1;
                    }
                }                
            },
            std::cmp::Ordering::Greater => {
                for _ in 0..value.abs() {
                    if pos == length - 1 {
                        answer.swap(pos, 0);
                        pos = 0;
                    } else {
                        answer.swap(pos, pos + 1);
                        pos += 1;
                    }
                }
            }
        }

    }

    let zero_pos = answer.iter().cycle().position(|x| x.1 == 0).unwrap();
    let first_half = answer.iter().cycle().nth(zero_pos + 1000).unwrap().1
        + answer.iter().cycle().nth(zero_pos + 2000).unwrap().1
        + answer.iter().cycle().nth(zero_pos + 3000).unwrap().1;

    println!("First half {}", first_half);

    // part two
    let vec2 = vec.iter().map(|(i, x)| (*i, x * 811589153)).collect::<Vec<(usize, i64)>>();

    let mut answer2 = vec2.clone();
    for _ in 0..10 {
        for (i, x) in vec2.iter() {
            let mut pos = answer2.iter().position(|(idx, _)| idx == i).unwrap();
            let value = x % (length - 1) as i64;
    
            match value.cmp(&0) {
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Less => {
                    for _ in 0..value.abs() {
                        if pos == 0 {
                            answer2.swap(pos, length - 1);
                            pos = length - 1;
                        } else {
                            answer2.swap(pos, pos - 1);
                            pos -= 1;
                        }
                    }                
                },
                std::cmp::Ordering::Greater => {
                    for _ in 0..value.abs() {
                        if pos == length - 1 {
                            answer2.swap(pos, 0);
                            pos = 0;
                        } else {
                            answer2.swap(pos, pos + 1);
                            pos += 1;
                        }
                    }
                }
            }
        }
    }

    let zero_pos2 = answer2.iter().cycle().position(|x| x.1 == 0).unwrap();
    let second_half = answer2.iter().cycle().nth(zero_pos2 + 1000).unwrap().1
        + answer2.iter().cycle().nth(zero_pos2 + 2000).unwrap().1
        + answer2.iter().cycle().nth(zero_pos2 + 3000).unwrap().1;
    
    println!("Second half {}", second_half);

}
