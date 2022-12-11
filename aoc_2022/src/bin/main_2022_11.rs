use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Operation {
    Add(u128),
    Multiply(Option<u128>),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: u128,
    next_true: usize,
    next_false: usize,
    business: u128,
}

impl Monkey {
    fn operate(&self, old: u128) -> u128 {
        match self.operation {
            Operation::Add(val) => old + val,
            Operation::Multiply(val) => old * val.unwrap_or(old),
        }
    }

    fn test(&self, val: u128) -> (usize, u128) {
        let new_val = val / 3;
        if new_val % self.test == 0 {
            (self.next_true, new_val)
        } else {
            (self.next_false, new_val)
        }
    }

    fn new_test(&self, val: u128, all_prods: u128) -> (usize, u128) {
        let new_val = val % all_prods;
        if new_val % self.test == 0 {
            (self.next_true, new_val)
        } else {
            (self.next_false, new_val)
        }
    }
}

fn main() {
    let mut file = File::open("inputs/input_2022_11.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_info: Vec<&str> = data
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if i != 0 && i % 7 != 6 && i % 7 != 0 {
                Some(line)
            } else {
                None
            }
        })
        .collect();

    let mut monkey_info: Vec<Monkey> = Vec::new();
    all_info.reverse();
    while !all_info.is_empty() {
        let mut info = all_info.pop().unwrap();
        // items
        let mut split_info: Vec<&str> = info.split([',', ' ']).collect();
        split_info.reverse();
        let mut items: Vec<u128> = split_info
            .iter()
            .filter_map(|&item| {
                if item.parse::<u128>().is_ok() {
                    Some(item.parse::<u128>().unwrap())
                } else {
                    None
                }
            })
            .collect();
        items.reverse();

        // operation
        info = all_info.pop().unwrap();
        split_info = info.split_whitespace().collect();
        let operation: Operation = match split_info[4] {
            "*" => {
                if let Ok(val) = split_info[5].parse::<u128>() {
                    Operation::Multiply(Some(val))
                } else {
                    Operation::Multiply(None)
                }
            }
            _ => Operation::Add(split_info[5].parse::<u128>().unwrap()),
        };

        // test
        info = all_info.pop().unwrap();
        split_info = info.split_whitespace().collect();
        let test: u128 = split_info[3].parse().unwrap();

        // if true
        info = all_info.pop().unwrap();
        split_info = info.split_whitespace().collect();
        let next_true: usize = split_info[5].parse().unwrap();

        // if false
        info = all_info.pop().unwrap();
        split_info = info.split_whitespace().collect();
        let next_false: usize = split_info[5].parse().unwrap();

        // new monkey created
        monkey_info.push(Monkey {
            items,
            operation,
            test,
            next_true,
            next_false,
            business: 0,
        });
    }

    // first part
    let mut first_monkey_info = monkey_info.clone();
    (0..20).for_each(|_| {
        (0..first_monkey_info.len()).for_each(|m_index| {
            (0..first_monkey_info[m_index].items.len()).for_each(|i| {
                let mut val = first_monkey_info[m_index].items[i];
                val = first_monkey_info[m_index].operate(val);
                let (new_monkey, new_val) = first_monkey_info[m_index].test(val);
                first_monkey_info[new_monkey].items.push(new_val);
            });

            first_monkey_info[m_index].business += first_monkey_info[m_index].items.len() as u128;
            first_monkey_info[m_index].items.clear();
        });
    });

    let mut m_business: Vec<u128> = first_monkey_info
        .iter()
        .map(|m| m.clone().business)
        .collect();

    m_business.sort();
    m_business.reverse();

    println!(
        "the level of monkey business after 20 rounds is {:?}",
        m_business[0] * m_business[1]
    );

    // second part
    let all_tests: Vec<u128> = monkey_info.iter().map(|m| m.clone().test).collect();
    let all_prods: u128 = all_tests.iter().product();
    (0..10000).for_each(|_| {
        (0..monkey_info.len()).for_each(|m_index| {
            (0..monkey_info[m_index].items.len()).for_each(|i| {
                let mut val = monkey_info[m_index].items[i];
                val = monkey_info[m_index].operate(val);
                let (new_monkey, new_val) = monkey_info[m_index].new_test(val, all_prods);
                monkey_info[new_monkey].items.push(new_val);
            });

            monkey_info[m_index].business += monkey_info[m_index].items.len() as u128;
            monkey_info[m_index].items.clear();
        });
    });

    m_business = monkey_info.iter().map(|m| m.clone().business).collect();

    m_business.sort();
    m_business.reverse();

    println!(
        "the level of monkey business after 10,000 rounds is {:?}",
        m_business[0] * m_business[1]
    );
}
