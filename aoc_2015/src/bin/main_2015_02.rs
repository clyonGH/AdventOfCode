use std::fs::File;
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    let mut file = File::open("inputs/input_2015_02.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut data_vector: Vec<Vec<u32>> = vec![];

    for line in data.lines() {
        let mut split_line: Vec<u32> = line
            .split("x")
            .map(|elt_str| elt_str.parse::<u32>().unwrap())
            .collect();
        split_line.sort();
        data_vector.push(split_line);
    }

    // first part
    let first_part_result: u32 = data_vector
        .iter()
        .map(|v| 3 * v[0] * v[1] + 2 * (v[0] * v[2] + v[1] * v[2]))
        .sum();

    println!(
        "total square feet of wrapping paper: {:?}",
        first_part_result
    );

    // second part
    let second_part_result: u32 = data_vector
        .iter()
        .map(|v| 2 * (v[0] + v[1]) + v[0] * v[1] * v[2])
        .sum();

    println!("total feet of ribbon: {:?}", second_part_result);

    Ok(())
}
