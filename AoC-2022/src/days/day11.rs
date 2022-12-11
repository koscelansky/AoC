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

pub fn day11(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines = content.split('\n');
}