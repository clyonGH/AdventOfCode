use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const E_VAL: u8 = 69;
const S_VAL: u8 = 83;
const A_MIN_VAL: u8 = 97;
const Z_MIN_VAL: u8 = 122;

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    g: u32,
    f: u32,
}

fn main() {
    let mut file = File::open("inputs/input_2022_12.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let elevations: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    // find start and end point
    let (start_x, start_y) = elevations
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            let x = row.iter().position(|&x| x == 'S');
            if let Some(found_x) = x {
                Some((found_x, y))
            } else {
                None
            }
        })
        .unwrap();

    let (goal_x, goal_y) = elevations
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            let x = row.iter().position(|&x| x == 'E');
            if let Some(found_x) = x {
                Some((found_x, y))
            } else {
                None
            }
        })
        .unwrap();

    // first part
    let found_g = a_star(&elevations, start_x, start_y, (goal_x, goal_y));
    println!(
        "the fewest number of steps required to reach E is {}",
        found_g
    );

    // second part
    let mut all_starts: Vec<(usize, usize)> = Vec::new();
    let mut found_gs: Vec<u32> = Vec::new();
    (0..elevations.len()).for_each(|y| all_starts.push((0, y)));
    (0..all_starts.len()).for_each(|i| {
        let (x, y) = all_starts[i];
        found_gs.push(a_star(&elevations, x, y, (goal_x, goal_y)));
    });

    found_gs.sort();

    println!(
        "from any square with elevation a, the fewest number of steps required to reach E is {}",
        found_gs[0]
    );
}

fn a_star(
    elevations: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    (goal_x, goal_y): (usize, usize),
) -> u32 {
    let coord: Node = Node {
        x: start_x,
        y: start_y,
        g: 0,
        f: heuristics(&elevations, start_x, start_y, (goal_x, goal_y)),
    };

    let mut nodes_to_visit: Vec<Node> = vec![coord];
    let mut current_node: Node = nodes_to_visit[0];
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    visited_nodes.insert((current_node.x, current_node.y));

    // first part
    while current_node.x != goal_x || current_node.y != goal_y {
        // getting the lowest f score in nodes_to_visit
        let mut i: usize = 0;

        let mut lowest_f = nodes_to_visit[i].f;
        nodes_to_visit.iter().enumerate().for_each(|(index, node)| {
            if node.f < lowest_f {
                lowest_f = node.f;
                i = index;
            }
        });
        current_node = nodes_to_visit[i];

        nodes_to_visit.remove(i);

        let current_neighbours = get_neighbours(
            &elevations,
            current_node.x,
            current_node.y,
            current_node.g,
            (goal_x, goal_y),
        );

        current_neighbours
            .iter()
            .filter(|&n| visited_nodes.insert((n.x, n.y)))
            .for_each(|&n| nodes_to_visit.push(n));
    }

    current_node.g
}

fn heuristics(
    elevations: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    (goal_x, goal_y): (usize, usize),
) -> u32 {
    let diff_el = 123 - elevations[y][x] as u32;
    let dist_man = (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32;

    if diff_el > dist_man {
        diff_el
    } else {
        dist_man
    }
}

fn can_go_right(elevations: &Vec<Vec<char>>, current_x: &usize, current_y: &usize) -> bool {
    let mut current_pos = elevations[*current_y][*current_x] as u8;
    let mut dest_pos = elevations[*current_y][*current_x + 1] as u8;

    if current_pos == S_VAL {
        current_pos = A_MIN_VAL;
    }
    if dest_pos == E_VAL {
        dest_pos = Z_MIN_VAL;
    }

    if current_pos + 1 >= dest_pos {
        true
    } else {
        false
    }
}

fn can_go_left(elevations: &Vec<Vec<char>>, current_x: &usize, current_y: &usize) -> bool {
    let mut current_pos = elevations[*current_y][*current_x] as u8;
    let mut dest_pos = elevations[*current_y][*current_x - 1] as u8;

    if current_pos == S_VAL {
        current_pos = A_MIN_VAL;
    }
    if dest_pos == E_VAL {
        dest_pos = Z_MIN_VAL;
    }

    if current_pos + 1 >= dest_pos {
        true
    } else {
        false
    }
}

fn can_go_up(elevations: &Vec<Vec<char>>, current_x: &usize, current_y: &usize) -> bool {
    let mut current_pos = elevations[*current_y][*current_x] as u8;
    let mut dest_pos = elevations[*current_y + 1][*current_x] as u8;

    if current_pos == S_VAL {
        current_pos = A_MIN_VAL;
    }
    if dest_pos == E_VAL {
        dest_pos = Z_MIN_VAL;
    }

    if current_pos + 1 >= dest_pos {
        true
    } else {
        false
    }
}

fn can_go_down(elevations: &Vec<Vec<char>>, current_x: &usize, current_y: &usize) -> bool {
    let mut current_pos = elevations[*current_y][*current_x] as u8;
    let mut dest_pos = elevations[*current_y - 1][*current_x] as u8;

    if current_pos == S_VAL {
        current_pos = A_MIN_VAL;
    }
    if dest_pos == E_VAL {
        dest_pos = Z_MIN_VAL;
    }

    if current_pos + 1 >= dest_pos {
        true
    } else {
        false
    }
}

fn get_neighbours(
    elevations: &Vec<Vec<char>>,
    current_x: usize,
    current_y: usize,
    current_g: u32,
    (goal_x, goal_y): (usize, usize),
) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();
    let new_g = current_g + 1;
    let elevations_width = elevations[0].len();
    let elevations_height = elevations.len();

    // right
    let mut new_x = current_x + 1;
    if current_x + 1 != elevations_width && can_go_right(elevations, &current_x, &current_y) {
        neighbours.push(Node {
            x: new_x,
            y: current_y,
            g: new_g,
            f: new_g + heuristics(elevations, new_x, current_y, (goal_x, goal_y)),
        });
    }

    // left
    new_x = current_x - 1;
    if current_x != 0 && can_go_left(elevations, &current_x, &current_y) {
        neighbours.push(Node {
            x: new_x,
            y: current_y,
            g: new_g,
            f: new_g + heuristics(elevations, new_x, current_y, (goal_x, goal_y)),
        });
    }

    // up
    let mut new_y = current_y + 1;
    if current_y + 1 != elevations_height && can_go_up(&elevations, &current_x, &current_y) {
        neighbours.push(Node {
            x: current_x,
            y: new_y,
            g: new_g,
            f: new_g + heuristics(elevations, current_x, new_y, (goal_x, goal_y)),
        });
    }

    // down
    new_y = current_y - 1;
    if current_y != 0 && can_go_down(elevations, &current_x, &current_y) {
        neighbours.push(Node {
            x: current_x,
            y: new_y,
            g: new_g,
            f: new_g + heuristics(elevations, current_x, new_y, (goal_x, goal_y)),
        });
    }

    neighbours
}
