use std::fs;

fn unique_window(content: &String, from: usize, size: usize) -> bool {
    let mut window: Vec<char> = content[from..from+size].chars().collect();

    window.sort();
    window.dedup();

    window.len() == size
}

pub fn day6(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    for i in 0..content.len()-3 {
        if unique_window(&content, i, 4) {
            println!("first half is {}", i + 4);
            break;
        }
    }
    
    for i in 0..content.len()-3 {
        if unique_window(&content, i, 14) {
            println!("first half is {}", i + 14);
            break;
        }
    }
}