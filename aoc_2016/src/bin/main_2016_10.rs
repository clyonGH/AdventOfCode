use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2016_10.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut all_bots: HashMap<u16, [u16; 2]> = HashMap::new();
    let mut value_given: Vec<&str> = Vec::new();
    data.lines().for_each(|line| {
        if line.starts_with("value") {
            let split_line: Vec<&str> = line.split(' ').collect();
            all_bots
                .entry(split_line[5].parse().unwrap())
                .and_modify(|values| {
                    values[1] = split_line[1].parse().unwrap();
                })
                .or_insert([split_line[1].parse().unwrap(), 0]);
        } else {
            value_given.push(line);
        }
    });

    // first part
    let mut all_outputs: HashMap<u16, u16> = HashMap::new();
    let mut found: bool = false;
    let (mut output_0, mut output_1, mut output_2) = (false, false, false);
    while !(found && output_0 && output_1 && output_2) {
        value_given.iter().for_each(|&line| {
            let split_line: Vec<&str> = line.split(' ').collect();

            let (mut lowest_value, mut highest_value) = (0, 0);
            let bot_entry = all_bots.get(&split_line[1].parse::<u16>().unwrap());
            let mut entry_found: bool = false;
            match bot_entry {
                Some(values) => {
                    if values[1] != 0 {
                        (lowest_value, highest_value) = if values[0] < values[1] {
                            (values[0], values[1])
                        } else {
                            (values[1], values[0])
                        };

                        if lowest_value == 17 && highest_value == 61 {
                            println!(
                                "the number of the bot is: {:?}",
                                split_line[1].parse::<u16>().unwrap()
                            );
                            found = true;
                        }

                        entry_found = true;
                    }
                }
                None => (),
            }

            if entry_found {
                // give the lowest value
                if split_line[5].eq("output") {
                    let output_number = split_line[6].parse().unwrap();
                    match output_number {
                        0 => output_0 = true,
                        1 => output_1 = true,
                        2 => output_2 = true,
                        _ => (),
                    }
                    all_outputs.insert(output_number, lowest_value);
                } else {
                    all_bots
                        .entry(split_line[6].parse().unwrap())
                        .and_modify(|values| {
                            values[1] = lowest_value;
                        })
                        .or_insert([lowest_value, 0]);
                }

                // give the highest value
                if split_line[10].eq("output") {
                    let output_number = split_line[11].parse().unwrap();
                    match output_number {
                        0 => output_0 = true,
                        1 => output_1 = true,
                        2 => output_2 = true,
                        _ => (),
                    }
                    all_outputs.insert(output_number, highest_value);
                } else {
                    all_bots
                        .entry(split_line[11].parse().unwrap())
                        .and_modify(|values| {
                            values[1] = highest_value;
                        })
                        .or_insert([highest_value, 0]);
                }

                // reset values once given
                all_bots.insert(split_line[1].parse::<u16>().unwrap(), [0, 0]);
            }
        });
    }

    // second part
    let multiplied_values: u16 =
        all_outputs.get(&0).unwrap() * all_outputs.get(&1).unwrap() * all_outputs.get(&2).unwrap();

    println!("multiplying the values of the outputs 0, 1 and 2 results in: {multiplied_values}");
}
