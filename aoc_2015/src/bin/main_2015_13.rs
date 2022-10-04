use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_13.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_data: HashMap<(&str, &str), i16> = HashMap::new(); // (first name, second name, happiness units)
    let mut all_guests: HashSet<&str> = HashSet::new();

    for line in data.lines() {
        let re = Regex::new(
            r"([a-zA-Z]*) would ([a-z]*) ([0-9]*) happiness units by sitting next to ([a-zA-Z]*).",
        )
        .unwrap();
        let caps = re.captures(line).unwrap();
        let happy_units = caps.get(3).unwrap().as_str().parse::<i16>().unwrap();
        let name_1;
        let name_2;

        if caps
            .get(1)
            .unwrap()
            .as_str()
            .cmp(caps.get(4).unwrap().as_str())
            .is_lt()
        {
            name_1 = caps.get(1).unwrap().as_str();
            name_2 = caps.get(4).unwrap().as_str();
        } else {
            name_2 = caps.get(1).unwrap().as_str();
            name_1 = caps.get(4).unwrap().as_str();
        }

        all_guests.insert(name_1);
        all_guests.insert(name_2);

        match caps.get(2) {
            Some(gain_lose) if gain_lose.as_str().eq("gain") => {
                all_data.insert(
                    (name_1, name_2),
                    all_data.get(&(name_1, name_2)).unwrap_or_else(|| &0) + happy_units,
                );
            }
            Some(gain_lose) if gain_lose.as_str().eq("lose") => {
                all_data.insert(
                    (name_1, name_2),
                    all_data.get(&(name_1, name_2)).unwrap_or_else(|| &0) - happy_units,
                );
            }
            _ => (),
        };
    }

    // same code as day 9, except for the first_last_sum addition
    let all_permute = permute_guests(all_guests.into_iter().collect());
    let max_happy_units = all_permute
        .iter()
        .map(|one_perm| {
            let first_last_sum = all_data
                .get(&(one_perm[0], one_perm[one_perm.len() - 1]))
                .unwrap_or_else(|| {
                    all_data
                        .get(&(one_perm[one_perm.len() - 1], one_perm[0]))
                        .unwrap()
                });
            one_perm
                .windows(2)
                .map(|window| {
                    all_data
                        .get(&(window[0], window[1]))
                        .unwrap_or_else(|| all_data.get(&(window[1], window[0])).unwrap())
                })
                .sum::<i16>()
                + first_last_sum
        })
        .max();

    println!("max happy units: {:?}", max_happy_units);

    // modified input for the second part adding Me...
}

// same code as day 9
fn permute_guests(all_guests: Vec<&str>) -> Vec<Vec<&str>> {
    if all_guests.len() == 1 {
        let mut lonely_guest = Vec::new();
        lonely_guest.push(all_guests);
        lonely_guest
    } else {
        all_guests
            .iter()
            .enumerate()
            .flat_map(|(index, guest)| {
                let mut all_guests_rm = all_guests.clone();
                all_guests_rm.remove(index);
                permute_guests(all_guests_rm)
                    .into_iter()
                    .map(|mut perm_guests| {
                        let new_guest = guest.clone();
                        perm_guests.push(new_guest);
                        perm_guests
                    })
                    .collect::<Vec<Vec<&str>>>()
            })
            .collect()
    }
}
