use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

fn parse_play(play: char) -> Play {
    match play {
        'A' | 'X' => return Play::Rock,
        'B' | 'Y' => return Play::Paper,
        'C' | 'Z' => return Play::Scissors,
        _ => panic!("Should never get here!!!"),
    }
}


fn play_value(play: Play) -> i32 {
    match play {
        Play::Rock     => return 1,
        Play::Paper    => return 2,
        Play::Scissors => return 3,
    }
}

fn result_value(opp: Play, my: Play) -> i32 {
    if opp == my {
        return 3
    } else if
        (opp == Play::Paper && my == Play::Scissors)
        || (opp == Play::Rock && my == Play::Paper)
        || (opp == Play::Scissors && my == Play::Rock)
        {
            return 6;
    } else {
        return 0;
    }
}



fn main() {
    let mut score: i32 = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let opp_char: char = ip.chars().nth(0).unwrap();
                let my_char: char = ip.chars().nth(2).unwrap();
                let opp: Play = parse_play(opp_char);
                let my: Play = parse_play(my_char);

                score += play_value(my) + result_value(opp, my)
            }
        }
    }

    println!("total score: {}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
