use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_06.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut light_grid: Vec<Vec<u64>> = vec![vec![0; 1000]; 1000];
    let mut brightness_grid: Vec<Vec<u64>> = vec![vec![0; 1000]; 1000];

    for line in data.lines() {
        let (instruction, start_x_coord, start_y_coord, end_x_coord, end_y_coord) =
            get_coord(line).unwrap();

        for x in start_x_coord..=end_x_coord {
            for y in start_y_coord..=end_y_coord {
                change_light(instruction, &mut light_grid, x, y);
                change_brightness(instruction, &mut brightness_grid, x, y);
            }
        }
    }

    // first part
    let light_result: u64 = light_grid
        .iter()
        .map(|column| column.iter().sum::<u64>())
        .sum::<u64>();

    println!("number of lit lights: {light_result}");

    // second part
    let brightness_result: u64 = brightness_grid
        .iter()
        .map(|column| column.iter().sum::<u64>())
        .sum::<u64>();
    println!("total brightness: {brightness_result}");
}

fn change_light(instr: &str, light_grid: &mut Vec<Vec<u64>>, x_coord: usize, y_coord: usize) {
    match instr {
        "turn on" => light_grid[x_coord][y_coord] = 1,
        "turn off" => light_grid[x_coord][y_coord] = 0,
        "toggle" => light_grid[x_coord][y_coord] ^= 1,
        _ => (),
    }
}

fn change_brightness(instr: &str, light_grid: &mut Vec<Vec<u64>>, x_coord: usize, y_coord: usize) {
    match instr {
        "turn on" => light_grid[x_coord][y_coord] += 1,
        "turn off" if light_grid[x_coord][y_coord] != 0 => light_grid[x_coord][y_coord] -= 1,
        "toggle" => light_grid[x_coord][y_coord] += 2,
        _ => (),
    }
}

fn get_coord(line: &str) -> Option<(&str, usize, usize, usize, usize)> {
    let re = Regex::new(r"([a-z ]*) (\d*),(\d*) through (\d*),(\d*)").unwrap();
    let caps = re.captures(line).unwrap();
    Some((
        caps.get(1)?.as_str(),
        caps.get(2)?.as_str().parse::<usize>().unwrap(),
        caps.get(3)?.as_str().parse::<usize>().unwrap(),
        caps.get(4)?.as_str().parse::<usize>().unwrap(),
        caps.get(5)?.as_str().parse::<usize>().unwrap(),
    ))
}
