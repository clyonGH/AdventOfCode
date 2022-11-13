use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2016_09.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let decompressed_data = decompress_data(data.clone());
    println!("the decompressed length is: {}", decompressed_data.len());

    // second part
    println!(
        "using this improved format the decompressed length is: {}",
        improved_decompress_data(&data)
    );
}

fn decompress_data(data: String) -> String {
    let mut marker: bool = false;
    let mut pattern: bool = false;
    let mut nb_chars: u16 = 0;
    let mut char_count: u16 = 0;
    let mut nb_times: u16 = 0;
    let mut processed_data: String = String::from("");
    let mut decompressed_data: String = String::from("");

    data.chars().for_each(|c| match (c, pattern) {
        ('(', false) => {
            decompressed_data.push_str(processed_data.as_str());
            processed_data.clear();
            char_count = 0;
            marker = true;
        }
        (')', false) => {
            marker = false;
            pattern = true;
            nb_times = processed_data.parse::<u16>().unwrap();
            processed_data.clear();
        }
        ('x', false) => {
            nb_chars = processed_data.parse::<u16>().unwrap();
            processed_data.clear();
        }
        _ => {
            processed_data.push(c);
            if !marker {
                char_count += 1;
                if char_count == nb_chars {
                    (0..nb_times).for_each(|_| {
                        decompressed_data.push_str(processed_data.as_str());
                    });
                    pattern = false;
                    processed_data.clear();
                    char_count = 0;
                }
            }
        }
    });

    decompressed_data
}

fn improved_decompress_data(data: &str) -> u64 {
    let mut decompressed_size: u64 = 0;
    let mut data_index: usize = 0;
    let data_bytes = data.as_bytes();

    while data_index < data.len() {
        match data_bytes[data_index] {
            b'(' => {
                let x_index = data[data_index..].find('x').unwrap() + data_index;
                let nb_chars: usize = data[data_index + 1..x_index].parse().unwrap();
                let end_marker_index = data[data_index..].find(')').unwrap() + data_index;
                let nb_times: u64 = data[x_index + 1..end_marker_index].parse().unwrap();

                data_index = end_marker_index + nb_chars + 1;

                decompressed_size += improved_decompress_data(
                    &data[end_marker_index + 1..=end_marker_index + nb_chars],
                ) * nb_times;
            }
            _ => {
                decompressed_size += 1;
                data_index += 1;
            }
        }
    }

    decompressed_size
}
