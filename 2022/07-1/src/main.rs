use std::{env, fs, process};
use std::collections::HashMap;
use regex::Regex;

const MAX_SIZE: u32 = 100000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let mut curr_path: Vec<String> = vec![];
    let mut dirsizes: HashMap<String, u32> = HashMap::<String, u32>::new();
    let cd_regex = Regex::new(r"^\$ cd (\S+)").unwrap();
    let file_regex = Regex::new(r"^(\d+) (\S+)").unwrap();
    let filename: &str = &args[1];
    fs::read_to_string(&filename).unwrap_or_else( |error| {
            panic!("error reading {}: {:?}", &filename, &error)
        })
        .lines()
        .for_each( |line| {
            if cd_regex.is_match(line) {
                let caps = cd_regex.captures(line).unwrap();
                let mut dirname = caps.get(1).unwrap().as_str().to_string();
                if dirname == ".." {
                    curr_path.pop();
                } else {
                    if dirname == "/" {
                        dirname = "<root>".to_string();
                    }
                    curr_path.push(dirname);
                    let path_str = curr_path.join("/");
                    if ! dirsizes.contains_key(&path_str) {
                        dirsizes.insert(path_str, 0);
                    }
                }
            } else if file_regex.is_match(line) {
                let caps = file_regex.captures(line).unwrap();
                let filesize: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let mut update_list: Vec<String> = vec![];
                for dir in &curr_path {
                    update_list.push(dir.to_string());
                    let update_path = update_list.join("/");
                    dirsizes.entry(update_path).and_modify(|size| *size += filesize);
                }
            }
        });

        for d in dirsizes.values() {
            println!("{}", d);
        }

    let total: u32 = dirsizes.values().filter( |&size| *size < MAX_SIZE).sum();
    println!("Total size is {}", total);
}
