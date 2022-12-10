use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_09.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let (mut hx, mut hy): (i64, i64) = (0, 0);
    let mut tail: (i64, i64) = (0, 0);
    let mut visited_pos: HashSet<(i64, i64)> = HashSet::new();
    visited_pos.insert(tail);

    // first part
    data.lines().for_each(|line| {
        let split_line: Vec<&str> = line.split(' ').collect();
        let dir: &str = split_line[0];
        let val: u8 = split_line[1].parse::<u8>().unwrap();
        (0..val).for_each(|_| {
            match dir {
                "U" => hy += 1,
                "D" => hy -= 1,
                "L" => hx -= 1,
                "R" => hx += 1,
                _ => (),
            }

            move_tail(hx, hy, &mut tail);
            visited_pos.insert(tail);
        });
    });

    println!(
        "the tail of the rope visits {:?} positions at least once",
        visited_pos.len()
    );

    // second part
    (hx, hy) = (0, 0);
    let mut all_knots: Vec<(i64, i64)> = vec![(0, 0); 10];
    visited_pos.clear();
    data.lines().for_each(|line| {
        let split_line: Vec<&str> = line.split(' ').collect();
        let dir: &str = split_line[0];
        let val: u8 = split_line[1].parse::<u8>().unwrap();
        (0..val).for_each(|_| {
            match dir {
                "U" => hy += 1,
                "D" => hy -= 1,
                "L" => hx -= 1,
                "R" => hx += 1,
                _ => (),
            }

            all_knots[0] = (hx, hy);
            (1..10).for_each(|i| {
                move_tail(all_knots[i - 1].0, all_knots[i - 1].1, &mut all_knots[i]);
            });
            visited_pos.insert(all_knots[9]);
        });
    });

    println!(
        "the longer tail of the rope visits {:?} positions at least once",
        visited_pos.len()
    );
}

fn move_tail(hx: i64, hy: i64, tail: &mut (i64, i64)) {
    let (ref mut tx, ref mut ty) = *tail;
    let diff_x: i64 = hx - *tx;
    let diff_y: i64 = hy - *ty;

    match (diff_x, diff_y) {
        (2, 2) => {
            *tx += 1;
            *ty += 1;
        }
        (-2, -2) => {
            *tx -= 1;
            *ty -= 1;
        }
        (-2, 2) => {
            *ty += 1;
            *tx -= 1;
        }
        (2, -2) => {
            *ty -= 1;
            *tx += 1;
        }
        (2, _) => {
            *tx += 1;
            *ty = hy;
        }
        (-2, _) => {
            *tx -= 1;
            *ty = hy;
        }
        (_, 2) => {
            *ty += 1;
            *tx = hx;
        }
        (_, -2) => {
            *ty -= 1;
            *tx = hx;
        }
        _ => (),
    }
}
