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

        // first part
        if compound_1_nb == *info_mfcsam.get(&compound_1).unwrap()
            && compound_2_nb == *info_mfcsam.get(&compound_2).unwrap()
            && compound_3_nb == *info_mfcsam.get(&compound_3).unwrap()
        {
            println!("Sue {sue_nb} got me the gift");
        }

        // second part
        let mut true_sue = false;
        if ((compound_1.eq("cats") || compound_1.eq("trees"))
            && compound_1_nb > *info_mfcsam.get(&compound_1).unwrap())
            || ((compound_1.eq("pomeranians") || compound_1.eq("goldfish"))
                && compound_1_nb < *info_mfcsam.get(&compound_1).unwrap())
            || (compound_1_nb == *info_mfcsam.get(&compound_1).unwrap())
        {
            true_sue = true;
        } else {
            true_sue = false;
        }

        if true_sue {
            if ((compound_2.eq("cats") || compound_2.eq("trees"))
                && compound_2_nb > *info_mfcsam.get(&compound_2).unwrap())
                || ((compound_2.eq("pomeranians") || compound_2.eq("goldfish"))
                    && compound_2_nb < *info_mfcsam.get(&compound_2).unwrap())
                || (compound_2_nb == *info_mfcsam.get(&compound_2).unwrap())
            {
                true_sue = true;
            } else {
                true_sue = false;
            }

            if true_sue {
                if ((compound_3.eq("cats") || compound_3.eq("trees"))
                    && compound_3_nb > *info_mfcsam.get(&compound_3).unwrap())
                    || ((compound_3.eq("pomeranians") || compound_3.eq("goldfish"))
                        && compound_3_nb < *info_mfcsam.get(&compound_3).unwrap())
                    || (compound_3_nb == *info_mfcsam.get(&compound_3).unwrap())
                {
                    true_sue = true;
                } else {
                    true_sue = false;
                }
            }
        }

        if compound_1_nb == *info_mfcsam.get(&compound_1).unwrap()
            && compound_2_nb == *info_mfcsam.get(&compound_2).unwrap()
            && compound_3_nb == *info_mfcsam.get(&compound_3).unwrap()
        {
            true_sue = false;
        }

        if true_sue {
            println!("In the end, it was Sue {sue_nb} who got me the gift");
        }
    }
}
