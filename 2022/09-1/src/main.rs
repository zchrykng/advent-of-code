use core::fmt;
use std::{env, fs, process, collections::HashMap};

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
    fn direction(begin: Point, end: Point) -> Movement {
        let x_diff = end.x - begin.x; // positive is a rightward motion
        let y_diff = end.y - begin.y; // positive is a upward motion

        if x_diff <= 1 && x_diff >= -1 && y_diff <= 1 && y_diff >= -1 {
            return Movement::Nothing;
        } else if x_diff   == 0 && y_diff >  1 {
            return Movement::Up;
        } else if x_diff   == 0 && y_diff < -1 {
            return Movement::Down;
        } else if x_diff   >  1 && y_diff == 0 {
            return Movement::Right;
        } else if x_diff   < -1 && y_diff == 0 {
            return Movement::Left;
        } else if ( x_diff >  1 && y_diff >  0) || (x_diff > 0 && y_diff >  1) {
            return Movement::UpRight;
        } else if ( x_diff < -1 && y_diff >  0) || (x_diff < 0 && y_diff >  1) {
            return Movement::UpLeft;
        } else if ( x_diff >  1 && y_diff <  0) || (x_diff > 0 && y_diff < -1) {
            return Movement::DownRight;
        } else if ( x_diff < -1 && y_diff <  0) || (x_diff < 0 && y_diff < -1) {
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

struct Rope {
    segments: Vec<Point>,
    head: usize,
    tail: usize,
}

impl Rope {
    fn new(start: Point, segments: usize) -> Self {
        let mut r = Rope { segments: vec![], head: 0, tail: segments };

        r.segments.push(start);

        for _i in 0..segments {
            r.segments.push(start);
        }

        println!("rope length: {}", r.segments.len());
        println!("rope head index: {}", r.head);
        println!("rope tail index: {}", r.tail);

        return r;
    }



    fn process_move(&mut self, d: Movement) -> Point {
        self.segments[self.head].process_move(d);

        let mut end = self.segments[0];
        let mut begin: Point;

        for i in 1..self.segments.len() {
            begin = self.segments[i];
            self.segments[i].process_move(Point::direction(begin, end));
            end = self.segments[i];
        }
        
        return self.segments[self.tail];
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

fn main () {

    let mut visits: HashMap<Point, bool> = HashMap::new();
    let mut moves: Vec<Movement> = vec![];
    let start: Point = Point { x: 0, y: 0 };

    let mut rope = Rope::new(start, 9);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let filename: &str = &args[1];
    fs::read_to_string(&filename).unwrap_or_else(|error| {
        panic!("error {}: {:?}", &filename, &error);
    })
    .lines()
    .for_each(|line| {
        let (dir, amount_str) = line.split_once(" ").unwrap();

        let direction: Movement;

        match dir {
            "U" => direction = Movement::Up,
            "D" => direction = Movement::Down,
            "R" => direction = Movement::Right,
            "L" => direction = Movement::Left,
            _ => panic!("Should never end up here")
        }

        let amount = amount_str.parse::<u32>().unwrap();

        for _i in 0..amount {
            moves.push(direction);
        }
    });

    for m in moves {
        visits.insert(rope.process_move(m), true);

        
    }

    println!("Result: {}", visits.len());

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        let center = Point { x: 0, y: 0 };

        let mut test_points: Vec<Point> = vec![];
        let mut test_answers: Vec<Movement> = vec![];

        // shared location test
        test_points.push(Point{ x: 0, y: 0});
        test_answers.push(Movement::Nothing);

        // within 1 tests
        test_points.push(Point{ x: 1, y: 0});
        test_answers.push(Movement::Nothing);

        test_points.push(Point{ x: 0, y: 1});
        test_answers.push(Movement::Nothing);

        test_points.push(Point{ x: -1, y: 0});
        test_answers.push(Movement::Nothing);
        
        test_points.push(Point{ x: 0, y: -1});
        test_answers.push(Movement::Nothing);

        test_points.push(Point{ x: 1, y: 1});
        test_answers.push(Movement::Nothing);

        test_points.push(Point{ x: 1, y: -1});
        test_answers.push(Movement::Nothing);

        test_points.push(Point{ x: -1, y: 1});
        test_answers.push(Movement::Nothing);
        
        test_points.push(Point{ x: -1, y: -1});
        test_answers.push(Movement::Nothing);


        // direct tests
        test_points.push(Point{ x: 2, y: 0});
        test_answers.push(Movement::Right);

        test_points.push(Point{ x: 0, y: 2});
        test_answers.push(Movement::Up);

        test_points.push(Point{ x: -2, y: 0});
        test_answers.push(Movement::Left);
        
        test_points.push(Point{ x: 0, y: -2});
        test_answers.push(Movement::Down);


        // diagnal tests
        // upward tests
        test_points.push(Point{ x: 2, y: 1});
        test_answers.push(Movement::UpRight);

        test_points.push(Point{ x: 1, y: 2});
        test_answers.push(Movement::UpRight);

        test_points.push(Point{ x: -2, y: 1});
        test_answers.push(Movement::UpLeft);

        test_points.push(Point{ x: -1, y: 2});
        test_answers.push(Movement::UpLeft);


        // downward tests
        test_points.push(Point{ x: 2, y: -1});
        test_answers.push(Movement::DownRight);

        test_points.push(Point{ x: 1, y: -2});
        test_answers.push(Movement::DownRight);

        test_points.push(Point{ x: -2, y: -1});
        test_answers.push(Movement::DownLeft);

        test_points.push(Point{ x: -1, y: -2});
        test_answers.push(Movement::DownLeft);

        test_points.reverse();
        test_answers.reverse();

        for i in 0..test_points.len() - 1 {
            assert_eq!(Point::direction(center, test_points[i]), test_answers[i]);
        }
        
    }
}