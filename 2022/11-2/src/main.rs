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

    fn business(&self) -> u64 {
        let mut monks: Vec<u64> = vec![];
        for i in 0..self.monkeys.len() {
            monks.push(self.monkeys[i].inspect_count);
        }

        monks.sort();
        monks.reverse();
        
        return monks[0] * monks[1];
    }

    fn dosage(&mut self) -> u64 {
        let mut result: u64 = 1;
        for i in 0..self.monkeys.len() {
            result *= self.monkeys[i].base();
        }

        return result;
    }

    fn print(&self) {
        println!("barrel of monkeys");
        println!("monkey business: {}", self.business());
        for i in 0..self.monkeys.len() {
            println!("monkey {}: {}", i, self.monkeys[i]);
        }
    }

    fn round(&mut self, dose: u64) {
        for i in 0..self.monkeys.len() {
            let true_target: usize;
            let false_target: usize;

            true_target = self.monkeys[i].true_target;
            false_target = self.monkeys[i].false_target;

            while let Some(item) = self.monkeys[i].items.pop_front() {
                let value = self.monkeys[i].inspect(item, dose);

                if self.monkeys[i].test(value) {
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
    items: VecDeque<u64>,
    formula: Option<Operation>,
    test: Option<Test>,
    true_target: usize,
    false_target: usize,
    inspect_count: u64,
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

    fn base(&self) -> u64 {
        return self.test.as_ref().unwrap().operation.right_value;
    }

    fn inspect(&mut self, item: u64, dose: u64) -> u64 {
        let mut result = self.formula.as_ref().unwrap().calc(item);
        self.inspect_count += 1;

        result %= dose;

        return result;
    }

    fn test(&self, value: u64) -> bool {
        match self.test {
            Some(t) => t.test(value),
            _ => panic!("test isn't set"),
        }
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
    left_value: u64,
    right_base: Base,
    right_value: u64,
    function: Type,
}

impl Operation {
    fn calc(&self, value: u64) -> u64 {
        let left: u64;
        if self.left_base == Base::Old {
            left = value;
        } else if self.left_base == Base::Val {
            left = self.left_value;
        } else {
            left = 0;
        }

        let right: u64;
        if self.right_base == Base::Old {
            right = value;
        } else if self.right_base == Base::Val {
            right = self.right_value;
        } else {
            right = 0;
        }

        match self.function {
            Type::Add => left + right,
            Type::Subtract => left - right,
            Type::Multiply => left * right,
            Type::Divide => left / right,
            Type::Modulo => left % right,
            Type::None => panic!("Should never get here"),
        }
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
    test: u64,
}

impl Test {
    fn test(&self, value: u64) -> bool {
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
                            let value = item.replace(",", "").parse::<u64>().unwrap();
                            monkey.items.push_back(value);
                        }
                    }
                },
                2 => {
                    let items = line.split(" ");

                    let mut left_base: Base = Base::None;
                    let mut left_value: u64 = 0;
                    let mut right_base: Base = Base::None;
                    let mut right_value: u64 = 0;
                    let mut operation: Type = Type::None;

                    for (i, item) in items.enumerate() {
                        if i == 5 {
                            match item {
                                "old" => left_base = Base::Old,
                                "new" => left_base = Base::New,
                                _     => left_base = Base::Val,
                            }

                            match left_base {
                                Base::Val => left_value = item.parse::<u64>().unwrap(),
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
                                Base::Val => right_value = item.parse::<u64>().unwrap(),
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
                            op.right_value = item.parse::<u64>().unwrap();
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

    let rounds = 10000;
    let dose = barrel.dosage();
    for i in 0..rounds {
        println!("round: {}", i);
        barrel.round(dose);
    }

    barrel.print();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
