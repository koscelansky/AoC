use std::fs;
use regex::Regex;

fn draw_crt(cycles: &[i32]) {
    let mut screen = String::new();

    for i in 0..6 {
        for j in 0..40 {
            if (cycles[i * 40 + j] - j as i32).abs() <= 1 {
                screen.push('#')
            } else {
                screen.push('.')
            }
        }

        screen.push('\n');
    }

    println!("{}", screen);
}

pub fn day10(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let mut reg = 1;

    let parse: Regex = Regex::new(r#"(\w+)\s*(\-?\w*)"#).unwrap();

    let mut values: Vec<i32> = Vec::new();

    for i in content.split('\n') {
        let captures = parse.captures(i).unwrap();

        let cmd = &captures[1];
        let param = &captures[2].parse::<i32>().unwrap_or(0);

        match cmd {
            "noop" => {
                values.push(reg);
            },
            "addx" => {
                values.push(reg);
                values.push(reg);
                reg += param;
            },
            _ => panic!("invalid instruction")
        }
    }

    let signal_strength: i32 = (20..values.len()).step_by(40).map(|i| values[i-1] * i as i32).sum();

    println!("First hald {}", signal_strength);

    println!("Below is crt for second half");

    draw_crt(&values[..])
}