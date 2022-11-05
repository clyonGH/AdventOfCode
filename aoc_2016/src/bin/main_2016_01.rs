use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy)]
enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
struct Coordinates {
    orientation: Orientation,
    x: i32,
    y: i32,
}

impl Coordinates {
    fn rotate(&mut self, rotation: char) {
        match (self.orientation, rotation) {
            (Orientation::North, 'R') => self.orientation = Orientation::East,
            (Orientation::North, 'L') => self.orientation = Orientation::West,
            (Orientation::South, 'R') => self.orientation = Orientation::West,
            (Orientation::South, 'L') => self.orientation = Orientation::East,
            (Orientation::East, 'R') => self.orientation = Orientation::South,
            (Orientation::East, 'L') => self.orientation = Orientation::North,
            (Orientation::West, 'R') => self.orientation = Orientation::North,
            (Orientation::West, 'L') => self.orientation = Orientation::South,
            _ => (),
        }
    }

    fn move_forward(&mut self) {
        match self.orientation {
            Orientation::North => self.y += 1,
            Orientation::South => self.y -= 1,
            Orientation::East => self.x += 1,
            Orientation::West => self.x -= 1,
        }
    }
}

fn main() {
    let mut file = File::open("inputs/input_2016_01.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let instructions: Vec<&str> = data.split(", ").collect();

    let mut santa_coord: Coordinates = Coordinates {
        orientation: Orientation::North,
        x: 0,
        y: 0,
    };

    let mut all_coord: HashSet<(i32, i32)> = HashSet::new();
    let mut bunny_found: bool = false;

    instructions.iter().for_each(|&instruction| {
        let (turn, nb_blocks) = split_first_char(&instruction).unwrap();

        santa_coord.rotate(turn);
        (0..nb_blocks.parse::<i32>().unwrap()).for_each(|_| {
            santa_coord.move_forward();

            // second part
            if !all_coord.insert((santa_coord.x, santa_coord.y)) && !bunny_found {
                bunny_found = true;
                println!(
                    "the first location you visit twice is {:?} blocks away",
                    santa_coord.x.abs() + santa_coord.y.abs()
                );
            }
        });
    });

    // first part
    println!(
        "Easter Bunny HQ is {:?} blocks away",
        santa_coord.x.abs() + santa_coord.y.abs()
    );
}

fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => Some((c, chars.as_str())),
        None => None,
    }
}
