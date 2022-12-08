use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2022_07.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_dirs: HashMap<String, u32> = HashMap::new();

    let mut current_dir: String = "".to_string();
    data.lines().for_each(|line| {
        let split_line: Vec<&str> = line.split(' ').collect();
        match (split_line[0], split_line[1]) {
            ("$", "cd") => {
                if current_dir == "" || current_dir == "/" {
                    current_dir = current_dir.to_owned() + split_line[2];
                    all_dirs.insert(current_dir.clone(), 0);
                } else {
                    if split_line[2] != ".." {
                        current_dir = current_dir.to_owned() + "/" + split_line[2];
                        all_dirs.insert(current_dir.clone(), 0);
                    } else {
                        let end_range = current_dir.rfind('/').unwrap();
                        if end_range != 0 {
                            current_dir = current_dir[0..end_range].to_string();
                        } else {
                            current_dir = "/".to_string();
                        }
                    }
                }
            }
            ("$", "ls") => (),
            ("dir", _) => (),
            _ => {
                let new_size: u32 = split_line[0].parse::<u32>().unwrap();

                all_dirs.iter_mut().for_each(|(ak, av)| {
                    if current_dir.starts_with(ak) {
                        *av += new_size;
                    }
                });
            }
        }
    });

    // first part
    let filtered_dirs: Vec<u32> = all_dirs
        .values()
        .filter(|&&v| v <= 100000)
        .cloned()
        .collect();

    println!(
        "{:?} is the sum of the total sizes of directories with a total size of at most 100000",
        filtered_dirs.iter().sum::<u32>()
    );

    // second part
    let free_space = 70000000 - all_dirs.get("/").unwrap();
    let space_to_free = 30000000 - free_space;

    let ans_space: u32 = all_dirs
        .values()
        .filter(|&&v| v >= space_to_free)
        .min()
        .unwrap()
        .clone();

    println!("{ans_space} is the total size of the directory that needs to be deleted");
}
