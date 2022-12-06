use std::fs::File;
use std::io::prelude::*;

const INIT_STACK_SIZE: usize = 9;
const NB_STACKS: usize = 9;
const START_INSTR: usize = 10;

fn main() {
    let mut file = File::open("inputs/input_2022_05.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let transp_crates: Vec<Vec<char>> = data
        .lines()
        .enumerate()
        .map(|(line_nb, line)| {
            let mut temp_vec: Vec<char> = Vec::new();
            if line_nb < INIT_STACK_SIZE {
                line.chars().enumerate().for_each(|(index, c)| {
                    if (index - 1) % 4 == 0 {
                        if c != ' ' {
                            temp_vec.push(c);
                        } else {
                            temp_vec.push(' ');
                        }
                    }
                });
            }
            temp_vec
        })
        .collect();

    let mut all_crates: Vec<Vec<char>> = vec![Vec::new(); NB_STACKS];
    transp_crates
        .iter()
        .enumerate()
        .for_each(|(tc_index, all_cr)| {
            if tc_index < INIT_STACK_SIZE - 1 {
                all_cr.iter().enumerate().for_each(|(ac_index, &cr)| {
                    if cr != ' ' {
                        all_crates[ac_index].push(cr);
                    }
                });
            }
        });

    all_crates.iter_mut().for_each(|cr| {
        cr.reverse();
    });

    let mut all_crates_sec = all_crates.clone();

    // first part
    data.lines().enumerate().for_each(|(line_nb, line)| {
        if line_nb >= START_INSTR {
            let split_line: Vec<&str> = line.split(' ').collect();
            let nb_crates = split_line[1].parse::<usize>().unwrap();
            let src_stack = split_line[3].parse::<usize>().unwrap();
            let dest_stack = split_line[5].parse::<usize>().unwrap();

            (0..nb_crates).for_each(|_| {
                let crate_to_move = all_crates[src_stack - 1].pop().unwrap();
                all_crates[dest_stack - 1].push(crate_to_move);
            });
        }
    });

    let ans1: String = all_crates
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();
    println!("{ans1} are the crates that end up on the top of each stack");

    // second part
    data.lines().enumerate().for_each(|(line_nb, line)| {
        if line_nb >= START_INSTR {
            let split_line: Vec<&str> = line.split(' ').collect();
            let nb_crates = split_line[1].parse::<usize>().unwrap();
            let src_stack = split_line[3].parse::<usize>().unwrap();
            let dest_stack = split_line[5].parse::<usize>().unwrap();

            let mut crate_to_move: Vec<char> = Vec::new();
            (0..nb_crates).for_each(|_| {
                crate_to_move.push(all_crates_sec[src_stack - 1].pop().unwrap());
            });
            crate_to_move.reverse();
            crate_to_move.iter().for_each(|&cr| {
                all_crates_sec[dest_stack - 1].push(cr);
            });
        }
    });

    let ans2: String = all_crates_sec
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();
    println!("after the rearrangement {ans2} are the crates that end up on the top of each stack");
}
