use std::fs;
use regex::Regex;

struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

fn manhattan_distance(x: (i32, i32), y: (i32, i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn distance_to_line(x: (i32, i32), y: i32) -> i32 {
    (x.1 - y).abs()
}

fn intervals_for_line(sensors: &Vec<Sensor>, y: i32) -> Vec<(i32, i32)> {
    let mut intervals: Vec<(i32, i32)> = vec![];

    for sensor in sensors {
        let distance = manhattan_distance(sensor.beacon, sensor.position);
        let offset = distance_to_line(sensor.position, y);

        if distance >= offset {
            let x_offset = distance - offset;

            intervals.push((sensor.position.0 - x_offset, sensor.position.0 + x_offset));
        }
    }

    intervals
}

fn merge_intervals(intervals: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut merged: Vec<(i32, i32)> = vec![];

    let mut curr: Option<(i32, i32)> = None;
    for i in intervals {
        curr = match curr {
            None => Some(*i),
            Some(x) => {
                if x.1 + 1 >= i.0 {
                    Some((x.0, std::cmp::max(x.1, i.1)))
                } else {
                    merged.push(x);
                    Some(*i)
                }
            }
        }
    }

    if let Some(x) = curr {
        merged.push(x)
    }

    merged
}

pub fn day15(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let parse = Regex::new(r#"Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)"#).unwrap();

    let mut sensors: Vec<Sensor> = vec![];

    for line in lines {
        let captures = parse.captures(line).unwrap();

        let sensor = (captures[1].parse::<i32>().unwrap(), captures[2].parse::<i32>().unwrap());
        let beacon = (captures[3].parse::<i32>().unwrap(), captures[4].parse::<i32>().unwrap());

        sensors.push(Sensor { position: sensor, beacon });
    }

    sensors.sort_by(|a, b| a.position.0.cmp(&b.position.0));

    let y = 2000000;

    let intervals = intervals_for_line(&sensors, y);
    
    let merged = merge_intervals(&intervals);

    let sum: i32 = merged.iter().map(|(a, b)| (a - b).abs()).sum();

    println!("First half is {}", sum);

    for i in 0..=4000000 {
        let mut intervals = intervals_for_line(&sensors, i);

        intervals.sort_by(|a, b| a.0.cmp(&b.0));

        let merged = merge_intervals(&intervals);
        
        if merged.len() == 2 {
            let second = ((merged[0].1 + 1) as i64) * 4000000 + (i as i64);
            println!("Second half is {}", second);
            break;
        }
    }
}
