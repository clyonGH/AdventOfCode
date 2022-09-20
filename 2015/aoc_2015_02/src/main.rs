use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Error: File not found");
    let data = BufReader::new(file);

    let mut data_vector: Vec<Vec<u32>> = vec![];

    for line in data.lines() {
        let mut split_line: Vec<u32> = line?
            .split("x")
            .map(|elt_str| elt_str.parse::<u32>().unwrap())
            .collect();
        split_line.sort();
        data_vector.push(split_line);
    }

    let first_part_result: u32 = data_vector
        .iter()
        .map(|v| 3 * v[0] * v[1] + 2 * (v[0] * v[2] + v[1] * v[2]))
        .sum();

    println!("{:?}", first_part_result);

    let second_part_result: u32 = data_vector
        .iter()
        .map(|v| 2 * (v[0] + v[1]) + v[0] * v[1] * v[2])
        .sum();

    println!("{:?}", second_part_result);

    Ok(())
}