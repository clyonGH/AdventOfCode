use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Instruction {
    Increase,
    Triple,
    Half,
    Jump(i8),
    JumpIfOne(i8),
    JumpIfEven(i8),
}

impl Instruction {
    fn compute(&self, reg: &mut u64, index: &mut i8) {
        // going to the next instruction
        *index += 1;

        match self {
            Instruction::Increase => *reg += 1,
            Instruction::Triple => *reg *= 3,
            Instruction::Half => *reg /= 2,
            Instruction::Jump(offset) => *index += offset - 1,
            Instruction::JumpIfOne(offset) => {
                if *reg == 1 {
                    *index += offset - 1
                }
            }
            Instruction::JumpIfEven(offset) => {
                if *reg % 2 == 0 {
                    *index += offset - 1
                }
            }
        }
    }
}

fn main() {
    let mut file = File::open("inputs/input_2015_23.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_instr: Vec<(Instruction, &str)> = data
        .lines()
        .filter_map(|line| {
            let line_instr: Vec<&str> = line.split(' ').collect();
            let instr_str = line_instr[0];
            match instr_str {
                "inc" => Some((Instruction::Increase, line_instr[1])),
                "tpl" => Some((Instruction::Triple, line_instr[1])),
                "hlf" => Some((Instruction::Half, line_instr[1])),
                "jmp" => {
                    let offset = line_instr[1].parse::<i8>().unwrap();
                    Some((Instruction::Jump(offset), "a"))
                }
                "jio" => {
                    let offset = line_instr[2].parse::<i8>().unwrap();
                    Some((
                        Instruction::JumpIfOne(offset),
                        line_instr[1].strip_suffix(',').unwrap(),
                    ))
                }
                "jie" => {
                    let offset = line_instr[2].parse::<i8>().unwrap();
                    Some((
                        Instruction::JumpIfEven(offset),
                        line_instr[1].strip_suffix(',').unwrap(),
                    ))
                }
                _ => None,
            }
        })
        .collect();

    // part two has a set to 1 at the beginning
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut index: i8 = 0;
    while (index as usize) < all_instr.len() {
        match all_instr[index as usize].1 {
            "a" => all_instr[index as usize].0.compute(&mut a, &mut index),
            "b" => all_instr[index as usize].0.compute(&mut b, &mut index),
            _ => (),
        };
    }

    println!("the value in register b when the program is finished executing is: {b}");
}
