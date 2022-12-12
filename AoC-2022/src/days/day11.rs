use std::fs;
use regex::Regex;

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: i32, // divisable by
    true_cond: i32,
    false_cond: i32,
}

fn to_function(cmd: &str) -> Box<dyn Fn(i64) -> i64> {
    let parse_function: Regex = Regex::new(r#"new = (old|\d+) (\+|\*) (old|\d+)"#).unwrap();

    let captures_function = parse_function.captures(cmd).unwrap();
    let param1 = captures_function[1].parse::<i64>();
    let op = &captures_function[2];
    let param2 = captures_function[3].parse::<i64>();

    match op {
        "+" => Box::new(move |x| param1.clone().unwrap_or(x) + param2.clone().unwrap_or(x)),
        "*" => Box::new(move |x| param1.clone().unwrap_or(x) * param2.clone().unwrap_or(x)),
        _ => panic!("unsupported operation")
    }
}

fn simulate(monkeys: &Vec<Monkey>, rounds: i32, divide: bool) -> usize {
    let mut inspected_items: Vec<usize> = vec![0; monkeys.len()];

    let mut items: Vec<Vec<i64>> = Vec::new();
    for i in monkeys {
        items.push(i.items.clone());
    }

    let division: i32 = monkeys.iter().map(|x| x.test).product(); // this can be gcd, but who cares

    // lets simulate
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            inspected_items[i] += items[i].len();

            let mut new_items:Vec<i64> = Vec::new();

            for item in items[i].iter() {
                let mut new_item = monkeys[i].operation.as_ref()(*item);
                if divide { 
                    new_item /= 3;
                } else { 
                    new_item %= division as i64;
                }

                new_items.push(new_item);
            }

            items[i].clear();

            for item in new_items {
                if item % monkeys[i].test as i64 == 0 {
                    let next = monkeys[i].true_cond as usize;
                    items[next].push(item);
                } else {
                    let next = monkeys[i].false_cond as usize;
                    items[next].push(item);
                }
            }
        }
    }
    //dbg!(&inspected_items);
    inspected_items.sort_by(|a, b| b.cmp(a));

    inspected_items[0] * inspected_items[1]
}

pub fn day11(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let monkeys_str = lines.split(|x| x.is_empty());

    let mut monkeys: Vec<Monkey> = Vec::new();

    for (i, x) in monkeys_str.enumerate() {
        if x.is_empty() {
            break;
        }

        let parse_id: Regex = Regex::new(r#"Monkey (\d+):"#).unwrap();
        let captures_id = parse_id.captures(x[0]).unwrap();
        let id = captures_id[1].parse::<i32>().unwrap();

        if id != i as i32 {
            panic!("monkeys must be in order");
        }

        let parse_items: Regex = Regex::new(r#"\s*Starting items: ([0-9, ]+)"#).unwrap();
        let captures_items = parse_items.captures(x[1]).unwrap();
        let items: Vec<i64> = captures_items[1].split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();        

        let parse_operation: Regex = Regex::new(r#"\s*Operation: (.+)"#).unwrap();
        let captures_operation = parse_operation.captures(x[2]).unwrap();
        let operation = &captures_operation[1];           

        let parse_test: Regex = Regex::new(r#"\s*Test: divisible by (\d+)"#).unwrap();
        let captures_test = parse_test.captures(x[3]).unwrap();
        let test = captures_test[1].parse::<i32>().unwrap();

        let parse_true: Regex = Regex::new(r#"\s*If true: throw to monkey (\d+)"#).unwrap();
        let captures_true = parse_true.captures(x[4]).unwrap();
        let true_cond = captures_true[1].parse::<i32>().unwrap();        

        let parse_false: Regex = Regex::new(r#"\s*If false: throw to monkey (\d+)"#).unwrap();
        let captures_false = parse_false.captures(x[5]).unwrap();
        let false_cond = captures_false[1].parse::<i32>().unwrap();  

        let x = Monkey {
            items,
            operation: to_function(operation),
            test,
            true_cond,
            false_cond,
        };

        monkeys.push(x);
    }

    let first_half = simulate(&monkeys, 20, true);

    println!("First half {}", first_half);

    let second_half = simulate(&monkeys, 10000, false);

    println!("Second half {}", second_half);
}