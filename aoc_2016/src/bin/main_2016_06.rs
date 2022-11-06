use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const PWD_SIZE: usize = 8;

fn main() {
    let mut file = File::open("inputs/input_2016_06.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_data: [HashMap<char, u16>; PWD_SIZE] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    data.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(index, c)| {
            let current_value = all_data[index].get(&c).unwrap_or_else(|| &1);
            all_data[index].insert(c, current_value + 1);
        });
    });

    // first part
    let password: Vec<char> = all_data
        .iter()
        .map(|hm| {
            hm.iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| *k)
                .unwrap()
        })
        .collect();

    println!(
        "the error-corrected version of the message being sent is: {:?}",
        password.iter().collect::<String>()
    );

    // second part
    let new_password: Vec<char> = all_data
        .iter()
        .map(|hm| {
            hm.iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| *k)
                .unwrap()
        })
        .collect();

    println!(
        "the original message that Santa is trying to send is: {:?}",
        new_password.iter().collect::<String>()
    );
}
