use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

const FIRST_ROW: i32 = 2000000;
const SECOND_ROW: i32 = 4000000;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i32,
    max: i32,
}

fn main() {
    let mut file = File::open("inputs/input_2022_15.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mapped_data: HashMap<Coord, Coord> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split([' ', '=', ',', ':']).collect();
            let sx = split_line[3].parse::<i32>().unwrap();
            let sy = split_line[6].parse::<i32>().unwrap();
            let bx = split_line[13].parse::<i32>().unwrap();
            let by = split_line[16].parse::<i32>().unwrap();
            (Coord { x: sx, y: sy }, Coord { x: bx, y: by })
        })
        .collect();

    // first part
    let mut pos: HashSet<Coord> = HashSet::new();
    let mut b_found: HashSet<Coord> = HashSet::new();
    mapped_data.iter().for_each(|(&sensor, &beacon)| {
        if beacon.y == FIRST_ROW {
            b_found.insert(beacon);
        }
        let m_dist = get_manhattan_dist(sensor, beacon);
        compute_range(sensor, m_dist, &mut pos, FIRST_ROW);
    });

    println!(
        "in the row where y={FIRST_ROW}, {:?} positions cannot contain a beacon",
        pos.len() - b_found.len()
    );

    // second part
    find_distress_beacon(mapped_data);
}

fn get_manhattan_dist(start_coord: Coord, end_coord: Coord) -> i32 {
    (start_coord.x - end_coord.x).abs() + (start_coord.y - end_coord.y).abs()
}

fn compute_range(coord: Coord, m_dist: i32, pos: &mut HashSet<Coord>, row: i32) {
    (0..=m_dist).for_each(|y| {
        (coord.x - m_dist + y..=coord.x + m_dist - y).for_each(|current_x| {
            if coord.y + y == row || coord.y - y == row {
                pos.insert(Coord {
                    x: current_x,
                    y: FIRST_ROW,
                });
            }
        });
    });
}

fn compute_limited_range(coord: Coord, m_dist: i32, all_ranges: &mut Vec<Range>, row: i32) {
    all_ranges.push(Range {
        min: coord.x - m_dist + (coord.y - row).abs(),
        max: coord.x + m_dist - (coord.y - row).abs(),
    });
}

fn is_distress_beacon(all_data: Vec<Range>) -> (bool, i32) {
    let mut all_ranges: Vec<Range> = all_data.clone();
    let mut min: i32 = 0;
    let mut found: bool = false;

    all_ranges.sort_by(|a, b| a.min.cmp(&b.min));

    all_ranges.iter().for_each(|new_range| {
        if new_range.min <= min {
            if new_range.max >= min {
                min = new_range.max + 1;
            }
        }
    });

    if min < SECOND_ROW {
        found = true;
    }

    (found, min)
}

fn find_distress_beacon(data: HashMap<Coord, Coord>) {
    (0..=SECOND_ROW).for_each(|y| {
        let mut all_ranges: Vec<Range> = Vec::new();
        data.iter().for_each(|(&sensor, &beacon)| {
            let m_dist = get_manhattan_dist(sensor, beacon);
            compute_limited_range(sensor, m_dist, &mut all_ranges, y);
        });

        let (found, x) = is_distress_beacon(all_ranges);
        if found {
            let ans: u64 = x as u64 * 4_000_000 + y as u64;
            println!("its tuning frequency is: {ans}");
        }
    });
}
