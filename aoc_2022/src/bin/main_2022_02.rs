use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_02.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let total_scores: Vec<u16> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(' ').collect();
            let mut total_score: u16 = 0;
            match (split_line[0], split_line[1]) {
                // draw
                ("A", "X") | ("B", "Y") | ("C", "Z") => total_score = 3,
                // you win
                ("C", "X") | ("A", "Y") | ("B", "Z") => total_score = 6,
                // you lose
                _ => (),
            };

            match split_line[1] {
                // rock
                "X" => total_score += 1,
                // paper
                "Y" => total_score += 2,
                // scissors
                "Z" => total_score += 3,
                _ => (),
            };

            total_score
        })
        .collect();

    println!(
        "the total score is {:?} if everything goes exactly according to the strategy guide",
        total_scores.iter().sum::<u16>()
    );

    // second part
    let new_total_scores: Vec<u16> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(' ').collect();
            let mut total_score: u16 = 0;
            match split_line[1] {
                // draw
                "Y" => {
                    total_score = 3;
                    match split_line[0] {
                        "A" => total_score += 1,
                        "B" => total_score += 2,
                        "C" => total_score += 3,
                        _ => (),
                    }
                }
                // you win
                "Z" => {
                    total_score = 6;
                    match split_line[0] {
                        "A" => total_score += 2,
                        "B" => total_score += 3,
                        "C" => total_score += 1,
                        _ => (),
                    }
                }
                // you lose
                _ => match split_line[0] {
                    "A" => total_score = 3,
                    "B" => total_score = 1,
                    "C" => total_score = 2,
                    _ => (),
                },
            };

            total_score
        })
        .collect();

    println!(
        "the new total score is {:?} if everything goes exactly according to the strategy guide",
        new_total_scores.iter().sum::<u16>()
    );
}
