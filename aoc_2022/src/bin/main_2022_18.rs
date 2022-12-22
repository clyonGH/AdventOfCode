use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

fn main() {
    let mut file = File::open("inputs/input_2022_18.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_coord: Vec<Coord> = data
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.split(',').collect();

            Coord {
                x: split_line[0].parse::<usize>().unwrap(),
                y: split_line[1].parse::<usize>().unwrap(),
                z: split_line[2].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let mut grid: [[[bool; 25]; 25]; 25] = [[[false; 25]; 25]; 25];

    // first part
    let mut surface_area: usize = 0;
    let mut surface_borders: usize = 0;
    all_coord.iter().for_each(|coord| {
        grid[coord.x][coord.y][coord.z] = true;
        surface_area += 6 - check_around(*coord, &grid);

        // for the second part
        if coord.x == 0 {
            surface_borders += 1;
        }
        if coord.y == 0 {
            surface_borders += 1;
        }
        if coord.z == 0 {
            surface_borders += 1;
        }
    });

    println!("the surface area of your scanned lava droplet is: {surface_area}");

    // second part
    surface_area = 0;
    let start_coord = Coord { x: 0, y: 0, z: 0 };
    let end_coord = Coord {
        x: 24,
        y: 24,
        z: 24,
    };
    breadth_first_search(start_coord, end_coord, &grid, &mut surface_area);

    println!(
        "the exterior surface area of your scanned lava droplet is: {:?}",
        surface_area + surface_borders
    );
}

fn check_around(coord: Coord, grid: &[[[bool; 25]; 25]; 25]) -> usize {
    let mut count: usize = 0;

    if coord.y < 24 && grid[coord.x][coord.y + 1][coord.z] {
        count += 2;
    }

    if coord.y != 0 && grid[coord.x][coord.y - 1][coord.z] {
        count += 2;
    }

    if coord.x < 24 && grid[coord.x + 1][coord.y][coord.z] {
        count += 2;
    }

    if coord.x != 0 && grid[coord.x - 1][coord.y][coord.z] {
        count += 2;
    }

    if coord.z < 24 && grid[coord.x][coord.y][coord.z + 1] {
        count += 2;
    }

    if coord.z != 0 && grid[coord.x][coord.y][coord.z - 1] {
        count += 2;
    }

    count
}

fn breadth_first_search(
    src: Coord,
    dest: Coord,
    grid: &[[[bool; 25]; 25]; 25],
    surface_area: &mut usize,
) {
    let mut already_visited: HashSet<Coord> = HashSet::new();
    let mut current_nodes: Vec<Coord> = vec![src];
    already_visited.insert(src);

    loop {
        current_nodes = current_nodes
            .iter()
            .flat_map(|node| {
                get_neighbours(grid, node.x, node.y, node.z, surface_area)
                    .iter()
                    .filter(|neighbour| already_visited.insert(**neighbour))
                    .map(|neighbour| *neighbour)
                    .collect::<Vec<Coord>>()
            })
            .collect();

        if current_nodes.iter().find(|&&node| node == dest).is_some() {
            break;
        }
    }
}

fn get_neighbours(
    grid: &[[[bool; 25]; 25]; 25],
    current_x: usize,
    current_y: usize,
    current_z: usize,
    surface_area: &mut usize,
) -> Vec<Coord> {
    let mut neighbours: Vec<Coord> = Vec::new();

    // right
    let mut new_x = current_x + 1;
    let mut start_coord = Coord {
        x: new_x,
        y: current_y,
        z: current_z,
    };
    if current_x < 24 {
        if grid[new_x][current_y][current_z] {
            *surface_area += 1;
        } else {
            neighbours.push(start_coord);
        }
    }

    // left
    new_x = current_x - 1;
    start_coord = Coord {
        x: new_x,
        y: current_y,
        z: current_z,
    };
    if current_x != 0 {
        if grid[new_x][current_y][current_z] {
            *surface_area += 1;
        } else {
            neighbours.push(start_coord);
        }
    }

    // up
    let mut new_y = current_y + 1;
    start_coord = Coord {
        x: current_x,
        y: new_y,
        z: current_z,
    };
    if current_y < 24 {
        if grid[current_x][new_y][current_z] {
            *surface_area += 1;
        } else {
            neighbours.push(start_coord);
        }
    }

    // down
    new_y = current_y - 1;
    start_coord = Coord {
        x: current_x,
        y: new_y,
        z: current_z,
    };
    if current_y != 0 {
        if grid[current_x][new_y][current_z] {
            *surface_area += 1;
        } else {
            neighbours.push(start_coord);
        }
    }

    // front
    let mut new_z = current_z + 1;
    start_coord = Coord {
        x: current_x,
        y: current_y,
        z: new_z,
    };
    if current_z < 24 {
        if grid[current_x][current_y][new_z] {
            *surface_area += 1;
        } else {
            neighbours.push(start_coord);
        }
    }

    // back
    new_z = current_z - 1;
    start_coord = Coord {
        x: current_x,
        y: current_y,
        z: new_z,
    };
    if current_z != 0 {
        if grid[current_x][current_y][new_z] {
            *surface_area += 1;
        } else {
            neighbours.push(start_coord);
        }
    }

    neighbours
}
