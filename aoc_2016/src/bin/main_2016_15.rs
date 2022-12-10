use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Disc {
    nb_pos: u64,
    start_pos: u64,
}

fn main() {
    let mut file = File::open("inputs/input_2016_15.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut time: u64 = 0;
    let mut all_discs: Vec<Disc> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(' ').collect();
            let mut pos = split_line[11].to_string();
            pos.pop();
            Disc {
                nb_pos: split_line[3].parse::<u64>().unwrap(),
                start_pos: pos.parse::<u64>().unwrap(),
            }
        })
        .collect();

    // first part
    loop {
        if all_discs
            .iter()
            .enumerate()
            .all(|(i, &d)| (d.start_pos + i as u64 + 1 + time) % d.nb_pos == 0)
        {
            println!("{time} is the first time you can press the button");
            break;
        }

        time += 1;
    }

    // second part
    all_discs.push(Disc {
        nb_pos: 11,
        start_pos: 0,
    });
    time = 0;
    loop {
        if all_discs
            .iter()
            .enumerate()
            .all(|(i, &d)| (d.start_pos + i as u64 + 1 + time) % d.nb_pos == 0)
        {
            println!("{time} is now the first time you can press the button");
            break;
        }

        time += 1;
    }
}
