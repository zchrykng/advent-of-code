use std::{env, process, fmt};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct BarrelOfMonkeys {
    monkeys: Vec<Monkey>,
}

impl BarrelOfMonkeys {
    fn new() -> Self {
        return BarrelOfMonkeys { monkeys: vec![] };
    }

    fn push(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn business(&self) -> i32 {
        let mut monks: Vec<i32> = vec![];
        for i in 0..self.monkeys.len() {
            monks.push(self.monkeys[i].inspect_count);
        }

        monks.sort();
        monks.reverse();
        
        return monks[0] * monks[1];
    }

    fn print(&self) {
        println!("barrel of monkeys");
        println!("monkey business: {}", self.business());
        for i in 0..self.monkeys.len() {
            println!("monkey {}: {}", i, self.monkeys[i]);
        }
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let mut throw_queue: VecDeque<i32> = VecDeque::new();
            let mut test: Option<Test> = None;
            let mut true_target: usize = 0;
            let mut false_target: usize = 0;

            if let monkey = &mut self.monkeys[i] {
                test = monkey.test;
                true_target = monkey.true_target;
                false_target = monkey.false_target;

                while let Some(item) = monkey.items.pop_front() {
                    let value = monkey.inspect(item);

                    throw_queue.push_back(value);    
                }
            }

            while let Some(value) = throw_queue.pop_front() {
                if test.unwrap().test(value) {
                    self.monkeys[true_target].items.push_back(value);
                } else {
                    self.monkeys[false_target].items.push_back(value);
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    formula: Option<Operation>,
    test: Option<Test>,
    true_target: usize,
    false_target: usize,
    inspect_count: i32,
}

impl Monkey {

    fn new() -> Self {
        return Monkey {
            items: VecDeque::new(),
            formula: None,
            test: None,
            true_target: 0,
            false_target: 0,
            inspect_count: 0,
        };
    }

    fn inspect(&mut self, item: i32) -> i32 {
        let mut result = self.formula.unwrap().calc(item);
        self.inspect_count += 1;

        result = result / 3;

        return result;
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result_str = "".to_owned();
        result_str.push_str("inspections: ");
        result_str.push_str(self.inspect_count.to_string().as_str());
        result_str.push_str(", items: ");
        for i in 0..self.items.len() {
            if i != 0 {
                result_str.push_str(", ");
            }
            result_str.push_str(self.items[i].to_string().as_str());
        }
        return write!(f, "{}", result_str);
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Operation {
    left_base: Base,
    left_value: i32,
    right_base: Base,
    right_value: i32,
    function: Type,
}

impl Operation {
    fn calc(&self, value: i32) -> i32 {
        let left: i32;
        if self.left_base == Base::Old {
            left = value;
        } else if self.left_base == Base::Val {
            left = self.left_value;
        } else {
            left = 0;
        }

        let right: i32;
        if self.right_base == Base::Old {
            right = value;
        } else if self.right_base == Base::Val {
            right = self.right_value;
        } else {
            right = 0;
        }

        let mut result: i32 = 0;

        match self.function {
            Type::Add => result = left + right,
            Type::Subtract => result = left - right,
            Type::Multiply => result = left * right,
            Type::Divide => result = left / right,
            Type::Modulo => result = left % right,
            Type::None => panic!("Should never get here"),
        }

        return result;
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Type {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    None,
}

#[derive(PartialEq, Clone, Copy)]
enum Base {
    New,
    Old,
    Val,
    None,
} 

#[derive(PartialEq, Clone, Copy)]
struct Test {
    operation: Operation,
    test: i32,
}

impl Test {
    fn test(&self, value: i32) -> bool {
        let result = self.operation.calc(value);
        return result == self.test;
    }
}

fn main () {

    let mut barrel: BarrelOfMonkeys = BarrelOfMonkeys::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let mut monkey_line: i32 = 0;
    let mut monkey = Monkey::new();

    let filename: &str = &args[1];

    let lines = if let Ok(lines) = read_lines(&filename) { lines } else { panic!("Should never happen!") };
    for ip in lines {
        if let Ok(line) = ip {
            match monkey_line {
                0 => {
                    println!("parsing {}", line);
                },
                1 => {
                    let items = line.split(" ");

                    for (i, item) in items.enumerate() {
                        if i > 3 {
                            let value = item.replace(",", "").parse::<i32>().unwrap();
                            monkey.items.push_back(value);
                        }
                    }
                },
                2 => {
                    let items = line.split(" ");

                    let mut left_base: Base = Base::None;
                    let mut left_value: i32 = 0;
                    let mut right_base: Base = Base::None;
                    let mut right_value: i32 = 0;
                    let mut operation: Type = Type::None;

                    for (i, item) in items.enumerate() {
                        if i == 5 {
                            match item {
                                "old" => left_base = Base::Old,
                                "new" => left_base = Base::New,
                                _     => left_base = Base::Val,
                            }

                            match left_base {
                                Base::Val => left_value = item.parse::<i32>().unwrap(),
                                _ => (),
                            }
                        } else if i == 6 {
                            match item {
                                "+" => operation = Type::Add,
                                "-" => operation = Type::Subtract,
                                "*" => operation = Type::Multiply,
                                "/" => operation = Type::Divide,
                                "%" => operation = Type::Modulo,
                                _   => panic!("Should never get here"),
                            }
                        } else if i == 7 {
                            match item {
                                "old" => right_base = Base::Old,
                                "new" => right_base = Base::New,
                                _     => right_base = Base::Val,
                            }

                            match right_base {
                                Base::Val => right_value = item.parse::<i32>().unwrap(),
                                _ => (),
                            }
                        }
                    }

                    let op: Operation = Operation {
                        left_base,
                        left_value,
                        right_base,
                        right_value,
                        function: operation,
                    };

                    monkey.formula = Some(op);

                },
                3 => {
                    let items = line.split(" ");

                    let mut op: Operation = Operation {
                        left_base: Base::Old,
                        left_value: 0,
                        right_base: Base::Val,
                        right_value: 0,
                        function: Type::None
                    };

                    for (i, item) in items.enumerate() {
                        if i == 3 {
                            match item {
                                "divisible" => op.function = Type::Modulo,
                                _ => println!("Didn't prepare handling for this"),
                            }
                        } else if i == 5 {
                            op.right_value = item.parse::<i32>().unwrap();
                        }
                    }

                    let test: Test = Test {
                        operation: op,
                        test: 0,
                    };

                    monkey.test = Some(test);
                },
                4 => {
                    let items = line.split(" ");

                    for (i, item) in items.enumerate() {
                        if i == 9 {
                            monkey.true_target = item.parse::<usize>().unwrap();
                        }
                    }
                },
                5 => {
                    let items = line.split(" ");

                    for (i, item) in items.enumerate() {
                        if i == 9 {
                            monkey.false_target = item.parse::<usize>().unwrap();
                        }
                    }

                    barrel.push(monkey.clone());
                },
                _ => {
                    monkey = Monkey::new();
                    monkey_line = 0;
                    continue;
                },
            }

            monkey_line += 1;
        }
    }

    barrel.print();

    let rounds = 20;
    for i in 0..rounds {
        println!("round: {}", i);

        barrel.round();
        barrel.print();
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}