use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Error: File not found");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // FIRST OPTION :)
    let mut floor: i16 = 0;
    let mut basement_found = false;

    for (input_index, input_char) in data.chars().enumerate() {
        match input_char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 && !basement_found {
            basement_found = true;
            println!("basement reached at position: {}", input_index + 1);
        }
    }

    println!("final floor: {}\n", floor);
}
