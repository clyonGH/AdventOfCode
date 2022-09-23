use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Error: File not found");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut coord_reached: HashSet<(i32, i32)> = HashSet::new();

    // (E/W, N/S)
    coord_reached.insert((0, 0));

    let mut coord_current: (i32, i32) = (0, 0);

    for instruction in data.chars() {
        match instruction {
            '^' => coord_current = (coord_current.0, coord_current.1 + 1),
            'v' => coord_current = (coord_current.0, coord_current.1 - 1),
            '<' => coord_current = (coord_current.0 - 1, coord_current.1),
            '>' => coord_current = (coord_current.0 + 1, coord_current.1),
            _ => (),
        }

        coord_reached.insert(coord_current);
    }

    println!("number of houses reached: {:?}", coord_reached.len());
}
