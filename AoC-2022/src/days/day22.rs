use std::{fs, collections::HashMap};

#[derive(Clone)]
enum SquareType {
    Empty,
    Wall,
    Warp,
}

type ApplyMove = fn(&Vec<Vec<SquareType>>, (usize, usize), i32, usize) -> (usize, usize);

fn apply_move(maze: &Vec<Vec<SquareType>>, mut position: (usize, usize), angle: i32, distance: usize) -> (usize, usize) {
    match &angle {
        0 => {
            for _ in 0..distance {
                match &maze[position.0][position.1 + 1] {
                    SquareType::Empty => position.1 += 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze[position.0].iter().position(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[position.0][warp_pos], SquareType::Empty) {
                            position.1 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        180 => {
            for _ in 0..distance {
                match &maze[position.0][position.1 - 1] {
                    SquareType::Empty => position.1 -= 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze[position.0].iter().rposition(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[position.0][warp_pos], SquareType::Empty) {
                            position.1 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        270 => {
            for _ in 0..distance {
                match &maze[position.0 - 1][position.1] {
                    SquareType::Empty => position.0 -= 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze.iter().map(|x| &x[position.1]).rposition(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[warp_pos][position.1], SquareType::Empty) {
                            position.0 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        90 => {
            for _ in 0..distance {
                match &maze[position.0 + 1][position.1] {
                    SquareType::Empty => position.0 += 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze.iter().map(|x| &x[position.1]).position(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[warp_pos][position.1], SquareType::Empty) {
                            position.0 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        _ => panic!("should not be here")
    };

    position
}

fn apply_move_cube(maze: &Vec<Vec<SquareType>>, mut position: (usize, usize), angle: i32, distance: usize) -> (usize, usize) {
    match &angle {
        0 => {
            for _ in 0..distance {
                match &maze[position.0][position.1 + 1] {
                    SquareType::Empty => position.1 += 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze[position.0].iter().position(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[position.0][warp_pos], SquareType::Empty) {
                            position.1 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        180 => {
            for _ in 0..distance {
                match &maze[position.0][position.1 - 1] {
                    SquareType::Empty => position.1 -= 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze[position.0].iter().rposition(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[position.0][warp_pos], SquareType::Empty) {
                            position.1 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        270 => {
            for _ in 0..distance {
                match &maze[position.0 - 1][position.1] {
                    SquareType::Empty => position.0 -= 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze.iter().map(|x| &x[position.1]).rposition(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[warp_pos][position.1], SquareType::Empty) {
                            position.0 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        90 => {
            for _ in 0..distance {
                match &maze[position.0 + 1][position.1] {
                    SquareType::Empty => position.0 += 1,
                    SquareType::Wall => break,
                    SquareType::Warp => {
                        let warp_pos = maze.iter().map(|x| &x[position.1]).position(|x| !matches!(x, SquareType::Warp)).unwrap();
                        if matches!(&maze[warp_pos][position.1], SquareType::Empty) {
                            position.0 = warp_pos;
                        } else {
                            break;
                        }
                    }
                }
            }
        },
        _ => panic!("should not be here")
    };

    position
}

fn walk(maze: &Vec<Vec<SquareType>>, mut instructions: &str, next: ApplyMove) -> usize {
    let mut current_position = (1_usize, maze[1].iter().position(|x| matches!(x, SquareType::Empty)).unwrap());
    let mut current_angle: i32 = 0; // 0 is to right, 90 down...

    let mut pos = instructions.chars().position(|x| x.is_ascii_uppercase());
    while let Some(x) = pos {
        let distance = instructions[..x].parse::<usize>().unwrap();
        let direction = instructions.chars().nth(x).unwrap();

        instructions = &instructions[x+1..];

        current_position = next(maze, current_position, current_angle, distance);
        

        //update direction
        match &direction {
            'L' => current_angle -= 90,
            'R' => current_angle += 90,
            _ => panic!("unknow direction")
        }

        current_angle = current_angle.rem_euclid(360_i32);

        pos = instructions.chars().position(|x| x.is_ascii_uppercase());
    }

    current_position = apply_move(&maze, current_position, current_angle, instructions.parse::<usize>().unwrap_or(0));

    current_position.0 * 1000 + current_position.1 * 4 + (current_angle / 90) as usize
}

pub fn day22(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').map(|x| x.trim_end()).collect();

    let task: Vec<&[&str]> = lines.split(|x| x.is_empty()).collect();

    let place = task[0].to_vec();

    let max_line = place.iter().map(|x| x.len()).max().unwrap() + 2;

    let mut maze: Vec<Vec<SquareType>> = vec![vec![SquareType::Warp; max_line]; place.len() + 2]; // true is wall
    for (i, line) in place.iter().enumerate() {
        for (j, x) in line.chars().enumerate() {
            maze[i + 1][j + 1] = match &x {
                '.' => SquareType::Empty,
                '#' => SquareType::Wall,
                _ => SquareType::Warp
            };
        }
    }

    let instructions = *task[1].iter().next().unwrap();

    let first_half = walk(&maze, instructions, apply_move);

    println!("First half {}", first_half);

    let cube_size: usize = ((maze.iter()
        .map(|x| x.iter().filter(|y| !matches!(y, SquareType::Warp)).count())
        .sum::<usize>() / 6) as f64).sqrt() as usize;

    println!("{}", cube_size);
}
