use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("inputs/input_2015_05.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let nice_string_count = data
        .lines()
        .filter(|&line| {
            !is_naughty_string(line) && has_three_vowels(line) && has_two_letters_in_a_row(line)
        })
        .count();

    println!("there are {nice_string_count} nice strings");

    // second part
    let new_nice_string_count = data
        .lines()
        .filter(|&line| has_twice_a_pair_of_two_letters(line) && has_one_letter_in_between(line))
        .count();
    println!("there are now {new_nice_string_count} nice strings");
}

fn has_three_vowels(input_line: &str) -> bool {
    let vowels = "aeiou";

    let vowels_found = input_line
        .chars()
        .filter(|&input_char| vowels.contains(input_char))
        .count();
    if vowels_found >= 3 {
        true
    } else {
        false
    }
}

fn has_two_letters_in_a_row(input_line: &str) -> bool {
    input_line
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|window| window[0] == window[1])
}

fn is_naughty_string(input_line: &str) -> bool {
    let substrings = ["ab", "cd", "pq", "xy"];
    substrings.iter().any(|pat| input_line.contains(pat))
}

fn has_twice_a_pair_of_two_letters(input_line: &str) -> bool {
    let collected_windows = input_line.chars().collect::<Vec<char>>();
    let mut all_windows = collected_windows.windows(2).enumerate();

    let find_another_pair = all_windows.any(|(index, window)| {
        let window_str = window.iter().collect::<String>();
        input_line[index + 2..].contains(&window_str)
    });

    find_another_pair
}

fn has_one_letter_in_between(input_line: &str) -> bool {
    input_line
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|window| window[0] == window[2])
}
