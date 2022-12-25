use std::collections::HashMap;
use std::{env, process, fmt, vec};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // process a single step move
    fn process_move(&mut self, d: Movement) {
        match d {
            Movement::Nothing   => return,
            Movement::Up        => self.y += 1,
            Movement::Down      => self.y -= 1,
            Movement::Right     => self.x += 1,
            Movement::Left      => self.x -= 1,
            Movement::UpRight   => { self.y += 1; self.x += 1 },
            Movement::UpLeft    => { self.y += 1; self.x -= 1 },
            Movement::DownRight => { self.y -= 1; self.x += 1 },
            Movement::DownLeft  => { self.y -= 1; self.x -= 1 },
        }
    }

    // figure out how to move toward other point
    fn direction(begin: Point, end: Point, diagonal: bool, not_these: Vec<Movement>) -> Movement {
        let x_diff = end.x - begin.x; // positive is a rightward motion
        let y_diff = end.y - begin.y; // positive is a upward motion

        if x_diff < 1 && x_diff > -1 && y_diff < 1 && y_diff > -1 {
            return Movement::Nothing;
        } else if (x_diff   == 0 && y_diff >  1)
            && !not_these.contains(&Movement::Up) {
            return Movement::Up;
        } else if (x_diff   == 0 && y_diff < -1)
            && !not_these.contains(&Movement::Down) {
            return Movement::Down;
        } else if (x_diff   >  1 && y_diff == 0)
            && !not_these.contains(&Movement::Right) {
            return Movement::Right;
        } else if (x_diff   < -1 && y_diff == 0)
            && !not_these.contains(&Movement::Left) {
            return Movement::Left;
        } else if diagonal
            && !not_these.contains(&Movement::UpRight)
            && (( x_diff >  1 && y_diff >  0) || (x_diff > 0 && y_diff >  1)) {
            return Movement::UpRight;
        } else if diagonal
            && !not_these.contains(&Movement::UpLeft)
            && (( x_diff < -1 && y_diff >  0) || (x_diff < 0 && y_diff >  1)) {
            return Movement::UpLeft;
        } else if diagonal
            && !not_these.contains(&Movement::DownRight)
            && (( x_diff >  1 && y_diff <  0) || (x_diff > 0 && y_diff < -1)) {
            return Movement::DownRight;
        } else if diagonal
            && !not_these.contains(&Movement::DownLeft)
            && (( x_diff < -1 && y_diff <  0) || (x_diff < 0 && y_diff < -1)) {
            return Movement::DownLeft;
        } else {
            return Movement::Nothing;
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Movement {
    Nothing,
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

fn print_map_char(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for v in 0..map[i].len() {
            print!("{}", map[i][v]);
        }
        print!("\n");
    }
}

fn print_map_i32(map: &Vec<Vec<i32>>) {
    for i in 0..map.len() {
        for v in 0..map[i].len() {
            print!("{}", map[i][v]);
        }
        print!("\n");
    }
}

fn map_char_to_int(c: char) -> i16 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'E' => i16::MAX,
        'S' => 0,
        _ => panic!("Should never get here!!!"),
    }
}

fn map_int_to_char(i: i16) -> char {
    match i {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        10 => 'j',
        11 => 'k',
        12 => 'l',
        13 => 'm',
        14 => 'n',
        15 => 'o',
        16 => 'p',
        17 => 'q',
        18 => 'r',
        19 => 's',
        20 => 't',
        21 => 'u',
        22 => 'v',
        23 => 'w',
        24 => 'x',
        25 => 'y',
        26 => 'z',
        i16::MAX => 'E',
        0 => 'S',
        _ => panic!("Should never get here!!!"),
    }
}

fn main () {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let mut map_str: Vec<Vec<char>> = vec![];
    let mut map: Vec<Vec<i16>> = vec![];
    let mut start: Point = Point { x: 0, y: 0 };
    let mut end: Point = Point { x: 0, y: 0 };
    let mut current: Point = Point { x: 0, y: 0 };
    let mut moves: Vec<Movement> = vec![];
    let mut positions: Vec<Point> = vec![];
    let mut visited: HashMap<Point, bool> = HashMap::new();

    let filename: &str = &args[1];

    let lines = if let Ok(lines) = read_lines(&filename) { lines } else { panic!("Should never happen!") };
    for ip in lines {
        if let Ok(line) = ip {
            let mut row: Vec<char> = vec![];

            for (_i, c) in line.chars().enumerate() {
                row.push(c);
            }
            map_str.push(row);
        }
    }
    print_map_char(&map_str);

    while current != end {
        let ideal = Point::direction(current, end, false, vec![]);
        if possible_move(&map, current, ideal) {

        }
    }
}

fn possible_moves(map: &Vec<Vec<i16>>, visited: HashMap<Point, bool>, cur: Point) -> Vec<Movement> {
    let mut options: Vec<Movement> = vec![];
    let height = map[cur.y as usize][cur.x as usize];
    let map_height = map.len();
    let map_width = map[0].len();

    for d in [Movement::Up, Movement::Right, Movement::Down, Movement::Left] {
        let mut np = cur;
        match d {
            Movement::Up    => np.y -= 1,
            Movement::Left  => np.x -= 1,
            Movement::Down  => np.y += 1,
            Movement::Right => np.x += 1,
            _ => panic!("shouldnt get here"),
        }

        if !visited[&np] { continue; }
        if np.x < 0 { continue; }
        if np.y < 0 { continue; }
        if np.x >= map_width as i32 { continue; }
        if np.y >= map_height as i32 { continue; }
        if map[np.y as usize][np.x as usize] - height > 1 {
            continue;
        }

        options.push(d);
    }

    return options;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}