use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_03.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let priorities: Vec<u16> = data
        .lines()
        .map(|line| {
            let half_size = line.len() / 2;
            let (first_part, last_part) = line.split_at(half_size);
            compare_letters(first_part, last_part)
        })
        .collect();

    println!(
        "the sum of the priorities of those item types is {}",
        priorities.iter().sum::<u16>()
    );

    // second part
    let elves: Vec<&str> = data.lines().collect_vec();
    let elves_grouped: Vec<u16> = elves
        .chunks(3)
        .map(|chunk| compare_three_elves(chunk[0], chunk[1], chunk[2]))
        .collect();

    println!(
        "the new sum of the priorities of those item types is {}",
        elves_grouped.iter().sum::<u16>()
    );
}

fn compare_letters(first_letters: &str, second_letters: &str) -> u16 {
    let found_letter = first_letters
        .chars()
        .find(|letter| second_letters.contains(*letter))
        .unwrap();

    if found_letter as u16 > 96 {
        found_letter as u16 - 96
    } else {
        found_letter as u16 - 38
    }
}

fn compare_three_elves(first_letters: &str, second_letters: &str, third_letters: &str) -> u16 {
    let found_letter = first_letters
        .chars()
        .find(|letter| second_letters.contains(*letter) && third_letters.contains(*letter))
        .unwrap();

    if found_letter as u16 > 96 {
        found_letter as u16 - 96
    } else {
        found_letter as u16 - 38
    }
}
