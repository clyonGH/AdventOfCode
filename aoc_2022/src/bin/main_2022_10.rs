use std::fs::File;
use std::io::prelude::*;

const MAX_CYCLE: usize = 240;

enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    fn compute(&self, signals: &mut Vec<i64>, x: &mut i64, index: &mut usize) {
        // going to the next instruction
        *index += 1;

        print_signal(*x, signals.len());
        signals.push(*x);
        match self {
            &Instruction::Addx(val) => {
                print_signal(*x, signals.len());
                signals.push(*x);
                *x += val;
            }
            _ => (),
        }
    }
}

fn main() {
    let mut file = File::open("inputs/input_2022_10.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_instr: Vec<Instruction> = data
        .lines()
        .flat_map(|line| {
            let line_instr: Vec<&str> = line.split(' ').collect();
            let instr_str = line_instr[0];
            match instr_str {
                "noop" => Some(Instruction::Noop),
                "addx" => {
                    let val = line_instr[1].parse::<i64>().unwrap();
                    Some(Instruction::Addx(val))
                }
                _ => None,
            }
        })
        .collect();

    // first part
    let mut x: i64 = 1;
    let mut index: usize = 0;
    let mut signals: Vec<i64> = Vec::new();

    while signals.len() < MAX_CYCLE {
        all_instr[index].compute(&mut signals, &mut x, &mut index);
    }

    let sig_strength: i64 = signals[19] * 20
        + signals[59] * 60
        + signals[99] * 100
        + signals[139] * 140
        + signals[179] * 180
        + signals[219] * 220;

    println!("the sum of these six signal strengths is {sig_strength}");
}

// second part
fn print_signal(x: i64, index: usize) {
    let i_cycle = index % 40;
    if (x - 1..x + 2).contains(&(i_cycle as i64)) {
        print!("#");
    } else {
        print!(" ");
    }

    if i_cycle == 39 {
        println!();
    }
}
