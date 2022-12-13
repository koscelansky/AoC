use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;


enum AstData {
    Element { value: i32 },
    List { children: Vec<Rc<RefCell<AstNode>>> },
}

struct AstNode {
    data: AstData
}

fn find_matching_bracket(x: &[char]) -> usize {
    let mut stack = 0;
    for (i, x) in x.iter().enumerate() {
        match x {
            '[' => stack += 1,
            ']' => stack += -1,
            _ => (),
        }

        if stack == 0 {
            return i
        }
    }

    panic!("string has no matchin brackets");
}


impl AstNode {
    fn new_list(value: i32) -> Self {
        Self { data: AstData::List { children: vec![
            Rc::new(RefCell::new(Self { data: AstData::Element { value } }))
        ] } }
    }

    fn parse(x: &[char]) -> Self {
        assert!(x[0] == '[');

        let mut children: Vec<Rc<RefCell<AstNode>>> = vec![];

        let mut a = &x[1..x.len()-1]; // chop first and last char

        loop {
            let next = match a.iter().next().unwrap_or(&'A') { // doesnt't really matter
                '[' => {
                    let last = find_matching_bracket(a);

                    children.push(
                        Rc::new(RefCell::new(Self::parse(&a[0..= last])))
                    );

                    last + 1
                },
                '0'..= '9' => {
                    let last = a.iter().position(|x| *x == ',').unwrap_or(a.len());
                    let num: i32 = a[0..last].iter().collect::<String>().parse().unwrap();

                    children.push(
                        Rc::new(RefCell::new(Self {
                            data: AstData::Element { value: num }
                        }))
                    );

                    last + 1
                },
                _ => 1,
            };

            if next >= a.len() {
                break
            }

            a = &a[next..];
        }

        Self { data: AstData::List { children } }
    }
}

fn compare(a: &AstNode, b: &AstNode) -> Ordering {
    match (&a.data, &b.data) {
        ( AstData::Element { value: a }, AstData::Element { value: b } ) => a.cmp(b),
        ( AstData::Element { value: a }, AstData::List { children: _ } ) => compare(&AstNode::new_list(*a), b),
        ( AstData::List { children: _ }, AstData::Element { value: b } ) => compare(a, &AstNode::new_list(*b)),
        ( AstData::List { children: a }, AstData::List { children: b } ) => {
            for i in 0..std::cmp::min(a.len(), b.len()) {
                let cmp = compare(&a[i].borrow(), &b[i].borrow());

                match &cmp {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => continue,
                    Ordering::Greater => return Ordering::Greater
                }
            }

            a.len().cmp(&b.len())
        }
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            AstData::Element { value } => write!(f, "{}", value),
            AstData::List { children } => {
                write!(f, "[ ")?;

                for i in children.iter() {
                    write!(f, "{} ", i.borrow())?;
                }

                write!(f, "]")?;

                Ok(())
            }
        }
    }
}

pub fn day13(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = content.split('\n').collect();

    let pairs = lines
        .split(|x| x.is_empty())
        .map(|x| x.iter().copied().collect_tuple::<(&str, &str)>().unwrap());

    let mut signals: Vec<AstNode> = Vec::new();

    let mut sum = 0;
    for (i, (a, b)) in pairs.enumerate() {
        let a_chars = a.chars().collect::<Vec<char>>();
        let b_chars = b.chars().collect::<Vec<char>>();
        
        let ast_a = AstNode::parse(&a_chars);
        let ast_b = AstNode::parse(&b_chars);

        if compare(&ast_a, &ast_b) == Ordering::Less {
            sum += i + 1;
        }

        signals.push(ast_a);
        signals.push(ast_b);
    }

    println!("First one is {}", sum);

    // add dividers
    signals.push(AstNode::parse(&"[[2]]".chars().collect::<Vec<char>>()));
    signals.push(AstNode::parse(&"[[6]]".chars().collect::<Vec<char>>()));

    signals.sort_by(compare);

    let s: Vec<String> = signals.iter().map(|x| format!("{}", x)).collect();

    let second = (s.iter().position(|x| *x == "[ [ 2 ] ]").unwrap() + 1) * (s.iter().position(|x| *x == "[ [ 6 ] ]").unwrap() + 1);

    println!("Second one is {}", second);
}