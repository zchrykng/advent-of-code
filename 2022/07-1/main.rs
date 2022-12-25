use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct VecTree {
    values: Vec<usize>,
    children: Vec<HashMap<String, usize>>,
    parents: Vec<usize>,
    root: usize,
    current: usize
}

impl VecTree {
    fn new() -> VecTree {
        let mut vc = VecTree {
            values: vec![],
            children: vec![],
            parents: vec![],
            root: 0,
            current: 0
        };

        vc.values.push(0);
        vc.parents.push(vc.current);
        vc.children.push(HashMap::new());

        return vc;
    }

    fn add(&mut self, size: usize, name: String) {
        self.values.push(size);
        self.parents.push(self.current);
        self.children.push(HashMap::new());
        self.children[self.current].insert(name, self.values.len()-1);

        assert_eq!(self.values[self.values.len()-1], size);
    }

    fn cd(&mut self, location: String) {
        if location == ".." {
            self.current = self.parents[self.current];
        } else if location == "/" {
            self.current = self.root;
        } else {
            if self.children[self.current].contains_key(&location) {
                self.current = self.children[self.current][&location]
            } else {
                self.add(0, String::from(&location));
                self.current = self.children[self.current][&location];
            }
        }
    }
 
    fn size(&self, index: usize) -> usize {
        let mut size:usize;

        size = self.values[index];

        for (_k, v) in &self.children[index] {
            size += self.size(*v);
        }

        return size;
    }

    fn print(&self, mut level: usize, current: usize) {

        if level == 0 {
            println!("- / {}", self.size(0));
        }

        level = level + 1;

        for (k, v) in &self.children[current] {
            println!("{}- {} {}", "  ".repeat(level), k, self.size(*v));
            self.print(level, *v);
        }
    }

    fn folder_sizes(&self) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        let mut queue: Vec<usize> = vec![0];

        while queue.len() > 0 {
            let current = queue.pop().unwrap();
            let size = self.size(current);

            if self.children[current].len() > 0 {
                result.push(size);
            }

            for (_k, v) in &self.children[current] {
                queue.push(*v);
            }
        }

        return result;
    }

    fn filtered_size(&self, limit: usize) -> usize {
        let mut folders = self.folder_sizes();

        let mut size: usize = 0;

        for f in folders {
            if f <= limit {
                size += f;
            }
        }

        return size;
    }

    // fn filtered_size(&self, limit: usize) -> usize {
    //     let mut stack: Vec<usize> = vec![0];
    //     let mut folders: Vec<usize> = vec![];
    //     let mut stack_str: Vec<&str> = vec![];
        
    //     while stack.len() > 0 {
    //         let current = stack.pop().unwrap();
    //         let size = self.size(current);

    //         if size <= limit && self.children[current].len() > 0 {
    //             folders.push(size);
    //         }

    //         for (_k, v) in &self.children[current] {
    //             stack.push(*v);
    //         }
    //     }

    //     let mut size: usize = 0;
    //     while folders.len() > 0 {
    //         let f = folders.pop().unwrap();

    //         size += f;
    //     }

    //     return size;
    // }
    
}

fn main() {
    let mut tree = VecTree::new();

    let lines = if let Ok(lines) = read_lines("./input.txt") { lines } else { panic!("Should never happen!") };

    let mut line: String;
    for li in lines {
        if let Ok(ip) = li {
            line = ip;

//            println!("line: {}", line);

            if line.chars().nth(0).unwrap() == '$' {
                let remains = line.replace("$ ", "");
                let tokens: Vec<&str> = remains.split(" ").collect();

                if tokens[0] == "cd" {
                    tree.cd(String::from(tokens[1]));
                    continue
                }                    
            } else {
                let (size_str, name) = line.split_once(" ").unwrap();

                if size_str != "dir" {
                    let size = size_str.parse::<usize>().unwrap();
                    tree.add(size, String::from(name));
                }
            }
        }
    }
    tree.print(0, 0);

    println!("result: {}", tree.filtered_size(1000000));
    println!("Answer is not:");
    println!(" - {}", 54064882);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

