use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_16.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let info_mfcsam = HashMap::from([
        ("children".to_owned(), 3),
        ("cats".to_owned(), 7),
        ("samoyeds".to_owned(), 2),
        ("pomeranians".to_owned(), 3),
        ("akitas".to_owned(), 0),
        ("vizslas".to_owned(), 0),
        ("goldfish".to_owned(), 5),
        ("trees".to_owned(), 3),
        ("cars".to_owned(), 2),
        ("perfumes".to_owned(), 1),
    ]);

    for line in data.lines() {
        let re =
            Regex::new(r"Sue ([0-9]*): ([a-z]*): ([0-9]*), ([a-z]*): ([0-9]*), ([a-z]*): ([0-9]*)")
                .unwrap();
        let caps = re.captures(line).unwrap();
        let sue_nb = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
        let compound_1 = caps.get(2).unwrap().as_str().to_string();
        let compound_1_nb = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
        let compound_2 = caps.get(4).unwrap().as_str().to_string();
        let compound_2_nb = caps.get(5).unwrap().as_str().parse::<u8>().unwrap();
        let compound_3 = caps.get(6).unwrap().as_str().to_string();
        let compound_3_nb = caps.get(7).unwrap().as_str().parse::<u8>().unwrap();

        let new_sue = HashMap::from([
            (compound_1, compound_1_nb),
            (compound_2, compound_2_nb),
            (compound_3, compound_3_nb),
        ]);

        // first part
        if new_sue
            .iter()
            .all(|(compound, &value)| value == info_mfcsam.get(compound).cloned().unwrap())
        {
            println!("Sue {sue_nb} got me the gift");
        }

        // second part
        if comp_sues(&info_mfcsam, new_sue) {
            println!("In the very end, it was Sue {sue_nb} who got me the gift");
        }
    }
}

fn comp_sues(info_mfcsam: &HashMap<String, u8>, new_sue: HashMap<String, u8>) -> bool {
    new_sue
        .into_iter()
        .all(|(compound, value)| match compound.as_str() {
            "cats" | "trees" => value > info_mfcsam.get(&compound).cloned().unwrap(),
            "pomeranians" | "goldfish" => value < info_mfcsam.get(&compound).cloned().unwrap(),
            _ => value == info_mfcsam.get(&compound).cloned().unwrap(),
        })
}
