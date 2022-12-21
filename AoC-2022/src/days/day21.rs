use std::{fs, collections::HashMap};
use regex::Regex;

enum Operation {
    Plus(usize, usize),
    Minus(usize, usize),
    Multiply(usize, usize),
    Divide(usize, usize)
}

enum Monkey {
    Data { value: i32 },
    Operation(Operation),
}

impl Monkey {
    fn children(&self) -> Option<(usize, usize)> {
        match &self {
            Monkey::Data { value: _ } => None,
            Monkey::Operation(x) => match &x {
                Operation::Plus(a, b) => Some((*a, *b)),
                Operation::Minus(a, b) => Some((*a, *b)),
                Operation::Multiply(a, b) => Some((*a, *b)),
                Operation::Divide(a, b) => Some((*a, *b)),
            }
        }
    }
}

fn evaluate(monkeys: &Vec<Monkey>, node: usize) -> i64 {
    let monkey = &monkeys[node];

    match monkey {
        Monkey::Data { value } => *value as i64,
        Monkey::Operation(x) => match &x {
            Operation::Plus(a, b) => evaluate(monkeys, *a) + evaluate(monkeys, *b),
            Operation::Minus(a, b) => evaluate(monkeys, *a) - evaluate(monkeys, *b),
            Operation::Multiply(a, b) => evaluate(monkeys, *a) * evaluate(monkeys, *b),
            Operation::Divide(a, b) => evaluate(monkeys,  *a) / evaluate(monkeys, *b)
        }
    }
}


fn find_me(monkeys: &Vec<Monkey>, node: usize, me: usize) -> Option<Vec<bool>> {
    if node == me {
        Some(vec![false; 0])
    } else {
        if let Some((a, b)) = monkeys[node].children() {
            if let Some(mut x) = find_me(monkeys, a, me) {
                x.insert(0, false);
                return Some(x);
            }
    
            if let Some(mut x) = find_me(monkeys, b, me) {
                x.insert(0, true);
                return Some(x);
            }
        }

        None
    }
}

fn evaluate_with_me(monkeys: &Vec<Monkey>, node: usize, path: &[bool], current: i64) -> i64 {
    if path.is_empty() {
        return current;
    }

    let monkey = &monkeys[node];
    if let Some((left, right)) = monkey.children() {
        let (me_branch, value) = if path[0] {
            (right, evaluate(monkeys, left))
        } else {
            (left, evaluate(monkeys, right))
        };

        match monkey {
            Monkey::Data { value: _ } => panic!("node on me paths should not be a value"),
            Monkey::Operation(x) => match &x {
                Operation::Plus(_, _) => evaluate_with_me(monkeys, me_branch, &path[1..], current - value),
                Operation::Minus(_, _) => if !path[0] {
                    evaluate_with_me(monkeys, me_branch, &path[1..], current + value)
                } else {
                    // current = value - X
                    // - X = current - value
                    // X = value - current
                    evaluate_with_me(monkeys, me_branch, &path[1..], value - current)
                },
                Operation::Multiply(_, _) => evaluate_with_me(monkeys, me_branch, &path[1..], current / value),
                Operation::Divide(_, _) => if !path[0] {
                    evaluate_with_me(monkeys, me_branch, &path[1..], current * value)
                } else {
                    // current = value / X
                    // X * current = value
                    // X = value / current
                    evaluate_with_me(monkeys, me_branch, &path[1..],  value / current)
                }
            }
        }
    } else {
        panic!("node on me paths should not be a value")
    }
}

pub fn day21(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let parse = Regex::new(r#"(\w{4,4}): (.*)"#).unwrap();
    let parse_body = Regex::new(r#"(\w{4,4}) ([\-\+\*/]) (\w{4,4})"#).unwrap();

    // get ids to usize
    let mut text_to_id: HashMap<String, usize> = HashMap::new();
    for (idx, line) in content.split('\n').enumerate() {
        let captures = parse.captures(line).unwrap();

        let id = &captures[1];

        text_to_id.insert(String::from(id), idx);
    }

    let mut monkeys: Vec<Monkey> = Vec::new();

    for (i, line) in content.split('\n').enumerate() {
        let captures = parse.captures(line).unwrap();

        let id = &captures[1];
        assert!(i == *text_to_id.get(id).unwrap());

        let body = &captures[2];

        let monkey = if let Ok(value) = body.trim().parse::<i32>() {
            Monkey::Data { value }
        } else {
            let captures = parse_body.captures(body).unwrap();

            let a = *text_to_id.get(&captures[1]).unwrap();
            let b = *text_to_id.get(&captures[3]).unwrap();

            let operation = match &captures[2] {
                "+" => Operation::Plus(a, b),
                "-" => Operation::Minus(a, b),
                "*" => Operation::Multiply(a, b),
                "/" => Operation::Divide(a, b),
                _ => panic!("operation unknown"),
            };

            Monkey::Operation(operation)
        };

        monkeys.push(monkey);
    }

    let root = text_to_id.get("root").unwrap();

    let first_half = evaluate(&monkeys, *root);

    println!("First half {}", first_half);

    let me = text_to_id.get("humn").unwrap();

    let me_path = find_me(&monkeys, *root, *me);
    let second_half = match me_path {
        None => panic!("cannot find human"),
        Some(path) => {
            if let Some((left, right)) = &monkeys[*root].children() {
                let (me_branch, value) = if path[0] {
                    (right, evaluate(&monkeys, *left))
                } else {
                    (left, evaluate(&monkeys, *right))
                };

                evaluate_with_me(&monkeys, *me_branch, &path[1..], value)
            } else {
                panic!("root is a value?")
            }
        }
    };

    println!("Seconf hals {}", second_half);
}
