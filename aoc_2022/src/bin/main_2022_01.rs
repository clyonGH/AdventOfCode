use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_01.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut cal_sum: u32 = 0;

    let all_cal_sums: Vec<u32> = data
        .lines()
        .filter_map(|cal| {
            let parsed_cal = cal.parse::<u32>();
            if parsed_cal.is_ok() {
                cal_sum += parsed_cal.unwrap();
                None
            } else {
                let final_sum = cal_sum;
                cal_sum = 0;
                Some(final_sum)
            }
        })
        .collect();

    // first part
    println!(
        "the elf carrying the most calories carries a total of {:?} calories",
        all_cal_sums.iter().max().unwrap()
    );

    // second part
    println!(
        "the top three elves carrying the most calories carry a total of {:?} calories",
        all_cal_sums.iter().sorted().rev().take(3).sum::<u32>()
    );
}
