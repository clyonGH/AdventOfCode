use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_19.txt").expect("Error: File not found");
    let mut data: String = String::new();

    let input_mol = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let replacements: HashSet<(String, String)> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(' ').collect();
            (split_line[0].to_string(), split_line[2].to_string())
        })
        .collect::<HashSet<(String, String)>>();

    // first part
    println!(
        "{:?} distinct molecules can be created",
        get_distinct_mol(replacements.to_owned(), input_mol.to_string()).len()
    );

    // second part
    let mut input_mol_hs: HashSet<String> = HashSet::new();
    input_mol_hs.insert(input_mol.to_string());

    // separating all molecules to count them
    let all_mol: Vec<String> = get_all_mol(input_mol);

    // excluding e, all patterns look like this xRnxAr, xRnxYxAr and xRnxYxYxAr
    // for each x, it requires nb of molecules in x -1 to reduce it
    // but for each Rn or Ar and it takes +1 to reduce it, which means that for xRn or xAr it requires the nb of molecules in x to reduce it
    // for xY, as it is included inside the global pattern, it won't require an additional reduction at the end, so it means nb of molecules -1
    // so to simplify it, each molecule count as 1, we don't count Rn or Ar and Y count as -1
    let steps: i32 = all_mol
        .iter()
        .map(|mol| match mol.as_str() {
            "Rn" | "Ar" => 0,
            "Y" => -1,
            _ => 1,
        })
        .sum();

    // steps - 1 because the last reducing step is not 2 steps, but just 1 contrary to all xRnxAr that require 2 reducing steps
    println!(
        "{:?} is the fewest number of steps to go from e to the medicine molecule",
        steps - 1
    );

    // too long to compute :(
    // let all_distances = get_distance(replacements, input_mol_hs, 0);
    // println!("{:?}", all_distances);
}

fn get_distinct_mol(
    replacements: HashSet<(String, String)>,
    current_mol: String,
) -> HashSet<String> {
    let mut all_mol: HashSet<String> = HashSet::new();
    replacements.iter().for_each(|(old_pattern, new_pattern)| {
        let all_indexes: Vec<_> = current_mol.match_indices(old_pattern).collect();
        all_indexes.iter().for_each(|&(index, _pattern)| {
            let mut input_mol_clone = current_mol.clone().to_string();
            let end_range = index + old_pattern.len();
            input_mol_clone.replace_range(index..end_range, new_pattern);
            all_mol.insert(input_mol_clone.to_owned());
        });
    });

    all_mol
}

fn get_all_mol(current_mol: &str) -> Vec<String> {
    current_mol
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter_map(|window| match (window[0], window[1]) {
            ('A'..='Z', 'a'..='z') => Some(format!("{}{}", window[0], window[1])),
            ('A'..='Z', 'A'..='Z') => Some(format!("{}", window[0])),
            _ => None,
        })
        .collect()
}

// too long to compute :(
// fn get_distance(
//     replacements: HashSet<(String, String)>,
//     all_mol: HashSet<String>,
//     current_distance: u32,
// ) -> (HashSet<String>, u32) {
//     if all_mol.contains("e") {
//         (all_mol, current_distance)
//     } else {
//         let mut new_all_mol: HashSet<String> = HashSet::new();
//         all_mol.iter().for_each(|current_mol| {
//             replacements.iter().for_each(|(old_pattern, new_pattern)| {
//                 match (old_pattern.as_str(), current_mol.as_str()) {
//                     ("e", "HF") | ("e", "NAl") | ("e", "OMg") => {
//                         new_all_mol.insert("e".to_string());
//                     }
//                     ("e", _) => (),
//                     _ => {
//                         let all_indexes: Vec<_> = current_mol.match_indices(new_pattern).collect();
//                         all_indexes.iter().for_each(|&(index, _pattern)| {
//                             let mut input_mol_clone = current_mol.clone().to_string();
//                             let end_range = index + new_pattern.len();
//                             input_mol_clone.replace_range(index..end_range, old_pattern);
//                             new_all_mol.insert(input_mol_clone.to_owned());
//                         });
//                     }
//                 }
//             })
//         });
//         get_distance(replacements, new_all_mol, current_distance + 1)
//     }
// }
