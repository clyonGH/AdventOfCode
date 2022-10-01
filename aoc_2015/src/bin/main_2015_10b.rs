use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_10.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let input: Vec<u8> = data
        .chars()
        .into_iter()
        .map(|each_char| each_char.to_digit(10).unwrap() as u8)
        .collect();

    println!("length of result: {}", look_and_say(input, 50).len());
}

fn look_and_say(previous: Vec<u8>, nb_iter: u8) -> Vec<u8> {
    let mut new_line: Vec<u8> = previous;
    for n in 0..nb_iter {
        new_line = following_line(new_line);
        println!("{}", n);
    }

    new_line
}

fn following_line(previous: Vec<u8>) -> Vec<u8> {
    let mut new_line: Vec<u8> = Vec::new();
    let mut prev_c: u8 = 0;
    let mut count_c: u8 = 0;
    for c in previous.into_iter() {
        if prev_c == 0 {
            count_c = 1;
        } else if c != prev_c {
            new_line.push(count_c);
            new_line.push(prev_c);
            count_c = 1;
        } else {
            count_c += 1;
        }
        prev_c = c;
    }
    new_line.push(count_c);
    new_line.push(prev_c);
    new_line
}
