use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_04.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let split_pairs: Vec<(u16, u16, u16, u16)> = data
        .lines()
        .map(|line| {
            let split_pair: Vec<&str> = line.split(&[',', '-']).collect();

            (
                split_pair[0].parse::<u16>().unwrap(),
                split_pair[1].parse::<u16>().unwrap(),
                split_pair[2].parse::<u16>().unwrap(),
                split_pair[3].parse::<u16>().unwrap(),
            )
        })
        .collect();

    // first part
    let nb_pairs: Vec<&(u16, u16, u16, u16)> = split_pairs
        .iter()
        .filter(|(e1min, e1max, e2min, e2max)| {
            (e1min <= e2min && e1max >= e2max) || (e2min <= e1min && e2max >= e1max)
        })
        .collect();

    println!(
        "{:?} assignment pairs have one range fully containing the other",
        nb_pairs.iter().count()
    );

    // second part
    let overlapping_pairs: Vec<&(u16, u16, u16, u16)> = split_pairs
        .iter()
        .filter(|(e1min, e1max, e2min, e2max)| {
            (e1max >= e2min && e1min <= e2max) || (e2max >= e1min && e2min <= e1max)
        })
        .collect();

    println!(
        "{:?} assignment pairs have ranges that overlap",
        overlapping_pairs.iter().count()
    );
}
