use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn parse_play(play: char) -> Play {
    match play {
        'A' => return Play::Rock,
        'B' => return Play::Paper,
        'C' => return Play::Scissors,
        _   => panic!("Should never get here!!!"),
    }
}

fn parse_result(result: char) -> Result {
    match result {
        'X' => return Result::Lose,
        'Y' => return Result::Draw,
        'Z' => return Result::Win,
        _ => panic!("Should never get here!!!"),
    }
}

// fn get_result(opp: Play, my: Play) -> Result {
//     if opp == my {
//         return Result::Draw
//     } else if
//         (opp == Play::Paper && my == Play::Scissors)
//         || (opp == Play::Rock && my == Play::Paper)
//         || (opp == Play::Scissors && my == Play::Rock)
//         {
//             return Result::Win;
//     } else {
//         return Result::Lose;
//     }
// }

fn get_play(opp: Play, result: Result) -> Play {
    if result == Result::Draw {
        match opp {
            Play::Rock => return Play::Rock,
            Play::Paper => return Play::Paper,
            Play::Scissors => return Play::Scissors,
        }
    } else if result == Result::Win {
        match opp {
            Play::Rock => return Play::Paper,
            Play::Paper => return Play::Scissors,
            Play::Scissors => return Play::Rock,
        }
    } else {
        match opp {
            Play::Rock => return Play::Scissors,
            Play::Paper => return Play::Rock,
            Play::Scissors => return Play::Paper,
        }
    }
}

fn play_value(play: Play) -> i32 {
    match play {
        Play::Rock     => return 1,
        Play::Paper    => return 2,
        Play::Scissors => return 3,
    }
}

fn result_value(result: Result) -> i32 {
    match result {
        Result::Win  => return 6,
        Result::Draw => return 3,
        Result::Lose => return 0,
    }
}



fn main() {
    let mut score: i32 = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let opp_char: char = ip.chars().nth(0).unwrap();
                let result_char: char = ip.chars().nth(2).unwrap();
                let opp: Play = parse_play(opp_char);
                let result: Result = parse_result(result_char);
                let my: Play = get_play(opp, result);


                score += play_value(my) + result_value(result)
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
