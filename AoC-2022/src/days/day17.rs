use std::fs;

#[derive(Copy, Clone)]
enum Rocks {
    Line, 
    Cross,
    InverseL,
    I,
    Box,
}

#[derive(Copy, Clone)]
enum Direction {
    Left, 
    Rigth,
}

fn horizontal_size(rock: Rocks) -> usize {
    match rock {
        Rocks::Line => 4,
        Rocks::Cross => 3,
        Rocks::InverseL => 3,
        Rocks::I => 1,
        Rocks::Box => 2,
    }   
}

fn get_rock_squares(rock: Rocks, position: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = position;
    
    match rock {
        Rocks::Line => vec![(x, y), (x+1, y), (x+2, y), (x+3, y)],
        Rocks::Cross => vec![(x+1, y), (x, y+1), (x+1, y+1), (x+2, y+1), (x+1, y+2)],
        Rocks::InverseL => vec![(x, y), (x+1, y), (x+2, y), (x+2, y+1), (x+2, y+2)],
        Rocks::I => vec![(x, y), (x, y+1), (x, y+2), (x, y+3)],
        Rocks::Box => vec![(x, y), (x, y+1), (x+1, y), (x+1, y+1)],
    }    
}

fn is_occupied(cave: &[Vec<bool>], position: (usize, usize)) -> bool {
    position.1 < cave.len() && cave[position.1][position.0]
}

fn apply_push(cave: &[Vec<bool>], rock: Rocks, position: (usize, usize), direction: Direction) -> (usize, usize) {
    assert!(position.0 < 7);

    let (x, y) = position;

    let size = horizontal_size(rock);

    match direction {
        Direction::Left => {
            if position.0 > 0 && get_rock_squares(rock, (x - 1, y)).iter().map(|x| is_occupied(cave, *x)).all(|x| !x) {
                (x - 1, y)
            } else {
                (x, y)
            }
        },
        Direction::Rigth => {
            if position.0 + size < 7 && get_rock_squares(rock, (x + 1, y)).iter().map(|x| is_occupied(cave, *x)).all(|x| !x) {
                (position.0 + 1, position.1)
            } else {
                position
            }
        }
    }
}

// position is bound box left bottom
fn is_collision(cave: &[Vec<bool>], rock: Rocks, position: (usize, usize)) -> bool {
    if position.1 == 0 {
        return true;
    }

    let squares = get_rock_squares(rock, (position.0, position.1 - 1));

    squares.iter().map(|x| is_occupied(cave, *x)).any(|x| x)
}

fn update_cave(mut cave: Vec<Vec<bool>>, rock: Rocks, position: (usize, usize)) -> Vec<Vec<bool>> {

    let squares = get_rock_squares(rock, position);

    for (x, y) in squares {
        while y >= cave.len() {
            cave.push(vec![false; 7])
        }

        assert!(!cave[y][x]);

        cave[y][x] = true;
    }

    cave
}

fn simulate(content: &String, steps: usize) -> Vec<Vec<bool>> {
    let rocks = vec![Rocks::Line, Rocks::Cross, Rocks::InverseL, Rocks::I, Rocks::Box];

    let mut rocks_iter = rocks.iter().cycle();
    let mut jets_iter = content.chars().cycle();

    let mut cave = vec![vec![false; 7]; 0];

    for _ in 0..steps {
        let rock = *rocks_iter.next().unwrap();
        let heighest = cave.len();

        let mut position = (2_usize, heighest + 3);

        let mut step = 0_usize;

        loop {
            if step % 2 == 0 {
                let jet = jets_iter.next().unwrap();

                // push step
                position = match jet {
                    '<' => apply_push(&cave, rock, position, Direction::Left),
                    '>' => apply_push(&cave, rock, position, Direction::Rigth),
                    _ => panic!("unknown jet")
                };
            } else {
                // gravity step :)
                if is_collision(&cave, rock, position) {
                    // update heights
                    cave = update_cave(cave, rock, position);

                    break;
                } else {
                    position = (position.0, position.1 - 1);
                }
            }

            step += 1;
        }
    }

    cave
}


fn get_cycle(content: &String) -> (usize, usize) {
    let rocks = vec![Rocks::Line, Rocks::Cross, Rocks::InverseL, Rocks::I, Rocks::Box];

    let mut result = (0_usize, 0_usize);

    let mut rocks_iter = rocks.iter().cycle();
    let mut jets_iter = content.chars().cycle();

    let mut cave = vec![vec![false; 7]; 0];

    let mut rock_starts = vec![(0_usize, 0_usize); 0];

    let mut jet_id = 0_usize;
    let mut rock_id = 0_usize;

    let mut cycle_window: Vec<(usize, usize)> = Vec::new();
    loop {
        let mut step = 0_usize;

        // this next part should determine cycle in simulation, you may notice the 
        // arbitrary 5, i cannot really understand how to solve this without some
        // guesswork, what I do not understand it why later pieces cannot drop
        // below whatever limit we propose. The structure is however complicated
        // so I guess there is a proof by contradition to the statement, this 
        // seems to work so we stick with it, but I cannot guaranteed that there 
        // is no input where it will break :(
        let window_size = 5_usize;

        rock_starts.push((rock_id % rocks.len(), jet_id % content.len()));
        if rock_starts.len() > window_size {
            let last = &rock_starts[rock_starts.len()-window_size..];
            
            if result.0 == 0 {
                if rock_starts[..rock_starts.len()-window_size].windows(last.len()).any(|window| window == last) {
                    result.0 = rock_id - window_size;
                    cycle_window = Vec::from(last);
                }
            } else if result.1 == 0 && cycle_window == last {
                result.1 = rock_id - window_size - result.0;
                return result;
            }
        }

        let rock = *rocks_iter.next().unwrap();
        let heighest = cave.len();

        let mut position = (2_usize, heighest + 3);

        loop {
            if step % 2 == 0 {
                let jet = jets_iter.next().unwrap();
                jet_id += 1;
                // push step
                position = match jet {
                    '<' => apply_push(&cave, rock, position, Direction::Left),
                    '>' => apply_push(&cave, rock, position, Direction::Rigth),
                    _ => panic!("unknown jet")
                };
            } else {
                // gravity step :)
                if is_collision(&cave, rock, position) {
                    // update heights
                    cave = update_cave(cave, rock, position);

                    break;
                } else {
                    position = (position.0, position.1 - 1);
                }
            }

            step += 1;
        }

        rock_id += 1;
    }
}

pub fn day17(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let first_half = simulate(&content, 2022);

    println!("First half {}", first_half.len());

    let (first, step) = get_cycle(&content);
    println!("First {} step {}", first, step);

    let first_height = simulate(&content, first).len();

    let step_height = simulate(&content, first + step).len() - first_height;

    let steps = (1000000000000_usize - first) / step;

    let last = (1000000000000_usize - first) % step;

    let last_height = simulate(&content, first + last).len() - first_height;

    println!("Second part {}", first_height + steps * step_height + last_height);
}
