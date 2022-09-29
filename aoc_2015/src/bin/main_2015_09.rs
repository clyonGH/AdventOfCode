use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_09.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut od_dist: HashMap<(String, String), u32> = HashMap::new();
    let mut all_cities: HashSet<String> = HashSet::new();
    data.lines().for_each(|line| {
        let vec_line = line.split(' ').collect::<Vec<&str>>();
        od_dist.insert(
            (vec_line[0].to_string(), vec_line[2].to_string()),
            vec_line[4].parse::<u32>().unwrap(),
        );
        all_cities.insert(vec_line[0].to_string());
        all_cities.insert(vec_line[2].to_string());
    });

    let all_permute = permute_cities(all_cities.into_iter().collect());

    // first part
    let lowest_dist = all_permute
        .iter()
        .map(|one_perm| {
            one_perm
                .windows(2)
                .map(|window| {
                    od_dist
                        .get(&(window[0].to_owned(), window[1].to_owned()))
                        .unwrap_or_else(|| {
                            od_dist
                                .get(&(window[1].to_owned(), window[0].to_owned()))
                                .unwrap()
                        })
                })
                .sum::<u32>()
        })
        .min();

    println!("lowest distance: {:?}", lowest_dist);

    // second part: change the min() distance calculated to the max() distance instead
}

fn permute_cities(all_cities: Vec<String>) -> Vec<Vec<String>> {
    if all_cities.len() == 1 {
        let mut lonely_city = Vec::new();
        lonely_city.push(all_cities);
        lonely_city
    } else {
        all_cities
            .iter()
            .enumerate()
            .flat_map(|(index, city)| {
                let mut all_cities_rm = all_cities.clone();
                all_cities_rm.remove(index);
                permute_cities(all_cities_rm)
                    .into_iter()
                    .map(|mut perm_cities| {
                        let new_city = city.clone();
                        perm_cities.push(new_city);
                        perm_cities
                    })
                    .collect::<Vec<Vec<String>>>()
            })
            .collect()
    }
}
