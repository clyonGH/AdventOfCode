use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

fn main() {
    let mut file = File::open("inputs/input_2016_20.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_ranges: Vec<Range> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split('-').collect();
            Range {
                min: split_line[0].parse::<u64>().unwrap(),
                max: split_line[1].parse::<u64>().unwrap(),
            }
        })
        .collect();
    all_ranges.sort_by(|a, b| a.min.cmp(&b.min));

    let mut new_all_ranges = all_ranges.clone();

    // first part
    let mut min: u64 = 0;
    loop {
        let start_len: usize = all_ranges.len();
        let mut i_to_remove: Vec<usize> = Vec::new();
        (0..all_ranges.len()).for_each(|i| {
            if all_ranges[i].min <= min {
                if all_ranges[i].max >= min {
                    min = all_ranges[i].max + 1;
                }
                i_to_remove.push(i);
            }
        });

        i_to_remove.reverse();
        i_to_remove.into_iter().for_each(|i| {
            all_ranges.remove(i);
        });

        if start_len == all_ranges.len() {
            println!("the lowest-valued IP that is not blocked is {min}");
            break;
        }
    }

    // second part
    new_all_ranges.sort_by(|a, b| a.min.cmp(&b.min));
    new_all_ranges.reverse();
    let mut ips_allowed: u64 = 0;
    min = 0;
    while !new_all_ranges.is_empty() {
        let new_range = new_all_ranges.pop().unwrap();
        if new_range.min <= min {
            if new_range.max >= min {
                min = new_range.max + 1;
            }
        } else {
            ips_allowed += new_range.min - min;
            min = new_range.max + 1;
        }
    }

    println!("{ips_allowed} IPs are allowed by the blacklist");
}
