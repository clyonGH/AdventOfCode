use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_20.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let nb_presents: u64 = data.parse::<u64>().unwrap();

    // first part
    let mut house: u64 = 0;
    loop {
        house += 1;

        let div_found = divisors::get_divisors(house);
        if div_found.iter().sum::<u64>() + 1 + house >= (nb_presents / 10) {
            break;
        }
    }

    println!("house {house} is the lowest house to get {nb_presents} presents");

    // second part
    house = 0;
    loop {
        house += 1;

        let div_found = divisors::get_divisors(house);
        let presents = div_found
            .iter()
            .filter(|&elf| house <= elf * 50)
            .sum::<u64>();

        if (presents + house) * 11 >= nb_presents {
            break;
        }
    }

    println!("with these changes, house {house} is the lowest house to get {nb_presents} presents");
}
