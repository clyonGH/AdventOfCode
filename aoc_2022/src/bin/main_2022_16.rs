use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: u16,
    neighbours: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tunnel {
    src: Valve,
    dest: Valve,
    dist: u16,
}
fn main() {
    let mut file = File::open("inputs/input_2022_16.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_valves: HashSet<Valve> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split([' ', ',', '=', ';']).collect();
            let name: String = split_line[1].to_string();
            let flow_rate: u16 = split_line[5].parse::<u16>().unwrap();
            let neighbours: Vec<String> = (11..split_line.len())
                .step_by(2)
                .map(|i| split_line[i].to_string())
                .collect();

            Valve {
                name,
                flow_rate,
                neighbours,
            }
        })
        .collect();

    // get all valves with a flow rate > 0
    let main_valves = get_main_valves(&all_valves);

    // get all tunnels
    let all_tunnels: HashMap<(String, String), Tunnel> = get_all_tunnels(&all_valves);

    // get the names of the neighbours of your starting point "AA"
    let main_valves_str: HashSet<String> = main_valves
        .iter()
        .filter(|&valve| valve.name != "AA")
        .map(|valve| valve.name.clone())
        .collect();
    let new_main_valves_str = main_valves_str.clone();

    // first part
    let current_valve: Valve = find_valve("AA", &all_valves);
    let mut pressure_released: u16 = 0;
    let mut time_remaining: u16 = 30;
    let ans: Vec<u16> = get_all_pressure_releases(
        &current_valve.name,
        time_remaining,
        pressure_released,
        &all_tunnels,
        main_valves_str,
    );

    println!(
        "{:?} is the most pressure you can release",
        ans.iter().max().unwrap()
    );

    // second part
    pressure_released = 0;
    time_remaining = 26;
    let ans2: Vec<(u16, HashSet<String>)> = get_new_all_pressure_releases(
        &current_valve.name,
        time_remaining,
        pressure_released,
        &all_tunnels,
        new_main_valves_str,
    );
    let ans2_val: Vec<u16> = ans2.iter().map(|tuple| tuple.0).collect();
    let i_max: usize = ans2_val
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
    let ans22: Vec<u16> = get_all_pressure_releases(
        &current_valve.name,
        time_remaining,
        pressure_released,
        &all_tunnels,
        ans2[i_max].1.clone(),
    );

    println!(
        "with you and an elephant working together for 26 minutes {:?} is the most pressure you can release",
        ans22.iter().max().unwrap() + ans2_val[i_max]
    );
}

fn find_valve(valve_str: &str, all_valves: &HashSet<Valve>) -> Valve {
    let found_valve = all_valves
        .iter()
        .find(|&valve| valve.name == valve_str)
        .unwrap();

    found_valve.clone()
}

fn breadth_first_search(src: &str, dest: &str, all_valves: HashSet<Valve>) -> u16 {
    let mut dist: u16 = 0;
    let mut already_visited: HashSet<String> = HashSet::new();
    let first_valve: Valve = find_valve(src, &all_valves);
    let mut current_valves: Vec<Valve> = vec![first_valve.clone()];
    already_visited.insert(first_valve.name);

    loop {
        current_valves = current_valves
            .iter()
            .flat_map(|valve| {
                valve
                    .neighbours
                    .iter()
                    .map(|neighbour| find_valve(neighbour, &all_valves))
                    .filter(|valve| already_visited.insert(valve.name.clone()))
                    .collect::<Vec<Valve>>()
            })
            .collect();

        dist += 1;

        if current_valves
            .iter()
            .find(|&valve| valve.name == dest)
            .is_some()
        {
            break;
        }
    }

    dist
}

fn get_main_valves(all_valves: &HashSet<Valve>) -> Vec<Valve> {
    all_valves
        .into_iter()
        .filter(|&valve| valve.flow_rate > 0 || valve.name == "AA")
        .cloned()
        .collect()
}

fn get_all_tunnels(all_valves: &HashSet<Valve>) -> HashMap<(String, String), Tunnel> {
    // get all valves with a flow rate > 0
    let main_valves: Vec<Valve> = get_main_valves(all_valves);

    // get all pairs from the main valves
    let all_pairs: Vec<Vec<&Valve>> = main_valves.iter().combinations(2).collect();

    // get all the tunnels: src, dest, dist
    let mut all_tunnels: HashMap<(String, String), Tunnel> = HashMap::new();
    (0..all_pairs.len()).for_each(|i| {
        let pair = all_pairs[i].clone();
        let pair_src = pair[0].clone();
        let pair_dest = pair[1].clone();
        let pair_src_name = pair_src.name.clone();
        let pair_dest_name = pair_dest.name.clone();
        let dist = breadth_first_search(&pair_src_name, &pair_dest_name, all_valves.clone());
        all_tunnels.insert(
            (pair_src_name.clone(), pair_dest_name.clone()),
            Tunnel {
                src: pair_src.clone(),
                dest: pair_dest.clone(),
                dist,
            },
        );

        all_tunnels.insert(
            (pair_dest_name, pair_src_name),
            Tunnel {
                src: pair_dest,
                dest: pair_src,
                dist,
            },
        );
    });

    all_tunnels
}

fn get_all_pressure_releases(
    src: &str,
    time_remaining: u16,
    pressure_release: u16,
    all_tunnels: &HashMap<(String, String), Tunnel>,
    to_visit: HashSet<String>,
) -> Vec<u16> {
    if time_remaining == 0 || to_visit.is_empty() {
        vec![pressure_release]
    } else {
        to_visit
            .iter()
            .flat_map(|valve| {
                let tunnel = all_tunnels
                    .get(&(src.to_string(), valve.to_string()))
                    .unwrap();
                let dist = tunnel.dist;
                let flow_rate = tunnel.dest.flow_rate;

                if time_remaining >= dist + 1 {
                    let new_pressure_release =
                        pressure_release + (time_remaining - dist - 1) * flow_rate;
                    let new_time_remaining = time_remaining - dist - 1;
                    let mut new_to_visit = to_visit.clone();
                    new_to_visit.remove(valve);

                    get_all_pressure_releases(
                        valve,
                        new_time_remaining,
                        new_pressure_release,
                        all_tunnels,
                        new_to_visit,
                    )
                } else {
                    vec![pressure_release]
                }
            })
            .collect()
    }
}

fn get_new_all_pressure_releases(
    src: &str,
    time_remaining: u16,
    pressure_release: u16,
    all_tunnels: &HashMap<(String, String), Tunnel>,
    to_visit: HashSet<String>,
) -> Vec<(u16, HashSet<String>)> {
    if time_remaining == 0 || to_visit.is_empty() {
        vec![(pressure_release, to_visit)]
    } else {
        to_visit
            .iter()
            .flat_map(|valve| {
                let tunnel = all_tunnels
                    .get(&(src.to_string(), valve.to_string()))
                    .unwrap();
                let dist = tunnel.dist;
                let flow_rate = tunnel.dest.flow_rate;

                if time_remaining >= dist + 1 {
                    let new_pressure_release =
                        pressure_release + (time_remaining - dist - 1) * flow_rate;
                    let new_time_remaining = time_remaining - dist - 1;
                    let mut new_to_visit = to_visit.clone();
                    new_to_visit.remove(valve);

                    get_new_all_pressure_releases(
                        valve,
                        new_time_remaining,
                        new_pressure_release,
                        all_tunnels,
                        new_to_visit,
                    )
                } else {
                    vec![(pressure_release, to_visit.clone())]
                }
            })
            .collect()
    }
}
