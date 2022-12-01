use std::collections::HashSet;

const GOAL_X: usize = 31;
const GOAL_Y: usize = 39;
const MAX_STEPS: u32 = 50;

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    g: u32,
    f: u32,
}

fn main() {
    let input: usize = 1352;

    let santa_coord: Node = Node {
        x: 1,
        y: 1,
        g: 0,
        f: heuristics(1, 1, (GOAL_X, GOAL_Y)),
    };

    let mut nodes_to_visit: Vec<Node> = vec![santa_coord];
    let mut current_node: Node = nodes_to_visit[0];
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    visited_nodes.insert((current_node.x, current_node.y));

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
        current_node = nodes_to_visit[i];

        nodes_to_visit.remove(i);

        let current_neighbours =
            get_neighbours(current_node.x, current_node.y, current_node.g, input);

        current_neighbours
            .iter()
            .filter(|&n| visited_nodes.insert((n.x, n.y)))
            .for_each(|&n| nodes_to_visit.push(n));
    }

    println!(
        "the fewest number of steps required to reach 31,39 is {}",
        current_node.g
    );

    // second part
    nodes_to_visit = vec![santa_coord];
    current_node = nodes_to_visit[0];
    visited_nodes.clear();
    visited_nodes.insert((current_node.x, current_node.y));
    loop {
        // getting the lowest g score in nodes_to_visit
        let mut i: usize = 0;
        let mut lowest_g = MAX_STEPS;
        nodes_to_visit.iter().enumerate().for_each(|(index, node)| {
            if node.g < lowest_g {
                lowest_g = node.g;
                i = index;
            }
        });
        current_node = nodes_to_visit[i];
        nodes_to_visit.remove(i);

        println!("current node: {:?}", current_node);

        if current_node.g == MAX_STEPS {
            println!(
                "{:?} locations can be reached in at most 50 steps",
                visited_nodes.len()
            );

            break;
        }

        let current_neighbours =
            get_neighbours(current_node.x, current_node.y, current_node.g, input);

        current_neighbours
            .iter()
            .filter(|&n| visited_nodes.insert((n.x, n.y)))
            .for_each(|&n| nodes_to_visit.push(n));
    }
}

fn is_open_space(x: usize, y: usize, d_fav_num: usize) -> bool {
    let nb: usize = x * x + 3 * x + 2 * x * y + y + y * y + d_fav_num;

    nb.count_ones() % 2 == 0
}

fn heuristics(x: usize, y: usize, (goal_x, goal_y): (usize, usize)) -> u32 {
    (goal_x.abs_diff(x) + goal_y.abs_diff(y)) as u32
}

fn get_neighbours(
    current_x: usize,
    current_y: usize,
    current_g: u32,
    d_fav_num: usize,
) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();

    let new_g = current_g + 1;

    let mut new_x = current_x + 1;
    if is_open_space(new_x, current_y, d_fav_num) {
        neighbours.push(Node {
            x: new_x,
            y: current_y,
            g: new_g,
            f: new_g + heuristics(new_x, current_y, (GOAL_X, GOAL_Y)),
        });
    }

    new_x = current_x - 1;
    if current_x != 0 && is_open_space(new_x, current_y, d_fav_num) {
        neighbours.push(Node {
            x: new_x,
            y: current_y,
            g: new_g,
            f: new_g + heuristics(new_x, current_y, (GOAL_X, GOAL_Y)),
        });
    }

    let mut new_y = current_y + 1;
    if is_open_space(current_x, new_y, d_fav_num) {
        neighbours.push(Node {
            x: current_x,
            y: new_y,
            g: new_g,
            f: new_g + heuristics(current_x, new_y, (GOAL_X, GOAL_Y)),
        });
    }

    new_y = current_y - 1;
    if current_y != 0 && is_open_space(current_x, new_y, d_fav_num) {
        neighbours.push(Node {
            x: current_x,
            y: new_y,
            g: new_g,
            f: new_g + heuristics(current_x, new_y, (GOAL_X, GOAL_Y)),
        });
    }

    neighbours
}
