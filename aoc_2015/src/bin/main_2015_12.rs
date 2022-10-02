use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_12.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    println!("sum of all numbers: {}", sum_all(&data));

    // second part
    let parsed_data: Value = serde_json::from_str(&data).unwrap();
    println!(
        "sum of all numbers considering the red: {:?}",
        find_red(parsed_data)
    );
}

fn sum_all(data: &str) -> i64 {
    let mut prev_c: bool = false;
    let mut found_nb: String = "".to_string();
    let mut all_nb: Vec<i64> = Vec::new();

    data.chars().for_each(|c| {
        if c == '-' || c.is_ascii_digit() {
            if prev_c {
                found_nb = found_nb.clone() + c.to_string().as_str();
            } else {
                found_nb = c.to_string();
            }
            prev_c = true;
        } else {
            if prev_c {
                all_nb.push(found_nb.parse::<i64>().unwrap());
                found_nb.clear();
                prev_c = false;
            }
        }
    });

    all_nb.iter().sum::<i64>()
}

fn find_red(json_object: Value) -> Option<i64> {
    match json_object {
        Value::Bool(_) => Some(0),
        Value::Null => Some(0),
        Value::Number(n) => n.as_i64(),
        Value::String(s) => {
            if s.eq("red") {
                None
            } else {
                Some(0)
            }
        }
        Value::Array(a) => {
            let res_array = Some(a.into_iter().flat_map(|v| find_red(v)).sum());
            res_array
        }
        Value::Object(ob) => {
            let mut map_ob = ob.clone().into_iter().map(|(_k, v)| find_red(v));
            if map_ob.any(|op| op.is_none()) {
                Some(0)
            } else {
                let res_obj = Some(ob.clone().into_iter().flat_map(|(_k, v)| find_red(v)).sum());
                res_obj
            }
        }
    }
}
