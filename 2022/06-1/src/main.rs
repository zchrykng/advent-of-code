
use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn unique_chars(s: &str) -> bool {
    let mut map:HashSet<char> = HashSet::new();

    for c in s.chars() {
        map.insert(c);
    }

    return map.len() == s.len();
}

fn main() {

    let mut result = 0;

    let lines = if let Ok(lines) = read_lines("./input.txt") { lines } else { panic!("Should never happen!") };

    let mut line: String = String::from("");
    for li in lines {
        if let Ok(ip) = li {
            line = ip;
        }
    }

    for i in 4..line.len() {
        if unique_chars(&line[i-4 .. i]) {
            result = i;
            break
        }
    }

    println!("result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

