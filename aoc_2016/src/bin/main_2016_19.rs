use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2016_19.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut nb_elves = data.parse::<usize>().unwrap();

    let mut all_elves: Vec<usize> = Vec::new();
    (0..nb_elves).for_each(|i| all_elves.push(i + 1));

    // first part
    while nb_elves != 1 {
        (0..nb_elves).for_each(|i| {
            if i % 2 != 0 {
                all_elves[i] = 0;
            }
        });

        if nb_elves % 2 != 0 {
            all_elves[0] = 0;
        }

        let new_elves: Vec<usize> = all_elves
            .clone()
            .into_iter()
            .filter(|&val| val != 0)
            .collect();

        all_elves = new_elves;
        nb_elves = all_elves.len();
    }
    println!("the winning elf is {:?}", all_elves[0]);

    // second part
    nb_elves = data.parse::<usize>().unwrap();
    all_elves.clear();
    (0..nb_elves).for_each(|i| all_elves.push(i + 1));
    let mut i = 0;

    while nb_elves != 1 {
        if i >= nb_elves {
            i = 0
        }

        let half_elves = nb_elves / 2;
        let new_i = (i + half_elves) % nb_elves;
        all_elves.remove(new_i);
        nb_elves -= 1;

        if new_i > i {
            i += 1;
        }
    }
    println!("the winning elf is now {:?}", all_elves[0]);
}
