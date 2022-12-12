use std::{fs, collections::HashMap};
use regex::Regex;
use std::rc::Rc;
use std::cell::RefCell;
use lazy_static::lazy_static;

enum FsData {
    File { size: u64 },
    Dir { children: HashMap<String, Rc<RefCell<FsNode>>> },
}

struct FsNode {
    data: FsData
}

impl FsNode {
    fn new_dir() -> Self {
        Self {
            data: FsData::Dir { children: HashMap::new() }
        }
    }

    fn new_file(size: u64) -> Self {
        Self {
            data: FsData::File { size }
        }
    }

    fn insert(&mut self, name: &str, node: FsNode) {
        let ins = match &mut self.data {
            FsData::File { .. } => panic!("cannot insert to file"),
            FsData::Dir { children } => children.insert(String::from(name), Rc::new(RefCell::new(node))).is_some()
        };

        if ins {
            panic!("we do not allow more than objects with the same name");
        }
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<FsNode>>> {
        match &self.data {
            FsData::File { .. } => None,
            FsData::Dir { children } => Some(Rc::clone(children.get(name).unwrap()))
        }    
    }

    fn total(&self) -> u64 {
        match &self.data {
            FsData::File { size } => *size,
            FsData::Dir { children } => children.values().map(|ch| ch.borrow().total()).sum()
        }
    }

    fn sum_with_threshold(&self, threshold: u64) -> u64 {
        match &self.data {
            FsData::Dir { children } => {
                let mut sum: u64 = 0;
                if self.total() < threshold {
                    sum += self.total();
                }

                sum += children
                    .values()
                    .map(|ch| ch.borrow().sum_with_threshold(threshold))
                    .sum::<u64>();

                sum
            },
            _ => 0,
        }
    }

    fn smallest_covering(&self, capacity: u64) -> Option<u64> {
        let total = self.total();

        if total < capacity {
            None
        } else {
            let mut smallest = total;

            match &self.data {
                FsData::Dir { children } => {
                    for v in children.values() {
                        let x = v.borrow().smallest_covering(capacity);

                        match x {
                            None => continue,
                            Some(data) => { smallest = std::cmp::min(smallest, data) }
                        }
                    }
                },
                _ => return None
            };

            Some(smallest)
        }
    }
}

fn find_parent(root: &Rc<RefCell<FsNode>>, node: &Rc<RefCell<FsNode>>) -> Option<Rc<RefCell<FsNode>>> {
    match &root.borrow().data {
        FsData::File { .. } => return None,
        FsData::Dir { children } => {
            for v in children.values() {
                if Rc::ptr_eq(v, node) {
                    return Some(Rc::clone(root));
                } else {
                    match find_parent(v, node) {
                        None => continue,
                        Some (v) => return Some(v),
                    }
                }
            }
        }
    };

    None
}

fn process_command(root: &Rc<RefCell<FsNode>>, cwd: Rc<RefCell<FsNode>>, command: &str, output: &[&str]) -> Rc<RefCell<FsNode>> {
    lazy_static! {
        static ref PARSE: Regex = Regex::new(r#"\$\s+(\w+)\s*(\S*)"#).unwrap();
    }
    
    let captures = PARSE.captures(command).unwrap();

    let cmd: &str = &captures[1];
    let param: &str = &captures[2];

    match cmd {
        "cd" => {
            if param.is_empty() {
                panic!("cd should have one parameter");
            }

            match param {
                "/" => Rc::clone(root),
                "." => cwd,
                ".." => find_parent(root, &cwd).unwrap(),
                x => cwd.borrow().get_child(x).unwrap()
            }
        },
        "ls" => {
            for i in output {
                lazy_static! {
                    static ref FILE_PARSE: Regex = Regex::new(r#"(\d+)\s+(\S+)"#).unwrap();
                    static ref DIR_PARSE: Regex = Regex::new(r#"dir\s+(\S+)"#).unwrap();
                }

                if let Some(res) = FILE_PARSE.captures(i) {
                    let size: &str = &res[1];
                    let name: &str = &res[2];
                    
                    cwd.borrow_mut().insert(name, FsNode::new_file(size.parse::<u64>().unwrap()))
                } else if let Some(res) = DIR_PARSE.captures(i) {
                    let name: &str = &res[1];

                    cwd.borrow_mut().insert(name, FsNode::new_dir())
                } else {
                    panic!("ls should either return dir or file");
                }
            }

            cwd
        },
        &_ => { panic!("we do not recognize this command") }
    }
}

pub fn day7(input: &str) {
    let content = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let root = Rc::new(RefCell::new(FsNode::new_dir()));
    let mut cwd = Rc::clone(&root);

    let mut command: &str = "";
    let mut output: Vec<&str> = Vec::new();

    for line in content.split('\n') {
        if line.starts_with('$') {
            if !command.is_empty() {
                cwd = process_command(&root, cwd, command, &output[..]);

                output.clear();
            }

            command = line;
        } else {
            output.push(line);
        }
    }

    // last one
    if !command.is_empty() {
        process_command(&root, cwd, command, &output[..]);
    }

    println!("First half {}", &root.borrow().sum_with_threshold(100000));

    let total = root.borrow().total();

    let capacity = 70000000 - total;

    let need_to_free = 30000000 - capacity;

    println!("First half {}", root.borrow().smallest_covering(need_to_free).unwrap());
}