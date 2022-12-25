use std::{env, fs, process};

fn main () {
    let mut queue: Vec<i32> = vec![];
    let mut register_x:i32 = 1;

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
        if line.contains(" ") {
            let (_cmd_str, remainder) = line.split_once(" ").unwrap();
            queue.push(0);
            queue.push(remainder.parse::<i32>().unwrap());
        } else {
            queue.push(0);
        }
    });

    let mut cursor: i32 = 0;

    for op in queue {

        
        if cursor >= register_x - 1 && cursor <= register_x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        
        cursor += 1;
        if cursor > 39 {
            print!("\n");
            cursor = 0;
        }

        register_x += op; // keep at end
    }
}
