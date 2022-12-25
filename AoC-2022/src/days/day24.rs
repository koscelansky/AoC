use std::cmp::Ordering;
use std::{fs, collections::HashSet, cmp};
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South, 
    West,
    East,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32, i32),
}

// make it max heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn gcd(a: usize, b: usize) -> usize{
    if b == 0 {
        return a;
    }

    match a.cmp(&b) {
        cmp::Ordering::Equal => a,
        cmp::Ordering::Less => gcd(b, a),
        cmp::Ordering::Greater => gcd(b, a % b),
    }
}

fn step(blizzards: &HashSet<(i32, i32, Direction)>, width: i32, height: i32) -> HashSet<(i32, i32, Direction)> {
    let mut result: HashSet<(i32, i32, Direction)> = HashSet::new();

    for (x, y, dir) in blizzards {
        let next = match &dir {
            Direction::North => {
                if *y > 0 {
                    (*x, y - 1)
                } else {
                    (*x, height - 1)
                }
            },
            Direction::South => {
                if *y < height - 1 {
                    (*x, y + 1)
                } else {
                    (*x, 0)
                }
            },
            Direction::West => {
                if *x > 0 {
                    (x - 1, *y)
                } else {
                    (width - 1, *y)
                }
            },
            Direction::East => {
                if *x < width - 1 {
                    (x + 1, *y)
                } else {
                    (0, *y)
                }
            },
        };

        result.insert((next.0, next.1, *dir));
    }

    result
}

fn print(blizzards: &HashSet<(i32, i32)>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            if blizzards.iter().filter(|(a, b)| *a == x && *b == y).count() > 0 {
                print!("*");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn dijkstra(blizzards: &Vec<HashSet<(i32, i32)>>, width: i32, height: i32, start: (i32, i32), finish: (i32, i32)) -> i32 {
    let mut distances: HashMap<(i32, i32, i32), i32> = HashMap::new();

    let mut open_set: BinaryHeap<State> = BinaryHeap::new();

    open_set.push(State{ cost: 0, position: (start.0, start.1, 0) });
    distances.insert((start.0, start.1, 0), 0);

    while let Some(State { cost, position }) = open_set.pop() {
        if position.0 == finish.0 && position.1 == finish.1 {
            return cost;
        }

        if distances.get(&position).unwrap_or(&i32::MAX) < &cost {
            continue;
        }

        let next_blizzard_id = (position.2 + 1) % blizzards.len() as i32;
        let next_blizzard = &blizzards[next_blizzard_id as usize];

        // wait
        if !next_blizzard.contains(&(position.0, position.1)) && distances.get(&(position.0, position.1, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
            let next_postion = (position.0, position.1, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });
        }

        // initial position
        if position.0 == 0 && position.1 == -1 {
            if !next_blizzard.contains(&(0, 0)) && distances.get(&(0, 0, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
                let next_postion = (0, 0, next_blizzard_id);
                
                distances.insert(next_postion, cost + 1);
    
                open_set.push(State { cost: cost + 1, position: next_postion });   
            }

            continue;   
        } 

        if position.0 == 0 && position.1 == 0 {
            let next_postion = (0, -1, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });             
        }

        // final position
        if position.0 == width - 1 && position.1 == height {
            if !next_blizzard.contains(&(width - 1, height - 1)) && distances.get(&(width - 1, height - 1, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
                let next_postion = (width - 1, height - 1, next_blizzard_id);
                
                distances.insert(next_postion, cost + 1);
    
                open_set.push(State { cost: cost + 1, position: next_postion });
            }

            continue;   
        } 

        if position.0 == width - 1 && position.1 == height - 1 {
            let next_postion = (width - 1, height, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });             
        }


        if position.0 > 0 && !next_blizzard.contains(&(position.0 - 1, position.1)) && distances.get(&(position.0 - 1, position.1, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
            let next_postion = (position.0 - 1, position.1, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });
        }

        if position.0 < width - 1 && !next_blizzard.contains(&(position.0 + 1, position.1)) && distances.get(&(position.0 + 1, position.1, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
            let next_postion = (position.0 + 1, position.1, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });
        }

        if position.1 > 0 && !next_blizzard.contains(&(position.0, position.1 - 1)) && distances.get(&(position.0, position.1 - 1, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
            let next_postion = (position.0, position.1 - 1, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });
        }

        if position.1 < height - 1 && !next_blizzard.contains(&(position.0, position.1 + 1)) && distances.get(&(position.0, position.1 + 1, next_blizzard_id)).unwrap_or(&i32::MAX) > &(cost + 1) {
            let next_postion = (position.0, position.1 + 1, next_blizzard_id);
            
            distances.insert(next_postion, cost + 1);

            open_set.push(State { cost: cost + 1, position: next_postion });
        }

    }

    panic!("no asnwer?");
}

pub fn day24(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines = content.split('\n').map(|x| x.trim()).collect::<Vec<&str>>();

    let width = (lines[0].len() - 2) as i32;
    let height = (lines.len() - 2) as i32;

    let mut blizzards: HashSet<(i32, i32, Direction)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match &c {
                '<' => blizzards.insert(((x - 1) as i32, (y - 1) as i32, Direction::West)),
                '>' => blizzards.insert(((x - 1) as i32, (y - 1) as i32, Direction::East)),
                '^' => blizzards.insert(((x - 1) as i32, (y - 1) as i32, Direction::North)),
                'v' => blizzards.insert(((x - 1) as i32, (y - 1) as i32, Direction::South)),
                _ => continue
            };
        }
    }

    let period = width * height / gcd(width as usize, height as usize) as i32;

    let mut blizzards_all: Vec<HashSet<(i32, i32)>> = Vec::new();
    for _ in 0..period {
        blizzards_all.push(blizzards.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<(i32, i32)>>());

        blizzards = step(&blizzards, width, height);

    }

    let first_half = dijkstra(&blizzards_all, width, height, (0, -1), (width - 1, height));

    println!("first half {}", first_half);

    let blizzards_all_len = blizzards_all.len() as i32;
    blizzards_all.rotate_left((first_half % blizzards_all_len) as usize);

    let back_for_snack = dijkstra(&blizzards_all, width, height, (width - 1, height), (0, -1));

    blizzards_all.rotate_left((back_for_snack % blizzards_all_len) as usize);

    let back_again = dijkstra(&blizzards_all, width, height, (0, -1), (width - 1, height));

    println!("Second half {}", first_half + back_for_snack + back_again);
}
