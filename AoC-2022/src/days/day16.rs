use std::{fs, collections::BTreeMap};
use regex::Regex;

#[derive(Clone)]
struct Valve {
    flow: i32,
    tunnels: Vec<usize>
}

fn bruteforce(valves: &Vec<Valve>, minute: i32, position: usize, previous: usize, open: Vec<i32>, start: usize, elephant_time: i32, best_so_far: &mut i32) -> i32 {
    if minute == 0 || open.iter().filter(|x| **x == 0).count() == 0 {
        if elephant_time > 0 {
            return bruteforce(valves, elephant_time, start, start, open, 0, 0, best_so_far);
        } 

        return open.iter()
            .enumerate()
            .map(|x| x.1 * valves[x.0].flow)
            .sum();
    }

    // let see what we can make
    let highest_possible: i32 = open.iter()
        .map(|x| if *x == 0 { std::cmp::max(minute, elephant_time) } else { *x })
        .enumerate()
        .map(|x| x.1 * valves[x.0].flow)
        .sum();

    if *best_so_far >= highest_possible {
        return highest_possible;
    }


    let mut max = 0;

    if position < open.len() && open[position] == 0 {
        // we can open something
        let mut open_copy = open.clone();
        open_copy[position] = minute - 1;
        max = bruteforce(valves, minute - 1, position, position, open_copy, start, elephant_time, best_so_far);
    }

    for tunnel in valves[position].tunnels.iter() {
        if *tunnel == previous {
            continue;
        }

        max = std::cmp::max(bruteforce(valves, minute - 1, *tunnel, position, open.clone(), start, elephant_time, best_so_far), max);
    }

    *best_so_far = std::cmp::max(max, *best_so_far);

    max
}


pub fn day16(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let parse = Regex::new(r#"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)"#).unwrap();

    let ids = {
        let mut result: BTreeMap<String, usize> = BTreeMap::new();

        let mut valves: Vec<(i32, String)> = vec![];
        for line in lines.iter() {
            let captures = parse.captures(line).unwrap();
    
            let id = &captures[1];
            let flow = captures[2].parse::<i32>().unwrap();

            valves.push((flow, String::from(id)));
        }

        valves.sort_by(|a, b| b.cmp(a));

        for (i, x) in valves.into_iter().map(|x| x.1).enumerate() {
            result.insert(x, i);
        }

        result
    };

    let mut valves: Vec<Valve> = vec![Valve { flow: 0, tunnels: vec![] }; ids.len()];

    for line in lines {
        let captures = parse.captures(line).unwrap();

        let id = &captures[1];
        let flow = captures[2].parse::<i32>().unwrap();
        let tunnels: Vec<usize> = captures[3].split(',').map(|w| *ids.get(w.trim()).unwrap()).collect();

        valves[*ids.get(id).unwrap()] = Valve { flow, tunnels };
    }

    let functioning_valves = valves.iter().map(|x| x.flow).filter(|x| *x > 0).count();
    let start: usize = *ids.get("AA").unwrap();

    let open_valves: Vec<i32> = vec![0; functioning_valves];

    let first_half = bruteforce(&valves, 30, start, start, open_valves.clone(), start, 0, &mut 0);

    println!("First half {}", first_half);

    let second_half = bruteforce(&valves, 26, start, start, open_valves, start, 26, &mut 0);

    println!("Second half {}", second_half);
}
