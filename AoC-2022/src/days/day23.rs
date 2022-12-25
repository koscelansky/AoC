use std::{fs, collections::HashSet};
use itertools::Itertools;
use hashbag::HashBag;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South, 
    West,
    East,
}

fn is_northish_vacant(elves: &HashSet<(i32, i32)>, elf: (i32, i32)) -> bool {
    !elves.contains(&(elf.0 - 1, elf.1 - 1)) && !elves.contains(&(elf.0, elf.1 - 1)) && !elves.contains(&(elf.0 + 1, elf.1 - 1))
}

fn is_southish_vacant(elves: &HashSet<(i32, i32)>, elf: (i32, i32)) -> bool {
    !elves.contains(&(elf.0 - 1, elf.1 + 1)) && !elves.contains(&(elf.0, elf.1 + 1)) && !elves.contains(&(elf.0 + 1, elf.1 + 1))
}

fn is_westish_vacant(elves: &HashSet<(i32, i32)>, elf: (i32, i32)) -> bool {
    !elves.contains(&(elf.0 - 1, elf.1 - 1)) && !elves.contains(&(elf.0 - 1, elf.1)) && !elves.contains(&(elf.0 - 1, elf.1 + 1))
}

fn is_eastish_vacant(elves: &HashSet<(i32, i32)>, elf: (i32, i32)) -> bool {
    !elves.contains(&(elf.0 + 1, elf.1 - 1)) && !elves.contains(&(elf.0 + 1, elf.1)) && !elves.contains(&(elf.0 + 1, elf.1 + 1))
}

fn is_all_vacant(elves: &HashSet<(i32, i32)>, elf: (i32, i32)) -> bool {
    is_northish_vacant(elves, elf) && is_southish_vacant(elves, elf) && is_westish_vacant(elves, elf) && is_eastish_vacant(elves, elf)
}

fn proposed_move(elves: &HashSet<(i32, i32)>, elf: (i32, i32), directions: &[Direction]) -> Option<(i32, i32)> {
    for dir in directions {
        match dir {
            Direction::North => if is_northish_vacant(elves, elf) {
                return Some((elf.0, elf.1 - 1));
            },
            Direction::South => if is_southish_vacant(elves, elf) {
                return Some((elf.0, elf.1 + 1));
            },
            Direction::West => if is_westish_vacant(elves, elf) {
                return Some((elf.0 - 1, elf.1));
            },
            Direction::East => if is_eastish_vacant(elves, elf) {
                return Some((elf.0 + 1, elf.1));
            },                
        }
    }

    None
}

pub fn day23(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines = content.split('\n');

    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        } 
    }

    let mut directions = vec![Direction::North, Direction::South, Direction::West, Direction::East];

    let mut round = 0;
    loop {
        let mut proposed_moves: HashBag<(i32, i32)> = HashBag::new();

        for elf in &elves {
            if is_all_vacant(&elves, *elf) {
                continue;
            }
    
            if let Some(x) = proposed_move(&elves, *elf, &directions) {
                proposed_moves.insert(x);
            }           
        }

        if proposed_moves.is_empty() {
            println!("Second half {}", round + 1);
            break;
        }

        let mut new_elves: HashSet<(i32, i32)> = HashSet::new();

        for elf in &elves {
            if is_all_vacant(&elves, *elf) {
                new_elves.insert(*elf);
                continue;
            }
    
            match &proposed_move(&elves, *elf, &directions) {
                Some(x) => {
                    if proposed_moves.contains(x) == 1 {
                        new_elves.insert(*x);
                    } else {
                        new_elves.insert(*elf);
                    }
                },
                None => {
                    new_elves.insert(*elf);
                },
            }


        }

        elves = new_elves;

        directions.rotate_left(1);

        round += 1;

        if round == 10 {
            let width = match &elves.iter().map(|x| x.0).minmax() {
                itertools::MinMaxResult::NoElements => panic!("we should have some elves"),
                itertools::MinMaxResult::OneElement(_) => 1,
                itertools::MinMaxResult::MinMax(a, b) => b - a + 1
            };
        
            let height = match &elves.iter().map(|x| x.1).minmax() {
                itertools::MinMaxResult::NoElements => panic!("we should have some elves"),
                itertools::MinMaxResult::OneElement(_) => 1,
                itertools::MinMaxResult::MinMax(a, b) => b - a + 1
            };
        
            let first_half = height * width - elves.len() as i32;
        
            println!("First half {}", first_half);
        }
    }

}
