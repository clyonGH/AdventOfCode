use md5;
use std::fs::File;
use std::io::prelude::*;

const GOAL_X: usize = 3;
const GOAL_Y: usize = 3;

#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
    g: u32,
    f: u32,
    path: String,
}

fn main() {
    let mut file = File::open("inputs/input_2016_17.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let santa_coord: Node = Node {
        x: 0,
        y: 0,
        g: 0,
        f: heuristics(0, 0, (GOAL_X, GOAL_Y)),
        path: "".to_string(),
    };

    let mut nodes_to_visit: Vec<Node> = vec![santa_coord.clone()];
    let mut current_node: Node = nodes_to_visit[0].clone();

    // first part
    while current_node.x != GOAL_X || current_node.y != GOAL_Y {
        // getting the lowest f score in nodes_to_visit
        let mut i: usize = 0;
        let mut lowest_f = nodes_to_visit[i].f;
        nodes_to_visit.iter().enumerate().for_each(|(index, node)| {
            if node.f < lowest_f {
                lowest_f = node.f;
                i = index;
            }
        });
        current_node = nodes_to_visit[i].clone();
        nodes_to_visit.remove(i);

        let current_neighbours = get_neighbours(
            current_node.x,
            current_node.y,
            current_node.g,
            &data,
            &current_node.path,
        );

        current_neighbours
            .iter()
            .for_each(|n| nodes_to_visit.push(n.clone()));

        if current_node.x == GOAL_X && current_node.y == GOAL_Y {
            println!("the shortest path is {:?}", current_node.path);
        }
    }

    // second part
    let mut longest_paths: Vec<usize> = Vec::new();
    nodes_to_visit = vec![santa_coord];
    while !nodes_to_visit.is_empty() {
        // getting the lowest f score in nodes_to_visit
        let mut i: usize = 0;
        let mut highest_f = nodes_to_visit[i].f;
        nodes_to_visit.iter().enumerate().for_each(|(index, node)| {
            if node.f > highest_f {
                highest_f = node.f;
                i = index;
            }
        });
        current_node = nodes_to_visit[i].clone();
        nodes_to_visit.remove(i);

        if current_node.x != GOAL_X || current_node.y != GOAL_Y {
            let current_neighbours = get_neighbours(
                current_node.x,
                current_node.y,
                current_node.g,
                &data,
                &current_node.path,
            );

            current_neighbours
                .iter()
                .for_each(|n| nodes_to_visit.push(n.clone()));
        }

        if current_node.x == GOAL_X && current_node.y == GOAL_Y {
            longest_paths.push(current_node.path.len());
        }
    }

    println!(
        "the longest path has a length of {:?} steps",
        longest_paths.iter().max().unwrap()
    );
}

fn heuristics(x: usize, y: usize, (goal_x, goal_y): (usize, usize)) -> u32 {
    (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32
}

fn get_neighbours(
    current_x: usize,
    current_y: usize,
    current_g: u32,
    data: &str,
    path: &str,
) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();

    let data_incr = format!("{data}{path}");
    let digest = md5::compute(data_incr);
    let digest_hex = format!("{:x}", digest);
    let new_g = current_g + 1;

    digest_hex[0..4]
        .chars()
        .enumerate()
        .for_each(|(index, c)| match c {
            'b'..='f' => match (index, current_x, current_y) {
                (0, _, 1..=3) => {
                    let new_y = current_y - 1;
                    let new_path = format!("{path}U");
                    neighbours.push(Node {
                        x: current_x,
                        y: new_y,
                        g: new_g,
                        f: new_g + heuristics(current_x, new_y, (GOAL_X, GOAL_Y)),
                        path: new_path,
                    });
                }
                (1, _, 0..=2) => {
                    let new_y = current_y + 1;
                    let new_path = format!("{path}D");
                    neighbours.push(Node {
                        x: current_x,
                        y: new_y,
                        g: new_g,
                        f: new_g + heuristics(current_x, new_y, (GOAL_X, GOAL_Y)),
                        path: new_path,
                    });
                }
                (2, 1..=3, _) => {
                    let new_x = current_x - 1;
                    let new_path = format!("{path}L");
                    neighbours.push(Node {
                        x: new_x,
                        y: current_y,
                        g: new_g,
                        f: new_g + heuristics(new_x, current_y, (GOAL_X, GOAL_Y)),
                        path: new_path,
                    });
                }
                (3, 0..=2, _) => {
                    let new_x = current_x + 1;
                    let new_path = format!("{path}R");
                    neighbours.push(Node {
                        x: new_x,
                        y: current_y,
                        g: new_g,
                        f: new_g + heuristics(new_x, current_y, (GOAL_X, GOAL_Y)),
                        path: new_path,
                    });
                }
                _ => (),
            },
            _ => (),
        });

    neighbours
}
