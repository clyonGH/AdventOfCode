use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_18.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut grid_1: [[bool; 100]; 100] = data
        .lines()
        .map(|line| {
            let grid_line: [bool; 100] = line
                .chars()
                .map(|c| match c {
                    '#' => true,
                    _ => false,
                })
                .collect::<Vec<bool>>()
                .try_into()
                .unwrap();
            grid_line
        })
        .collect::<Vec<[bool; 100]>>()
        .try_into()
        .unwrap();

    (1..=100).into_iter().for_each(|_| {
        grid_1 = change_grid(grid_1);
    });

    let lights_on: usize = grid_1
        .iter()
        .map(|line| line.iter().filter(|&&light| light).count())
        .sum();

    println!("after 100 steps {:?} lights are on", lights_on);
}

fn get_neighbours(grid: &[[bool; 100]; 100], x: usize, y: usize) -> Vec<bool> {
    let mut neighbours: Vec<bool> = Vec::new();
    match (x, y) {
        (0, 0) => {
            neighbours.push(grid[0][1]);
            neighbours.push(grid[1][1]);
            neighbours.push(grid[1][0]);
        }
        (0, 99) => {
            neighbours.push(grid[0][98]);
            neighbours.push(grid[1][98]);
            neighbours.push(grid[1][99]);
        }
        (99, 0) => {
            neighbours.push(grid[98][0]);
            neighbours.push(grid[98][1]);
            neighbours.push(grid[99][1]);
        }
        (99, 99) => {
            neighbours.push(grid[98][98]);
            neighbours.push(grid[98][99]);
            neighbours.push(grid[99][98]);
        }
        (0, _) => {
            neighbours.push(grid[x + 1][y]);
            neighbours.push(grid[x][y - 1]);
            neighbours.push(grid[x + 1][y - 1]);
            neighbours.push(grid[x][y + 1]);
            neighbours.push(grid[x + 1][y + 1]);
        }
        (99, _) => {
            neighbours.push(grid[x - 1][y]);
            neighbours.push(grid[x - 1][y - 1]);
            neighbours.push(grid[x][y - 1]);
            neighbours.push(grid[x - 1][y + 1]);
            neighbours.push(grid[x][y + 1]);
        }
        (_, 0) => {
            neighbours.push(grid[x - 1][y]);
            neighbours.push(grid[x + 1][y]);
            neighbours.push(grid[x - 1][y + 1]);
            neighbours.push(grid[x][y + 1]);
            neighbours.push(grid[x + 1][y + 1]);
        }
        (_, 99) => {
            neighbours.push(grid[x - 1][y]);
            neighbours.push(grid[x + 1][y]);
            neighbours.push(grid[x - 1][y - 1]);
            neighbours.push(grid[x][y - 1]);
            neighbours.push(grid[x + 1][y - 1]);
        }
        _ => {
            neighbours.push(grid[x - 1][y]);
            neighbours.push(grid[x + 1][y]);
            neighbours.push(grid[x - 1][y - 1]);
            neighbours.push(grid[x][y - 1]);
            neighbours.push(grid[x + 1][y - 1]);
            neighbours.push(grid[x - 1][y + 1]);
            neighbours.push(grid[x][y + 1]);
            neighbours.push(grid[x + 1][y + 1]);
        }
    }

    neighbours
}

fn change_light(light: bool, neighbours: Vec<bool>) -> bool {
    let neigh_count = neighbours.iter().filter(|&n| *n == true).count();

    match (light, neigh_count) {
        (true, 2 | 3) => true,
        (true, _) => false,
        (false, 3) => true,
        _ => false,
    }
}

fn change_grid(grid: [[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut new_grid: [[bool; 100]; 100] = [[false; 100]; 100];

    grid.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, &light)| {
            let neighbours = get_neighbours(&grid, x, y);
            match (x, y) {
                // second part
                (0, 0) | (0, 99) | (99, 0) | (99, 99) => new_grid[x][y] = true,
                // first part
                _ => new_grid[x][y] = change_light(light, neighbours),
            }
        })
    });

    new_grid
}
