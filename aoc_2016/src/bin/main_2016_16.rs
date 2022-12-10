use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

const DISC_1_SIZE: usize = 272;
const DISC_2_SIZE: usize = 35651584;

fn main() {
    let mut file = File::open("inputs/input_2016_16.txt").expect("Error: File not found");
    let mut a: String = String::new();

    file.read_to_string(&mut a)
        .expect("Error while reading file");

    // first part
    let mut data = a.clone();
    while data.len() < DISC_1_SIZE {
        let b: String = dragon_curve(&data).into_iter().collect();
        data += "0";
        data += &b;
    }

    let mut checksum: String = get_checksum(&data[..DISC_1_SIZE]).into_iter().collect();
    while checksum.len() % 2 == 0 {
        let old_checksum = checksum;
        checksum = get_checksum(&old_checksum).into_iter().collect();
    }

    println!(
        "the correct checksum for a disc of size {:?} is: {:?}",
        DISC_1_SIZE, checksum
    );

    // second part
    data = a;
    while data.len() < DISC_2_SIZE {
        let b: String = dragon_curve(&data).into_iter().collect();
        data += "0";
        data += &b;
    }

    checksum = get_checksum(&data[..DISC_2_SIZE]).into_iter().collect();
    while checksum.len() % 2 == 0 {
        let old_checksum = checksum;
        checksum = get_checksum(&old_checksum).into_iter().collect();
    }

    println!(
        "the correct checksum for a disc of size {:?} is: {:?}",
        DISC_2_SIZE, checksum
    );
}

fn dragon_curve(input: &str) -> Vec<char> {
    input
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect()
}

fn get_checksum(input: &str) -> Vec<char> {
    input
        .chars()
        .collect_vec()
        .chunks(2)
        .map(|window| if window.iter().all_unique() { '0' } else { '1' })
        .collect()
}
