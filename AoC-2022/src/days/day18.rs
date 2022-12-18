use std::{fs, collections::{VecDeque, BTreeSet}};

use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq)]
enum FloodFill {
    Unknown, 
    Connected,
    NotConnected,
}

pub fn day18(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let mut coordinates: Vec<(usize, usize, usize)> = Vec::new(); 

    for line in content.split('\n') {
        let xyz: (usize, usize, usize) = line.split(',').map(|x| x.parse::<usize>().unwrap()).next_tuple().unwrap();

        coordinates.push(xyz);
    }

    let max = (
        coordinates.iter().map(|x| x.0).max().unwrap(),
        coordinates.iter().map(|x| x.1).max().unwrap(),
        coordinates.iter().map(|x| x.2).max().unwrap()
    );

    let mut space = vec![vec![vec![false; max.2 + 1]; max.1 + 1]; max.0 + 1];

    for i in coordinates {
        space[i.0][i.1][i.2] = true;
    }

    let mut surface = 0;
    for x in 0..max.0+1 {
        for y in 0..max.1+1 {
            for z in 0..max.2+1 {
                if !space[x][y][z] {
                    continue;
                }

                if x == 0 || !space[x-1][y][z] {
                    surface += 1;
                }

                if x == max.0 || !space[x+1][y][z] {
                    surface += 1;
                }

                if y == 0 || !space[x][y-1][z] {
                    surface += 1;
                }

                if y == max.1 || !space[x][y+1][z] {
                    surface += 1;
                }

                if z == 0 || !space[x][y][z-1] {
                    surface += 1;
                }

                if z == max.2 || !space[x][y][z+1] {
                    surface += 1;
                }
            }
        }
    }

    println!("First half is {}", surface);

    let mut connected_to_outside = vec![vec![vec![FloodFill::Unknown; max.2 + 1]; max.1 + 1]; max.0 + 1];

    for x in 0..max.0+1 {
        for y in 0..max.1+1 {
            for z in 0..max.2+1 {
                if space[x][y][z] || connected_to_outside[x][y][z] != FloodFill::Unknown {
                    continue;
                }

                // flood fill the holes
                let mut all: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
                let mut fifo: VecDeque<(usize, usize, usize)> = VecDeque::new();

                all.insert((x, y, z));
                fifo.push_back((x, y, z));

                let mut connected = false;
                while !fifo.is_empty() {
                    let (x, y, z) = fifo.pop_front().unwrap();

                    if connected_to_outside[x][y][z] != FloodFill::Unknown {
                        connected = connected_to_outside[x][y][z] == FloodFill::Connected;
                        break;
                    }

                    if x == 0 || y == 0 || z == 0 || x == max.0 || y == max.1 || z == max.2 {
                        // it is possible to get to ouside
                        connected = true;
                        break;
                    }

                    if !space[x-1][y][z] && all.insert((x-1, y, z)) {
                        fifo.push_back((x-1, y, z));
                    }

                    if !space[x+1][y][z] && all.insert((x+1, y, z)) {
                        fifo.push_back((x+1, y, z));
                    }

                    if !space[x][y-1][z] && all.insert((x, y-1, z)) {
                        fifo.push_back((x, y-1, z));
                    }

                    if !space[x][y+1][z] && all.insert((x, y+1, z)) {
                        fifo.push_back((x, y+1, z));
                    }

                    if !space[x][y][z-1] && all.insert((x, y, z-1)) {
                        fifo.push_back((x, y, z-1));
                    }

                    if !space[x][y][z+1] && all.insert((x, y, z+1)) {
                        fifo.push_back((x, y, z+1));
                    }
                }

                if connected {
                    for (x, y, z) in all {
                        connected_to_outside[x][y][z] = FloodFill::Connected;
                    }
                } else {
                    for (x, y, z) in all {
                        connected_to_outside[x][y][z] = FloodFill::NotConnected;
                    }
                }
            }
        }
    }

    let mut surface2 = 0;
    for x in 0..max.0+1 {
        for y in 0..max.1+1 {
            for z in 0..max.2+1 {
                if !space[x][y][z] {
                    continue;
                }

                if x == 0 || !space[x-1][y][z] && connected_to_outside[x-1][y][z] == FloodFill::Connected {
                    surface2 += 1;
                }

                if x == max.0 || !space[x+1][y][z] && connected_to_outside[x+1][y][z] == FloodFill::Connected {
                    surface2 += 1;
                }

                if y == 0 || !space[x][y-1][z] && connected_to_outside[x][y-1][z] == FloodFill::Connected {
                    surface2 += 1;
                }

                if y == max.1 || !space[x][y+1][z] && connected_to_outside[x][y+1][z] == FloodFill::Connected {
                    surface2 += 1;
                }

                if z == 0 || !space[x][y][z-1] && connected_to_outside[x][y][z-1] == FloodFill::Connected {
                    surface2 += 1;
                }

                if z == max.2 || !space[x][y][z+1] && connected_to_outside[x][y][z+1] == FloodFill::Connected {
                    surface2 += 1;
                }
            }
        }
    }

    println!("Second half is {}", surface2);
}
