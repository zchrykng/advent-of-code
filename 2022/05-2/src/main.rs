
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn move_cargo(&mut self, number: usize, from: usize, to: usize) {
        let mut tmp_stack: Vec<char> = vec![];
        for _i in 0..number {
            let f = self.stacks[from-1].pop().unwrap();
            tmp_stack.push(f);
        }

        tmp_stack.reverse();

        for f in tmp_stack {
            self.stacks[to-1].push(f);
        }
    }

    fn get_top(&self) -> String {
        let mut result = String::from("");

        for v in 0..self.stacks.len() {
            let c = self.stacks[v][self.stacks[v].len()-1];
            result = format!("{}{}", result, c);
        }
        
        return result;
    }
}

fn parse_stack(mut stack_lines: Vec<String>) -> Ship {
    let mut ship: Ship = Ship { stacks: vec![] };

    let stack_index_str = stack_lines.pop().unwrap();

    for _i in stack_index_str.split_ascii_whitespace() {
        ship.stacks.push(vec![]);
    }
    
    for line in stack_lines {
        for index in 0..ship.stacks.len() {
            let c = line.chars().nth(1+index*4).unwrap();
            if c != ' ' {
                ship.stacks[index].push(c);
            }
        }        
    }

    for i in 0..ship.stacks.len() {
        ship.stacks[i].reverse();
    }

    return ship;
}

fn write_stack(ship: &Ship) {
    let mut lines: Vec<String> = vec![];
    let mut line: String = String::from("");
    let mut max_height: usize = 0;

    for i in 0..ship.stacks.len() {
        if i != 0 {
            line = format!("{}  ", line);
        }
        line = format!("{} {}", line, i+1);

        let stack_height = ship.stacks[i].len();

        if stack_height > max_height {
            max_height = stack_height;
        }
    }

    lines.push(line);

    for i in 0..max_height {
        let mut boxes: Vec<String> = vec![];
        for col in 0..ship.stacks.len() {
            let mut c: char = ' ';
            if ship.stacks[col].len() > i {
                c = ship.stacks[col][i];
            }

            if c == ' ' {
                boxes.push(String::from("   "));
            } else {
                boxes.push(format!("[{}]", c));
            }
        }

        lines.push(boxes.join(" "));
    }

    lines.reverse();

    for l in lines {
        println!("{}", l);
    }
}

fn parse_move(line: String) -> (usize, usize, usize) {
    let mut terms = line.split_whitespace();

    _ = terms.next(); // move
    let count = terms.next().unwrap().parse::<usize>().unwrap();

    _ = terms.next(); // from
    let from = terms.next().unwrap().parse::<usize>().unwrap();

    _ = terms.next(); // to
    let to = terms.next().unwrap().parse::<usize>().unwrap();

    return (count, from, to);
}

fn main() {
    let mut stack_strs: Vec<String> = vec![];
    let mut move_strs: Vec<String> = vec![];
    let mut in_stack: bool = true;
    

    let lines = if let Ok(lines) = read_lines("./input.txt") { lines } else { panic!("Should never happen!") };

    for line in lines {
        if let Ok(ip) = line {
            if in_stack {
                if ip == "" {
                    in_stack = false;
                    continue;
                }
                stack_strs.push(ip);
            } else {
                move_strs.push(ip);
            }
        }
    }


    let mut ship = parse_stack(stack_strs);
    
    write_stack(&ship);

    for move_str in move_strs {
        let (count, from, to) = parse_move(move_str);

        ship.move_cargo(count, from, to);
    }

    write_stack(&ship);

    println!("result: {}", ship.get_top());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

