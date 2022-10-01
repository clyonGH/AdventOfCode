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
    if nb_iter == 0 {
        previous
    } else {
        // fold seems to slow calculations
        let mut truple = look_and_say(previous, nb_iter - 1)
            .iter()
            .fold((Vec::new(), 0, 0), find_answer);

        // adding last computed digit
        println!("{}", nb_iter);
        truple.0.push(truple.1);
        truple.0.push(truple.2);
        truple.0
    }
}

fn find_answer(acc: (Vec<u8>, u8, u8), &current_u8: &u8) -> (Vec<u8>, u8, u8) {
    match acc.2 {
        0 => (acc.0, 1, current_u8), // initial case
        n if n == current_u8 => (acc.0, acc.1 + 1, n),
        _ => {
            let mut new_acc = acc.0.clone();
            new_acc.push(acc.1);
            new_acc.push(acc.2);
            (new_acc, 1, current_u8)
        }
    }
}
