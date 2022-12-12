use std::fs;

fn mark_records(data: &Vec<i32>) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    res.resize(data.len(), false);

    let mut curr = -1;
    for (i, x) in data.iter().enumerate() {
        if x > &curr {
            curr = *x;
            res[i] = true;
        }
    }

    curr = -1;
    for (i, x) in data.iter().enumerate().rev() {
        if x > &curr {
            curr = *x;
            res[i] = true;
        }
    }

    res
}

fn transpose<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    for i in 0..v.len() {
        for j in 0..i {
            (v[i][j], v[j][i]) = (v[j][i], v[i][j])
        }
    }

    v
}

fn first_half(mut matrix: Vec<Vec<i32>>) -> i32 {
    let mut visibility: Vec<Vec<bool>> = Vec::new();

    for i in matrix.iter() {
        visibility.push(mark_records(i));
    }

    matrix = transpose(matrix);

    visibility = transpose(visibility);

    for (i, x) in matrix.iter().enumerate() {
        let r = mark_records(x);

        for (j, y) in r.iter().enumerate() {
            visibility[i][j] |= y;
        }
    }

    let count: usize = visibility.iter().map(|l| l.iter().filter(|c| **c).count()).sum();

    count as i32
}

fn scenic_score(matrix: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut right = 0;
    let mut left = 0;
    let mut down = 0;
    let mut up = 0;


    let tree = matrix[x][y];
    let size = matrix.len();


    for i in y+1..size {
        right += 1;
        
        if matrix[x][i] >= tree {
            break;    
        }
    }

    for i in (0..y).rev() {
        left += 1;
        
        if matrix[x][i] >= tree {
            break;    
        }
    }

    #[allow(clippy::needless_range_loop)] for i in x+1..size {
        down += 1;

        
        if matrix[i][y] >= tree {
            break;    
        }
    }

    for i in (0..x).rev() {
        up += 1;
        
        if matrix[i][y] >= tree {
            break;    
        }
    }

    left * right * up * down
}

fn second_half(matrix: Vec<Vec<i32>>) -> i32 {
    let mut best = 0;

    for i in 0..matrix.len() {
        for j in 0..i {
            best = std::cmp::max(best, scenic_score(&matrix, i, j))        
        }
    }

    best
}

pub fn day8(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines = content.split('\n');

    let matrix = lines.map(|w| w.chars().map(|c| c as i32 - '0' as i32).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    println!("First half {}", first_half(matrix.clone()));

    println!("Second half {}", second_half(matrix));
}