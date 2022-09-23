use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_03.txt").expect("Error: File not found");

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut coord_reached: HashSet<(i32, i32)> = HashSet::new();
    coord_reached.insert((0, 0));

    // Santa's coordinates
    let mut s_coord_current: (i32, i32) = (0, 0);

    // first part
    for instruction in data.chars() {
        match instruction {
            '^' => s_coord_current = (s_coord_current.0, s_coord_current.1 + 1),
            'v' => s_coord_current = (s_coord_current.0, s_coord_current.1 - 1),
            '<' => s_coord_current = (s_coord_current.0 - 1, s_coord_current.1),
            '>' => s_coord_current = (s_coord_current.0 + 1, s_coord_current.1),
            _ => (),
        }

        coord_reached.insert(s_coord_current);
    }

    println!(
        "number of houses reached by Santa alone: {:?}",
        coord_reached.len()
    );

    // second part
    coord_reached.clear();
    s_coord_current = (0, 0);

    // Robo-Santa's coordinates
    let mut rs_coord_current: (i32, i32) = (0, 0);

    for instruction in data.chars().enumerate() {
        if instruction.0 % 2 == 0 {
            match instruction.1 {
                '^' => rs_coord_current = (rs_coord_current.0, rs_coord_current.1 + 1),
                'v' => rs_coord_current = (rs_coord_current.0, rs_coord_current.1 - 1),
                '<' => rs_coord_current = (rs_coord_current.0 - 1, rs_coord_current.1),
                '>' => rs_coord_current = (rs_coord_current.0 + 1, rs_coord_current.1),
                _ => (),
            }
            coord_reached.insert(rs_coord_current);
        } else {
            match instruction.1 {
                '^' => s_coord_current = (s_coord_current.0, s_coord_current.1 + 1),
                'v' => s_coord_current = (s_coord_current.0, s_coord_current.1 - 1),
                '<' => s_coord_current = (s_coord_current.0 - 1, s_coord_current.1),
                '>' => s_coord_current = (s_coord_current.0 + 1, s_coord_current.1),
                _ => (),
            }
            coord_reached.insert(s_coord_current);
        }
    }

    println!(
        "number of houses reached by Santa and Robo-Santa: {:?}",
        coord_reached.len()
    );
}
