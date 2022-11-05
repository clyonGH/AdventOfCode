use std::fs::File;
use std::io::prelude::*;

const MAX_LENGTH: u8 = 3;
const NEW_MAX_LENGTH: u8 = 5;

fn main() {
    let mut file = File::open("inputs/input_2016_02.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // first part
    let keypad: [[u8; MAX_LENGTH as usize]; MAX_LENGTH as usize] = [
        [1, 2, 3], // row 0
        [4, 5, 6], // row 1
        [7, 8, 9], // row 2
    ];

    // initial button is 5: row 1, column 1
    let mut button: (u8, u8) = (1, 1);
    let mut code: Vec<u8> = Vec::new();

    data.lines().for_each(|line| {
        line.chars().into_iter().for_each(|instruction| {
            match instruction {
                'U' => move_up(&mut button),
                'D' => move_down(&mut button),
                'R' => move_right(&mut button),
                'L' => move_left(&mut button),
                _ => button,
            };
        });
        code.push(keypad[button.0 as usize][button.1 as usize]);
    });

    println!("the bathroom code is: {:?}", code);

    // second part
    let new_keypad: [[char; NEW_MAX_LENGTH as usize]; NEW_MAX_LENGTH as usize] = [
        [' ', ' ', '1', ' ', ' '], // row 0
        [' ', '2', '3', '4', ' '], // row 1
        ['5', '6', '7', '8', '9'], // row 2
        [' ', 'A', 'B', 'C', ' '], // row 3
        [' ', ' ', 'D', ' ', ' '], // row 4
    ];

    // initial button is 5: row 2, column 0
    let mut new_button: (u8, u8) = (2, 0);
    let mut new_code: Vec<char> = Vec::new();

    data.lines().for_each(|line| {
        line.chars().into_iter().for_each(|instruction| {
            match instruction {
                'U' => new_move_up(&mut new_button, new_keypad),
                'D' => new_move_down(&mut new_button, new_keypad),
                'R' => new_move_right(&mut new_button, new_keypad),
                'L' => new_move_left(&mut new_button, new_keypad),
                _ => new_button,
            };
        });
        new_code.push(new_keypad[new_button.0 as usize][new_button.1 as usize]);
    });

    println!("the correct bathroom code is: {:?}", new_code);
}

fn move_up(position: &mut (u8, u8)) -> (u8, u8) {
    if position.0 > 0 {
        position.0 -= 1;
    }

    (position.0, position.1)
}

fn move_down(position: &mut (u8, u8)) -> (u8, u8) {
    if position.0 < MAX_LENGTH - 1 {
        position.0 += 1;
    }

    (position.0, position.1)
}

fn move_right(position: &mut (u8, u8)) -> (u8, u8) {
    if position.1 < MAX_LENGTH - 1 {
        position.1 += 1;
    }

    (position.0, position.1)
}

fn move_left(position: &mut (u8, u8)) -> (u8, u8) {
    if position.1 > 0 {
        position.1 -= 1;
    }

    (position.0, position.1)
}

fn new_move_up(
    position: &mut (u8, u8),
    new_keypad: [[char; NEW_MAX_LENGTH as usize]; NEW_MAX_LENGTH as usize],
) -> (u8, u8) {
    if position.0 > 0 && new_keypad[position.0 as usize - 1][position.1 as usize] != ' ' {
        position.0 -= 1;
    }

    (position.0, position.1)
}

fn new_move_down(
    position: &mut (u8, u8),
    new_keypad: [[char; NEW_MAX_LENGTH as usize]; NEW_MAX_LENGTH as usize],
) -> (u8, u8) {
    if position.0 < NEW_MAX_LENGTH - 1
        && new_keypad[position.0 as usize + 1][position.1 as usize] != ' '
    {
        position.0 += 1;
    }

    (position.0, position.1)
}

fn new_move_right(
    position: &mut (u8, u8),
    new_keypad: [[char; NEW_MAX_LENGTH as usize]; NEW_MAX_LENGTH as usize],
) -> (u8, u8) {
    if position.1 < NEW_MAX_LENGTH - 1
        && new_keypad[position.0 as usize][position.1 as usize + 1] != ' '
    {
        position.1 += 1;
    }

    (position.0, position.1)
}

fn new_move_left(
    position: &mut (u8, u8),
    new_keypad: [[char; NEW_MAX_LENGTH as usize]; NEW_MAX_LENGTH as usize],
) -> (u8, u8) {
    if position.1 > 0 && new_keypad[position.0 as usize][position.1 as usize - 1] != ' ' {
        position.1 -= 1;
    }

    (position.0, position.1)
}
