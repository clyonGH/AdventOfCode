use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_25.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let split_data: Vec<&str> = data.split(' ').collect();
    let row: u64 = split_data[15].strip_suffix(',').unwrap().parse().unwrap();
    let column: u64 = split_data[17].strip_suffix('.').unwrap().parse().unwrap();

    // get the nth code to find
    let first_row = (1..=column).sum::<u64>();
    let mut increment = column;
    let mut final_row: u64 = first_row;

    (1..row).for_each(|_| {
        final_row += increment;
        increment += 1;
    });

    println!("I have to find the {:?}th code", final_row);

    // getting the nth code value
    let mut code: u64 = 20151125;
    (1..final_row).for_each(|_| {
        code *= 252533;
        code %= 33554393;
    });

    println!("the machine gives me the code {:?}", code);
}
