use std::fs::File;
use std::io::prelude::*;
use std::str;

fn main() {
    let mut file = File::open("inputs/input_2015_11.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first solution was: hxbxxyzz so new data is first solution with one incrementation
    data = "hxbxxzaa".to_string();

    increment(data);
}

fn increment(mut password: String) -> () {
    while !check_three_letters(&password)
        || !check_confusing_letters(&password)
        || !check_pairs(&password)
    {
        let new_pwd = password
            .as_bytes()
            .iter()
            .rev()
            .fold((Vec::new(), true), incr_char)
            .0
            .into_iter()
            .rev()
            .collect();

        password = String::from_utf8(new_pwd).unwrap();
    }

    println!("the new password is: {}", password);
}

fn incr_char(mut acc: (Vec<u8>, bool), &current_char: &u8) -> (Vec<u8>, bool) {
    if acc.1 {
        let mut new_char = current_char + 1;
        if new_char > 122 {
            new_char = (new_char % 122) + 96;
            acc.0.push(new_char);
            (acc.0, true)
        } else {
            acc.0.push(new_char);
            (acc.0, false)
        }
    } else {
        acc.0.push(current_char);
        acc
    }
}

fn check_three_letters(password: &str) -> bool {
    password
        .as_bytes() // collected to Vec<char> for windows
        .windows(3)
        .any(|window| window[0] == window[1] - 1 && window[1] - 1 == window[2] - 2)
}

fn check_confusing_letters(password: &str) -> bool {
    if password.contains('i') || password.contains('o') || password.contains('l') {
        false
    } else {
        true
    }
}

fn check_pairs(password: &str) -> bool {
    password
        .as_bytes() // collected to Vec<char> for windows
        .windows(2)
        .enumerate() // getting the index where pair found to slice it later
        .filter_map(|(index, window)| {
            if window[0] == window[1] {
                Some(index)
            } else {
                None
            }
        }) // put the window of 2 inside a string
        .any(|index| {
            password[index + 2..]
                .as_bytes()
                .windows(2)
                .any(|window| window[0] == window[1]) // finding another pair
        })
}
