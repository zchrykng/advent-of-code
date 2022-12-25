use std::{env, fs, process};

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn main () {

    let mut forest: Vec<Vec<u32>> = vec![];

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
        let mut row: Vec<u32> = vec![];

        line.split("").for_each(| c | {
            if c != "" {
                row.push(c.parse::<u32>().unwrap());
            }
        });

        forest.push(row);
    });

    let mut visible_count: u32 = 0;
    let mut max_scenic_score: u32 = 0;

    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            let tree = Point{ x, y };
            if is_visible(&forest, tree) {
                visible_count += 1;
            }

            let score = scenic_score(&forest, tree);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    println!("Visible trees: {}", visible_count);
    println!("Max scenic score: {}", max_scenic_score);

}

fn scenic_score(forest: &Vec<Vec<u32>>, tree: Point) -> u32 {
    let height: u32 = forest[tree.y][tree.x];

    if tree.x == 0 || tree.y == 0 || tree.x == forest[tree.y].len()-1 || tree.y == forest.len()-1 {
        return 0;
    }

    // Check left
    let mut current: Point = Point { x: tree.x, y: tree.y };
    let mut left: u32 = 0;
    while  current.x > 0 {
        current.x -= 1;
        if forest[current.y][current.x] < height {
            left += 1;
        } else {
            left += 1;
            break
        }
    }
    

    // Check right
    current = Point { x: tree.x, y: tree.y  };
    let mut right: u32 = 0;
    while current.x < forest[current.y].len() - 1 {
        current.x += 1;
        if forest[current.y][current.x] < height {
            right += 1;
        } else {
            right += 1;
            break;
        }
    }

    // Check up
    current = Point { x: tree.x, y: tree.y };
    let mut top: u32 = 0;
    while current.y > 0 {
        current.y -= 1;
        if forest[current.y][current.x] < height {
            top  += 1;
        } else {
            top += 1;
            break;
        }
    }

    // Check down
    current = Point { x: tree.x, y: tree.y };
    let mut bottom: u32 = 0;
    while current.y < forest.len() - 1 {
        current.y += 1;
        if forest[current.y][current.x] < height {
            bottom  += 1;
        } else {
            bottom += 1;
            break;
        }
    }

    return left * top * right * bottom;
}

fn is_visible(forest: &Vec<Vec<u32>>, p: Point) -> bool {
    let height: u32 = forest[p.y][p.x];

    if p.x == 0 || p.y == 0 || p.x == forest[p.y].len()-1 || p.y == forest.len()-1 {
        return true
    }

    // Check left
    let mut current: Point = Point { x: 0, y: p.y };
    let mut left: bool = true;
    while current.x < p.x {
        if forest[current.y][current.x] >= height {
            left = false;
            break;
        }
        current.x += 1;
    }

    // Check right
    current = Point { x: forest[current.y].len() - 1, y: p.y  };
    let mut right: bool = true;
    while p.x < current.x {
        if forest[current.y][current.x] >= height {
            right = false;
            break;
        }
        current.x -= 1;
    }

    // Check up
    current = Point { x: p.x, y: 0 };
    let mut top: bool = true;
    while current.y < p.y {
        if forest[current.y][current.x] >= height {
            top = false;
            break;
        }
        current.y += 1;
    }

    // Check down
    current = Point { x: p.x, y: forest.len() - 1 };
    let mut bottom: bool = true;
    while p.y < current.y {
        if forest[current.y][current.x] >= height {
            bottom = false;
            break;
        }
        current.y -= 1;
    }

    return left || top || right || bottom;
}
