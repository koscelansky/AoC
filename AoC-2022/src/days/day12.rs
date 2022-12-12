use std::fs;
use std::collections::BTreeMap;


fn to_height(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - 'a' as i32
    } else {
        match c {
            'S' => 0,
            'E' => 'z' as i32 - 'a' as i32,
            _ => panic!("unsupported char")
        }
    }
}

fn find_element(matrix: &[&str], needle: char) -> Option<(usize, usize)> {
    for (i, x) in matrix.iter().enumerate() {
        for (j, c) in x.chars().enumerate() {
            if needle == c {
                return Some((i, j))
            }
        }
    }

    None
}

fn dijkstra(map: &[Vec<i32>], start: (usize, usize)) -> Vec<Vec<i32>> {
    let mut result = vec![vec![-1; map[0].len()]; map.len()];

    let mut openset: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    openset.insert(start, 0); // let kick things offf

    while !openset.is_empty() {
        let (v, dist) = openset.iter().min_by(|a, b| a.1.cmp(b.1)).map(|(a, b)| (*a, *b)).unwrap();

        result[v.0][v.1] = dist;

        openset.remove(&v);

        // add new to open set and update

        if v.0 > 0 && result[v.0 - 1][v.1] == -1 &&  map[v.0 - 1][v.1] - map[v.0][v.1] >= -1 {
            let mut best = *openset.get(&(v.0 - 1, v.1)).unwrap_or(&i32::MAX);

            best = std::cmp::min(best, dist + 1);
            openset.insert((v.0 - 1, v.1), best);
        }

        if v.0 < map.len() - 1 && result[v.0 + 1][v.1] == -1 && map[v.0 + 1][v.1] - map[v.0][v.1] >= -1 {
            let mut best = *openset.get(&(v.0 + 1, v.1)).unwrap_or(&i32::MAX);

            best = std::cmp::min(best, dist + 1);
            openset.insert((v.0 + 1, v.1), best);
        }

        if v.1 > 0 && result[v.0][v.1 - 1] == -1 && map[v.0][v.1 - 1] - map[v.0][v.1] >= -1 {
            let mut best = *openset.get(&(v.0, v.1 - 1)).unwrap_or(&i32::MAX);

            best = std::cmp::min(best, dist + 1);
            openset.insert((v.0, v.1 - 1), best);
        }

        if v.1 < map[0].len() - 1 && result[v.0][v.1 + 1] == -1 && map[v.0][v.1 + 1] - map[v.0][v.1] >= -1 {
            let mut best = *openset.get(&(v.0, v.1 + 1)).unwrap_or(&i32::MAX);

            best = std::cmp::min(best, dist + 1);
            openset.insert((v.0, v.1 + 1), best);
        }
    }

    result
}


pub fn day12(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let map = lines.iter().map(|&line| line.chars().map(to_height).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    let start = find_element(&lines, 'S').unwrap();
    let end = find_element(&lines, 'E').unwrap();

    let paths = dijkstra(&map[..], end);

    println!("First half {}", paths[start.0][start.1]);

    let mut best = i32::MAX;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 && paths[i][j] >= 0 {
                best = std::cmp::min(best, paths[i][j]);
            }
        }
    }

    println!("Second half {}", best);
}