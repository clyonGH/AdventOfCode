use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_06.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let input: Vec<char> = data.chars().collect();

    // first part
    let res = input.windows(4).enumerate().find(|&(_, cs)| {
        let mut c_hs: HashSet<char> = HashSet::new();
        cs.iter().all(|&c| c_hs.insert(c))
    });

    println!(
        "the first start-of-packet marker is detected after {:?} characters",
        res.unwrap().0 + 4
    );

    // second part
    let new_res = input.windows(14).enumerate().find(|&(_, cs)| {
        let mut c_hs: HashSet<char> = HashSet::new();
        cs.iter().all(|&c| c_hs.insert(c))
    });

    println!(
        "the first start-of-message marker is detected after {:?} characters",
        new_res.unwrap().0 + 14
    );
}
