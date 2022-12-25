use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Elf {
    food: Vec<i32>,
    calories: i32,
}



fn main() {
    let mut elves = Vec::new();
    let mut first: bool = true;
    let mut max: i32 = 0;
    let mut max_index: usize = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut e: Elf = Elf { food: Vec::new(), calories: 0 };
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if !first {
                        if max < e.calories {
                            max = e.calories;
                            max_index = elves.len();
                        }
                        elves.push(e);
                    }
                    e = Elf { food: Vec::new(), calories: 0 }
                } else {
                    let parsed: i32 = ip.parse::<i32>().unwrap();
                    e.food.push(parsed);
                    e.calories = e.calories + parsed;
                }
                first = false;
            }
        }

        let mut tt: [i32; 3] = [0, 0, 0];

        for (i, x) in elves.iter().enumerate() {
            println!("> {}: {}, {:?}", i, x.calories, x.food);
            if x.calories > tt[0] {
                tt[2] = tt[1];
                tt[1] = tt[0];
                tt[0] = x.calories;
            } else if x.calories > tt[1] {
                tt[2] = tt[1];
                tt[1] = x.calories;
            } else if x.calories > tt[2] {
                tt[2] = x.calories;
            }
        }

        println!("max: {}", max);
        println!("max_index: {}", max_index + 1);
        println!("ttt: {}", tt[0] + tt[1] + tt[2])
    }
}




fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
