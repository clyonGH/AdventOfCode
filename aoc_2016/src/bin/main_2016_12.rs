use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Instruction {
    Copy,
    Decrease,
    Increase,
    JumpIfNotZero(i16),
}

impl Instruction {
    fn compute(&self, reg: &mut i64, new_value: Option<i64>, index: &mut usize) {
        // going to the next instruction
        *index += 1;

        match self {
            Instruction::Copy => {
                *reg = new_value.unwrap();
            }
            Instruction::Decrease => *reg -= 1,
            Instruction::Increase => *reg += 1,
            Instruction::JumpIfNotZero(offset) => {
                if *reg != 0 {
                    *index += (offset - 1) as usize
                }
            }
        }
    }
}

fn main() {
    let mut file = File::open("inputs/input_2016_12.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_instr: Vec<(Instruction, &str, &str)> = data
        .lines()
        .filter_map(|line| {
            let line_instr: Vec<&str> = line.split(' ').collect();
            let instr_str = line_instr[0];
            match instr_str {
                "cpy" => Some((Instruction::Copy, line_instr[2], line_instr[1])),
                "dec" => Some((Instruction::Decrease, line_instr[1], "")),
                "inc" => Some((Instruction::Increase, line_instr[1], "")),
                "jnz" => {
                    let offset = line_instr[2].parse::<i16>().unwrap();
                    Some((Instruction::JumpIfNotZero(offset), line_instr[1], ""))
                }
                _ => None,
            }
        })
        .collect();

    // first part c = 0
    // second part c = 1
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut c: i64 = 0;
    let mut d: i64 = 0;
    let mut new_value: Option<i64> = None;
    let mut index: usize = 0;
    while (index) < all_instr.len() {
        match all_instr[index].0 {
            Instruction::Copy => {
                if all_instr[index].2.parse::<i64>().is_ok() {
                    new_value = Some(all_instr[index].2.parse::<i64>().unwrap());
                } else {
                    match all_instr[index].2 {
                        "a" => new_value = Some(a.clone()),
                        "b" => new_value = Some(b.clone()),
                        "c" => new_value = Some(c.clone()),
                        "d" => new_value = Some(d.clone()),
                        _ => (),
                    };
                }
            }
            Instruction::JumpIfNotZero(_) => {
                if all_instr[index].1.parse::<i64>().is_ok() {
                    new_value = Some(all_instr[index].1.parse::<i64>().unwrap());
                }
            }
            _ => new_value = None,
        }

        match all_instr[index].1 {
            "a" => all_instr[index].0.compute(&mut a, new_value, &mut index),
            "b" => all_instr[index].0.compute(&mut b, new_value, &mut index),
            "c" => all_instr[index].0.compute(&mut c, new_value, &mut index),
            "d" => all_instr[index].0.compute(&mut d, new_value, &mut index),
            _ => all_instr[index]
                .0
                .compute(&mut new_value.unwrap(), None, &mut index), // case for jump if not zero where an int is checked instead of a reg
        };
    }

    println!("the value in register a when the program is finished executing is: {a}");
}
