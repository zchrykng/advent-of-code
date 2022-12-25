use core::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Range(i32, i32);

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

// fn is_inside(f: Range, s: Range) -> bool {
//     if f.0 >= s.0 && f.1 <= s.1 {
//         return true;
//     } else if s.0 >= f.0 && s.1 <= f.1 {
//         return true;
//     } else {
//         return false;
//     }
// }

fn is_overlap(f: Range, s: Range) -> bool {

    if f.0 >= s.0 && f.0 <= s.1 {
        return true;
    } else if f.1 >= s.0 && f.1 <= s.1 {
        return true;
    } else if s.0 >= f.0 && s.0 <= f.1 {
        return true;
    } else if s.1 >= f.0 && s.1 <= f.1 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut value: i32 = 0;
    
    let lines = if let Ok(lines) = read_lines("./input.txt") { lines } else { panic!("Should never happen!") };

    for line in lines {
        if let Ok(ip) = line {
            let mut split = ip.split(",");
            let left_str = split.next().unwrap();
            let right_str = split.next().unwrap();


            let mut l_split = left_str.split("-");
            let left1 = l_split.next().unwrap();
            let left2 = l_split.next().unwrap();

            let l1: i32 = left1.parse().unwrap();
            let l2: i32 = left2.parse().unwrap();


            let mut r_split = right_str.split("-");
            let right1 = r_split.next().unwrap();
            let right2 = r_split.next().unwrap();

            let r1: i32 = right1.parse().unwrap();
            let r2: i32 = right2.parse().unwrap();

            let left = Range(l1, l2);
            let right = Range(r1, r2);

            if is_overlap(left, right) {
                value += 1;
            }
        }
    }

    println!("total score: {}", value);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
