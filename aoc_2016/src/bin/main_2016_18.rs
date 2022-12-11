use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2016_18.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_tiles: Vec<Vec<usize>> = Vec::new();
    let first_line: Vec<usize> = data.chars().map(|c| if c == '.' { 1 } else { 0 }).collect();
    all_tiles.push(first_line.clone());
    let mut safe_tiles: usize = first_line.iter().sum::<usize>();

    // second part: 400 000 rows
    (0..399999).for_each(|i| {
        let mut next_row: Vec<usize> = all_tiles[i]
            .windows(3)
            .map(
                |prev_tiles| match (prev_tiles[0], prev_tiles[1], prev_tiles[2]) {
                    (0, 0, 1) | (1, 0, 0) | (0, 1, 1) | (1, 1, 0) => 0,
                    _ => 1,
                },
            )
            .collect();

        next_row.reverse();
        match (all_tiles[i][0], all_tiles[i][1]) {
            (0, 0) | (1, 0) => next_row.push(0),
            _ => next_row.push(1),
        }

        next_row.reverse();
        match (all_tiles[i][98], all_tiles[i][99]) {
            (0, 0) | (0, 1) => next_row.push(0),
            _ => next_row.push(1),
        }

        all_tiles.push(next_row.clone());
        safe_tiles += next_row.iter().sum::<usize>();

        // first part: 40 rows
        if i == 38 {
            println!("in 40 rows there are {safe_tiles} safe tiles");
        }
    });

    println!("in 400,000 rows there are {safe_tiles} safe tiles");
}
