use std::fs;
use itertools::Itertools;

/*
fn print_cave(cave: &Vec<Vec<char>>) {
    for i in cave {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}
*/

pub fn simulate_sand(mut cave: Vec<Vec<char>>, source: (usize, usize)) -> i32 {
    let mut round = 0;
    'rounds: loop {
        let mut sand: (usize, usize) = source;

        loop {
            // we are finished
            if sand.1 == cave.len() - 1 {
                break 'rounds;
            }

            if cave[sand.1 + 1][sand.0] == '.' {
                sand.1 += 1;
                continue;
            }

            if sand.0 == 0 {
                break 'rounds;
            }

            if cave[sand.1 + 1][sand.0 - 1] == '.' {
                sand.1 += 1;
                sand.0 -= 1;
                continue;
            }

            if sand.0 == cave[sand.1].len() - 1 {
                break 'rounds;
            }

            if cave[sand.1 + 1][sand.0 + 1] == '.' {
                sand.1 += 1;
                sand.0 += 1;
                continue;
            }

            break;
        }

        cave[sand.1][sand.0] = 'o';
            
        round += 1;

        if sand == source {
            break;
        } 
    }

    round
}

pub fn day14(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let shapes: Vec<Vec<(i32, i32)>> = 
        lines.iter()
            .map(|&x| x.split(" -> ")
                .map(|y| {
                    let bounds: (&str, &str)  = y.split(',').next_tuple().unwrap();

                    (bounds.0.parse::<i32>().unwrap(), bounds.1.parse::<i32>().unwrap())
                })
                .into_iter()
                .collect::<Vec<(i32, i32)>>())
            .collect();

    let y = {
        let mut result = i32::MIN;

        for x in &shapes {
            let max = match x.iter().map(|a| a.1).max() {
                None => panic!("empty shape?"),
                Some(x) => x,
            };

            result = std::cmp::max(result, max);
        }

        result + 2 // +2 for second half
    };

    let x = (500 - y, 500 + y); // cannot be more than this pyramid


    let mut cave: Vec<Vec<char>> = vec![vec![]; y as usize + 1];
    for i in &mut cave {
        i.append(&mut vec!['.'; (x.1 - x.0 + 1) as usize]);
    }

    for shape in shapes {
        for (a, b) in shape.iter().tuple_windows() {
            if a.0 == b.0 {
                let min = std::cmp::min(a.1, b.1);
                let max = std::cmp::max(a.1, b.1);

                for i in min..=max {
                    cave[i as usize][(a.0 - x.0) as usize] = '#';
                }
            }

            if a.1 == b.1 {
                let min = std::cmp::min(a.0 - x.0, b.0 - x.0);
                let max = std::cmp::max(a.0 - x.0, b.0 - x.0);

                for i in min..=max {
                    cave[a.1 as usize][i as usize] = '#';
                }
            }
        }
    }

    let round = simulate_sand(cave.clone(), ((500 - x.0) as usize, 0));

    println!("First half is {}", round);

    for i in &mut cave.iter_mut().last() {
        for j in i.iter_mut() {
            *j = '#'; 
        }
    }

    let round2 = simulate_sand(cave.clone(), ((500 - x.0) as usize, 0));

    println!("Second half is {}", round2);
}