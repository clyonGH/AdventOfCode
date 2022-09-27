use bytelines::{self, ByteLines};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut file = File::open("inputs/input_2015_08.txt").expect("Error: File not found");
    let reader = BufReader::new(file);
    let lines = ByteLines::new(reader);
    let mut nb_str_lit = 0;
    let mut nb_char_mem = 0;
    let mut total_nb_char_mem = 0;

    // first part
    lines.into_iter().for_each(|vec_line| {
        nb_str_lit += vec_line.as_ref().unwrap().len();
        nb_char_mem = vec_line.as_ref().unwrap().len() - 2; // removing the 2 quotes at the beginning and at the end
        vec_line
            .as_ref()
            .unwrap()
            .windows(2)
            .enumerate() // getting the index where pair found to slice it later
            .for_each(|(index, window)| {
                if index == 0
                    || (vec_line.as_ref().unwrap()[index - 1] != 92)
                    || (index >= 2
                        && vec_line.as_ref().unwrap()[index - 1] == 92
                        && vec_line.as_ref().unwrap()[index - 2] == 92)
                {
                    // bytes values of ": 34   backslash: 92   x: 120
                    match (window[0], window[1]) {
                        (92, 34) => {
                            nb_char_mem -= 1; // removing \"
                        }
                        (92, 92) => {
                            nb_char_mem -= 1; // removing \\
                        }
                        (92, 120) => {
                            nb_char_mem -= 3; // removing \x and 2 characters for the hex value
                        }
                        _ => (),
                    }
                }
            });
        total_nb_char_mem += nb_char_mem;
    });

    println!("number of string literals: {}", nb_str_lit);
    println!("number of characters in memory: {}", total_nb_char_mem);
    println!("first answer: {}", nb_str_lit - total_nb_char_mem - 1); // removing the whitespace

    // second part
    file = File::open("inputs/input_2015_08.txt").expect("Error: File not found");
    let mut data: String = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut nb_new_str = 0;

    data.lines().into_iter().for_each(|vec_line| {
        nb_new_str += vec_line.len() + 2; // adding the 2 quotes at the beginning and at the end
        vec_line.bytes().for_each(|current_byte| {
            // bytes values of ": 34   backslash: 92
            match current_byte {
                92 | 34 => {
                    nb_new_str += 1; // adding 1 \
                }
                _ => (),
            }
        });
    });

    println!(
        "number of characters representing the newly encoded strings: {}",
        nb_new_str
    );
    println!("second answer: {}", nb_new_str - nb_str_lit);
}
