use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

const MAX_WIDTH: usize = 500;
const MAX_HEIGHT: usize = 170;
const X_OFFSET: usize = 250;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() {
    let mut file = File::open("inputs/input_2022_14.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut rock_struct: [[char; MAX_WIDTH]; MAX_HEIGHT] = [['.'; MAX_WIDTH]; MAX_HEIGHT];
    let mapped_data: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .collect();

    let mut highest_y: usize = 0;
    mapped_data.iter().for_each(|path| {
        path.windows(2).for_each(|window| {
            let coord1: Vec<&str> = window[0].split(',').collect();
            let coord2: Vec<&str> = window[1].split(',').collect();
            let (x1, y1): (usize, usize) = (coord1[0].parse().unwrap(), coord1[1].parse().unwrap());
            let (x2, y2): (usize, usize) = (coord2[0].parse().unwrap(), coord2[1].parse().unwrap());
            let (xmin, xmax) = get_min_max(x1, x2);
            let (ymin, ymax) = get_min_max(y1, y2);

            if ymax > highest_y {
                highest_y = ymax;
            }

            (ymin..=ymax).for_each(|y| {
                (xmin - X_OFFSET as usize..=xmax - X_OFFSET as usize).for_each(|x| {
                    rock_struct[y][x] = '#';
                });
            });
        });
    });

    let mut sand_coord: Coord = Coord {
        x: 500 - X_OFFSET,
        y: 0,
    };
    rock_struct[sand_coord.y][sand_coord.x] = '+';
    let mut sand_sum: u32 = 0;
    let mut first_part: bool = false;

    // second part
    (0..MAX_WIDTH).for_each(|x| {
        rock_struct[highest_y + 2][x] = '#';
    });
    while rock_struct[sand_coord.y][sand_coord.x] != 'o' {
        move_sand(&mut rock_struct, &mut sand_coord, &mut sand_sum);

        // first part
        if sand_coord.y > 160 && !first_part {
            first_part = true;
            println!(
                "{:?} units of sand come to rest before sand starts flowing into the abyss below",
                sand_sum
            );
        }
    }

    println!("{:?} units of sand come to rest", sand_sum);
}

fn move_sand(
    rock_struct: &mut [[char; MAX_WIDTH]; MAX_HEIGHT],
    sand_coord: &mut Coord,
    sand_sum: &mut u32,
) -> bool {
    let mut moved: bool = true;

    if rock_struct[sand_coord.y + 1][sand_coord.x] == '.' {
        sand_coord.y += 1;
    } else if rock_struct[sand_coord.y + 1][sand_coord.x - 1] == '.' {
        sand_coord.y += 1;
        sand_coord.x -= 1;
    } else if rock_struct[sand_coord.y + 1][sand_coord.x + 1] == '.' {
        sand_coord.y += 1;
        sand_coord.x += 1;
    } else {
        rock_struct[sand_coord.y][sand_coord.x] = 'o';
        sand_coord.y = 0;
        sand_coord.x = 500 - X_OFFSET;
        *sand_sum += 1;
        moved = false;
    }

    moved
}

fn get_min_max(coord1: usize, coord2: usize) -> (usize, usize) {
    let (mut xmin, mut xmax): (usize, usize) = (0, 0);

    match coord1.cmp(&coord2) {
        Ordering::Equal => {
            xmin += coord1;
            xmax += coord2;
        }
        Ordering::Less => {
            xmin += coord1;
            xmax += coord2;
        }
        Ordering::Greater => {
            xmin += coord2;
            xmax += coord1;
        }
    }

    (xmin, xmax)
}
