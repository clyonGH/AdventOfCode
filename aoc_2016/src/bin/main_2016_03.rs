use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2016_03.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let all_triangles: Vec<[u16; 3]> = data
        .lines()
        .map(|line| {
            let re = Regex::new(r"[ ]*([0-9]*)[ ]*([0-9]*)[ ]*([0-9]*)").unwrap();
            let caps = re.captures(line).unwrap();

            [
                caps.get(1).unwrap().as_str().parse::<u16>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<u16>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<u16>().unwrap(),
            ]
        })
        .collect();

    let real_triangles = all_triangles
        .iter()
        .filter(|&triangle| {
            let mut sorted_triangle = triangle.clone();
            sorted_triangle.sort();
            sorted_triangle[0] + sorted_triangle[1] > sorted_triangle[2]
        })
        .count();

    println!("{:?} of the listed triangles are possible", real_triangles);

    // second part
    let mut all_v_triangles: Vec<[u16; 3]> = Vec::new();
    (0..=all_triangles.len() - 3).for_each(|row| {
        if row % 3 == 0 || row == 0 {
            (0..3).for_each(|column| {
                all_v_triangles.push([
                    all_triangles[row][column],
                    all_triangles[row + 1][column],
                    all_triangles[row + 2][column],
                ]);
            });
        }
    });

    let real_v_triangles = all_v_triangles
        .iter()
        .filter(|&triangle| {
            let mut sorted_triangle = triangle.clone();
            sorted_triangle.sort();
            sorted_triangle[0] + sorted_triangle[1] > sorted_triangle[2]
        })
        .count();

    println!(
        "reading by columns, {:?} of the listed triangles are possible",
        real_v_triangles
    );
}
