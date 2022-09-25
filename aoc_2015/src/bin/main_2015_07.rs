use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_07.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut direct_assign: Vec<(String, String)> = Vec::new(); // (first_input, output)
    let mut complement_assign: Vec<(String, String)> = Vec::new(); // (second_input, output)
    let mut other_assign: Vec<(String, String, String, String)> = Vec::new(); // (first_input, gate, second_input, output)
    let mut values_obtained: HashMap<String, u16> = HashMap::new();

    for line in data.lines() {
        let re = Regex::new(r"([a-z0-9]*)[ ]?([A-Z]*)[ ]?([a-z0-9]*) -> ([a-z]+)").unwrap();
        let caps = re.captures(line).unwrap();
        let first_input = caps.get(1);
        let gate = caps.get(2);
        let second_input = caps.get(3);
        let output = caps.get(4);

        match gate {
            Some(gate_found) if gate_found.as_str().is_empty() => direct_assign.push((
                first_input.unwrap().as_str().to_string(),
                output.unwrap().as_str().to_string(),
            )),
            Some(gate_found) if gate_found.as_str().eq("NOT") => complement_assign.push((
                second_input.unwrap().as_str().to_string(),
                output.unwrap().as_str().to_string(),
            )),
            Some(gate_found) => other_assign.push((
                first_input.unwrap().as_str().to_string(),
                gate_found.as_str().to_string(),
                second_input.unwrap().as_str().to_string(),
                output.unwrap().as_str().to_string(),
            )),
            None => {
                println!("no match for gate");
            }
        }
    }

    loop {
        // direct assignment
        direct_assign = direct_assign
            .into_iter()
            .filter(|(first_input, output)| {
                let first_input_value = first_input
                    .parse::<u16>()
                    .ok()
                    .or(values_obtained.get(first_input).cloned());

                match first_input_value {
                    Some(f) => {
                        values_obtained.insert(output.to_string(), f);
                        false
                    }
                    _ => true,
                }
            })
            .collect();

        // NOT gate
        complement_assign = complement_assign
            .into_iter()
            .filter(|(first_input, output)| {
                let first_input_value = values_obtained.get(first_input).cloned();

                match first_input_value {
                    Some(f) => {
                        values_obtained.insert(output.to_string(), !f);
                        false
                    }
                    _ => true,
                }
            })
            .collect();

        // other gates
        other_assign = other_assign
            .into_iter()
            .filter(|(first_input, gate, second_input, output)| {
                let first_input_value = first_input
                    .parse::<u16>()
                    .ok()
                    .or(values_obtained.get(first_input).cloned());

                let second_input_value = second_input
                    .parse::<u16>()
                    .ok()
                    .or(values_obtained.get(second_input).cloned());

                match (gate.as_str(), first_input_value, second_input_value) {
                    ("AND", Some(f), Some(s)) => {
                        values_obtained.insert(output.to_string(), f & s);
                        false
                    }
                    ("OR", Some(f), Some(s)) => {
                        values_obtained.insert(output.to_string(), f | s);
                        false
                    }
                    ("RSHIFT", Some(f), Some(s)) => {
                        values_obtained.insert(output.to_string(), f >> s);
                        false
                    }
                    ("LSHIFT", Some(f), Some(s)) => {
                        values_obtained.insert(output.to_string(), f << s);
                        false
                    }
                    _ => true,
                }
            })
            .collect();

        // first part
        if values_obtained.contains_key("a") {
            println!("the value of a is: {:?}", values_obtained.get("a"));
            break;
        }

        // second part: modify the input line 335 with "46065 -> b" and run again! :)
    }
}
