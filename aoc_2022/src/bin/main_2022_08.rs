use std::fs::File;
use std::io::prelude::*;

const GRID_SIZE: usize = 99;

fn main() {
    let mut file = File::open("inputs/input_2022_08.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let tree_grid: Vec<Vec<u8>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // first part
    let mut visible_trees: u32 = 0;
    (1..GRID_SIZE - 1).for_each(|tree_line| {
        (1..GRID_SIZE - 1).for_each(|tree_column| {
            let tree_height = tree_grid[tree_line][tree_column];
            if is_left_visible(&tree_grid[tree_line], tree_column, tree_height)
                || is_right_visible(&tree_grid[tree_line], tree_column, tree_height)
                || is_up_visible(&tree_grid, tree_line, tree_column, tree_height)
                || is_down_visible(&tree_grid, tree_line, tree_column, tree_height)
            {
                visible_trees += 1;
            }
        })
    });

    visible_trees += 4 * (GRID_SIZE as u32 - 1);
    println!("{:?} trees are visible", visible_trees);

    // second part
    let mut max_scenic_score: u32 = 0;
    (1..GRID_SIZE - 1).for_each(|tree_line| {
        (1..GRID_SIZE - 1).for_each(|tree_column| {
            let tree_height = tree_grid[tree_line][tree_column];
            let scenic_score = left_scenic_score(&tree_grid[tree_line], tree_column, tree_height)
                * right_scenic_score(&tree_grid[tree_line], tree_column, tree_height)
                * up_scenic_score(&tree_grid, tree_line, tree_column, tree_height)
                * down_scenic_score(&tree_grid, tree_line, tree_column, tree_height);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        })
    });
    println!("the highest scenic score is: {:?}", max_scenic_score);
}

fn is_left_visible(tree_vec: &[u8], tree_column: usize, tree_height: u8) -> bool {
    (0..tree_column).all(|index| tree_vec[index] < tree_height)
}

fn is_right_visible(tree_vec: &[u8], tree_column: usize, tree_height: u8) -> bool {
    (tree_column + 1..GRID_SIZE).all(|index| tree_vec[index] < tree_height)
}

fn is_up_visible(
    tree_grid: &[Vec<u8>],
    tree_line: usize,
    tree_column: usize,
    tree_height: u8,
) -> bool {
    (0..tree_line).all(|index| tree_grid[index][tree_column] < tree_height)
}

fn is_down_visible(
    tree_grid: &[Vec<u8>],
    tree_line: usize,
    tree_column: usize,
    tree_height: u8,
) -> bool {
    (tree_line + 1..GRID_SIZE).all(|index| tree_grid[index][tree_column] < tree_height)
}

fn left_scenic_score(tree_vec: &[u8], tree_column: usize, tree_height: u8) -> u32 {
    let mut scenic_score: u32 = 0;
    (0..tree_column).for_each(|index| {
        if tree_vec[index] < tree_height {
            scenic_score += 1;
        } else {
            scenic_score = 1;
        }
    });
    scenic_score
}

fn right_scenic_score(tree_vec: &[u8], tree_column: usize, tree_height: u8) -> u32 {
    let mut scenic_score: u32 = 0;
    let mut score_found: bool = false;
    (tree_column + 1..GRID_SIZE).for_each(|index| {
        if !score_found {
            scenic_score += 1;
            if tree_vec[index] >= tree_height {
                score_found = true;
            }
        }
    });
    scenic_score
}

fn up_scenic_score(
    tree_grid: &[Vec<u8>],
    tree_line: usize,
    tree_column: usize,
    tree_height: u8,
) -> u32 {
    let mut scenic_score: u32 = 0;
    (0..tree_line).for_each(|index| {
        if tree_grid[index][tree_column] < tree_height {
            scenic_score += 1;
        } else {
            scenic_score = 1;
        }
    });
    scenic_score
}

fn down_scenic_score(
    tree_grid: &[Vec<u8>],
    tree_line: usize,
    tree_column: usize,
    tree_height: u8,
) -> u32 {
    let mut scenic_score: u32 = 0;
    let mut score_found: bool = false;
    (tree_line + 1..GRID_SIZE).for_each(|index| {
        if !score_found {
            scenic_score += 1;
            if tree_grid[index][tree_column] >= tree_height {
                score_found = true;
            }
        }
    });
    scenic_score
}
