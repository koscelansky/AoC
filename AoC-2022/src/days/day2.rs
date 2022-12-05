use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Kind {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Result {
    Win,
    Draw,
    Lost,
}

fn result(me: Kind, you: Kind) -> Result {
    if me == you {
        Result::Draw
    } else if you == Kind::Rock && me == Kind::Paper
           || you == Kind::Paper && me == Kind::Scissors
           || you == Kind::Scissors && me == Kind::Rock {
        Result::Win
    } else {
        Result::Lost
    }
}

fn opponent(c: char) -> Kind {
    match c {
        'A' => Kind::Rock,
        'B' => Kind::Paper,
        'C' => Kind::Scissors,
        _ => std::panic!("This should not be possible.")
    }
}

fn player(c: char) -> Kind {
    match c {
        'X' => Kind::Rock,
        'Y' => Kind::Paper,
        'Z' => Kind::Scissors,
        _ => std::panic!("This should not be possible.")
    }
}

fn expected_result(c: char) -> Result {
    match c {
        'X' => Result::Lost,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => std::panic!("This should not be possible.")
    }        
}

fn what_to_play(res: Result, you: Kind) -> Kind {
    if res == Result::Draw {
        you
    } else if res == Result::Win {
        match you {
            Kind::Paper => Kind::Scissors,
            Kind::Rock => Kind::Paper,
            _ => Kind::Rock
        }
    } else {
        match you {
            Kind::Paper => Kind::Rock,
            Kind::Rock => Kind::Scissors,
            _ => Kind::Paper
        }    
    }
}

pub fn day2(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let rounds: Vec<&str> = content.split('\n').collect();
        
    let mut score = 0;
    for round in rounds.iter() {
        let plies: Vec<&str> = round.split(' ').collect();

        let me = player(plies[1].trim().chars().next().expect("Should not be empty"));
        let you = opponent(plies[0].trim().chars().next().expect("Should not be empty"));

        let res = result(me, you);

        score += match res {
            Result::Win => 6,
            Result::Draw => 3,
            _ => 0
        };

        score += match me {
            Kind::Rock => 1,
            Kind::Paper => 2,
            Kind::Scissors => 3, 
        }
    }

    println!("1 half answer is {score}");

    score = 0;
    for round in rounds.iter() {
        let plies: Vec<&str> = round.split(' ').collect();

        let res = expected_result(plies[1].trim().chars().next().expect("Should not be empty"));
        let you = opponent(plies[0].trim().chars().next().expect("Should not be empty"));

        let me = what_to_play(res, you);

        score += match res {
            Result::Win => 6,
            Result::Draw => 3,
            _ => 0
        };

        score += match me {
            Kind::Rock => 1,
            Kind::Paper => 2,
            Kind::Scissors => 3, 
        }
    }

    println!("1 half answer is {score}");
}