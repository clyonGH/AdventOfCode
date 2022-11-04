use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_24.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let sorted_packages: Vec<u64> = data
        .lines()
        .map(|package| package.parse::<u64>().unwrap())
        .collect();

    // first part: the packages are divided in 3
    // second part: the packages are divided in 4
    let nb_groups: u64 = 3;
    let max_weight: u64 = sorted_packages.iter().sum::<u64>() / nb_groups;

    let all_groups = fill_packages(max_weight, sorted_packages, Vec::new());
    let smallest_group = all_groups.iter().map(|group| group.len()).min().unwrap();
    let small_groups: Vec<u64> = all_groups
        .iter()
        .filter_map(|group| {
            if group.len() > smallest_group {
                None
            } else {
                Some(group.iter().product())
            }
        })
        .collect();

    println!(
        "{:?} is the quantum entanglement of the first group of packages",
        small_groups.iter().min()
    );
}

fn fill_packages(weight: u64, mut packages: Vec<u64>, found_packages: Vec<u64>) -> Vec<Vec<u64>> {
    if weight == 0 {
        return vec![found_packages];
    }

    if packages.is_empty() {
        return Vec::new();
    }

    let first_package = packages.pop().unwrap();
    if weight < first_package {
        fill_packages(weight, packages, found_packages)
    } else {
        let mut new_found_packages = found_packages.clone();
        new_found_packages.push(first_package);
        let mut first_found = fill_packages(weight, packages.clone(), found_packages);
        first_found.extend(fill_packages(
            weight - first_package,
            packages,
            new_found_packages,
        ));
        first_found
    }
}
