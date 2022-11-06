use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const NB_MAX: usize = 5;

fn main() {
    let mut file = File::open("inputs/input_2016_04.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_rooms: Vec<[&str; 3]> = data
        .lines()
        .map(|line| {
            let re = Regex::new(r"([a-z-]*)-([0-9]*)\[([a-z]*)\]").unwrap();
            let caps = re.captures(line).unwrap();

            [
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str(),
                caps.get(3).unwrap().as_str(),
            ]
        })
        .collect();

    // first part
    let real_rooms: u32 = all_rooms
        .iter()
        .map(|&room| {
            if is_real_room(room[0], room[2]) {
                room[1].parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum();

    println!(
        "{:?} is the sum of the sector IDs of the real rooms",
        real_rooms
    );

    // second part
    all_rooms.iter().for_each(|&room| {
        compute_rot(room[0], room[1].parse::<u32>().unwrap());
    });
}

fn is_real_room(encr_name: &str, checksum: &str) -> bool {
    let room = encr_name
        .chars()
        .fold(HashMap::<char, u8>::new(), |mut acc, c| {
            if c != '-' {
                let entry = acc.entry(c).or_insert(0);
                *entry += 1;
            }
            acc
        });

    let five_most_common = get_five_most_common(room);
    five_most_common.eq(checksum)
}

fn get_five_most_common(encr_name: HashMap<char, u8>) -> String {
    let mut room: HashMap<char, u8> = encr_name.clone();
    let mut most_common_letters: Vec<Vec<char>> = Vec::new();

    let mut max_val_old: u8 = 0;

    while !room.is_empty() {
        let max_val = room
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(_k, v)| *v)
            .unwrap();
        let mut max_val_letters: Vec<char> = Vec::new();
        let mut char_found: char = ' ';
        room.iter_mut().for_each(|(&key, value)| {
            if *value == max_val {
                char_found = key.clone();
                max_val_letters.push(key);
            }
        });
        room.remove(&char_found);

        if max_val_old != max_val {
            max_val_letters.sort();
            most_common_letters.push(max_val_letters);
        }

        max_val_old = max_val;
    }

    let sorted_letters: Vec<char> = most_common_letters.into_iter().flatten().collect();
    let five_most_common: &[char] = &sorted_letters[..NB_MAX];

    five_most_common.iter().collect()
}

fn compute_rot(encr_name: &str, sector_id: u32) {
    let decr_name: String = encr_name
        .chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                char::from_u32(((c as u32 + sector_id - 'a' as u32) % 26) + 'a' as u32).unwrap()
            }
        })
        .collect();

    if decr_name.contains("north") {
        println!(
            "the North Pole objects are stored in sector ID {:?}",
            sector_id
        );
    }
}
