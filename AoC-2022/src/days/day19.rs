use std::fs;
use regex::Regex;
use rayon::prelude::*;

struct Blueprint {
    robot: (u32, u32, u32),
    clay: (u32, u32, u32),
    obsidian: (u32, u32, u32),
    geode: (u32, u32, u32),
}

fn mine_materials(materials: (u32, u32, u32, u32), robots: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    (materials.0 + robots.0, materials.1 + robots.1, materials.2 + robots.2, materials.3 + robots.3)
}

fn pay_for_robot(materials: (u32, u32, u32, u32), robot: (u32, u32, u32)) -> (u32, u32, u32, u32) {
    (materials.0 - robot.0, materials.1 - robot.1, materials.2 - robot.2, materials.3)
}

fn bruteforce(blueprint: &Blueprint, minute: u32, materials: (u32, u32, u32, u32), robots: (u32, u32, u32, u32), max_so_far: &mut u32) -> u32 {
    if minute == 0 {
        return materials.3;
    }

    if materials.3 + robots.3 * minute + (minute) * (minute - 1) < *max_so_far {
        // cutoff
        return *max_so_far;
    }

    let mut max = 0_u32;

    let materials_next = mine_materials(materials, robots);

    // do not create any bot
    max = std::cmp::max(max, bruteforce(blueprint, minute - 1, materials_next, robots, max_so_far));

    if materials.0 >= blueprint.geode.0 && materials.1 >= blueprint.geode.1 && materials.2 >= blueprint.geode.2 {
        let mut robots_next = robots;
        robots_next.3 += 1;

        max = std::cmp::max(max, bruteforce(blueprint, minute - 1, pay_for_robot(materials_next, blueprint.geode), robots_next, max_so_far));
    }

    if materials.0 >= blueprint.obsidian.0 && materials.1 >= blueprint.obsidian.1 && materials.2 >= blueprint.obsidian.2 {
        let mut robots_next = robots;
        robots_next.2 += 1;

        max = std::cmp::max(max, bruteforce(blueprint, minute - 1, pay_for_robot(materials_next, blueprint.obsidian), robots_next, max_so_far));
    }

    if materials.0 >= blueprint.clay.0 && materials.1 >= blueprint.clay.1 && materials.2 >= blueprint.clay.2 {
        let mut robots_next = robots;
        robots_next.1 += 1;

        max = std::cmp::max(max, bruteforce(blueprint, minute - 1, pay_for_robot(materials_next, blueprint.clay), robots_next, max_so_far));
    }

    if materials.0 >= blueprint.robot.0 && materials.1 >= blueprint.robot.1 && materials.2 >= blueprint.robot.2 {
        let mut robots_next = robots;
        robots_next.0 += 1;

        max = std::cmp::max(max, bruteforce(blueprint, minute - 1, pay_for_robot(materials_next, blueprint.robot), robots_next, max_so_far));
    }

    *max_so_far = std::cmp::max(*max_so_far, max);

    max
}

pub fn day19(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let parse = Regex::new(r#"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap();

    let mut blueprints: Vec<Blueprint> = vec![];

    for line in lines {
        let captures = parse.captures(line).unwrap();

        blueprints.push(Blueprint { 
            robot: (captures[1].parse::<u32>().unwrap(), 0, 0),
            clay: (captures[2].parse::<u32>().unwrap(), 0, 0),
            obsidian: (captures[3].parse::<u32>().unwrap(), captures[4].parse::<u32>().unwrap(), 0),
            geode: (captures[5].parse::<u32>().unwrap(), 0, captures[6].parse::<u32>().unwrap()),
        });
    }

    let first_half = blueprints.par_iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * bruteforce(x, 24, (0, 0, 0, 0), (1, 0, 0, 0), &mut 0))
        .sum::<u32>();

    println!("First half {}", first_half);

    let second_half = blueprints.par_iter()
        .take(3)
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * bruteforce(x, 32, (0, 0, 0, 0), (1, 0, 0, 0), &mut 0))
        .product::<u32>();

    println!("First half {}", second_half);
}
