use std::fs::File;
use std::io::prelude::*;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug)]
enum Instruction {
    Rectangle(u8, u8),
    RotateRow(u8, u8),
    RotateColumn(u8, u8),
}

fn main() {
    let mut file = File::open("inputs/input_2016_08.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_instr: Vec<Instruction> = data
        .lines()
        .filter_map(|line| {
            let instr: Vec<&str> = line.split(' ').collect();
            match (instr[0], instr[1]) {
                ("rect", _) => {
                    let indexes: Vec<u8> = instr[1]
                        .split('x')
                        .map(|index_str| index_str.parse::<u8>().unwrap())
                        .collect();
                    Some(Instruction::Rectangle(indexes[0], indexes[1]))
                }
                ("rotate", "row") => {
                    let param: Vec<&str> = instr[2].split('=').collect();
                    Some(Instruction::RotateRow(
                        param[1].parse().unwrap(),
                        instr[4].parse().unwrap(),
                    ))
                }
                ("rotate", "column") => {
                    let param: Vec<&str> = instr[2].split('=').collect();
                    Some(Instruction::RotateColumn(
                        param[1].parse().unwrap(),
                        instr[4].parse().unwrap(),
                    ))
                }
                _ => None,
            }
        })
        .collect();

    // first part
    let mut final_screen: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH] =
        [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];
    all_instr.iter().for_each(|instr| match instr {
        Instruction::Rectangle(wide, tall) => create_rectangle(&mut final_screen, *wide, *tall),
        Instruction::RotateRow(row, nb_shift) => rotate_row(&mut final_screen, *row, *nb_shift),
        Instruction::RotateColumn(column, nb_shift) => {
            rotate_column(&mut final_screen, *column, *nb_shift)
        }
    });

    let nb_pixels_lit: usize = final_screen
        .iter()
        .map(|column| column.iter().filter(|&&pixel| pixel).count())
        .sum();

    println!("{:?} pixels should be lit", nb_pixels_lit);

    // second part
    for row in 0..SCREEN_HEIGHT {
        for column in 0..SCREEN_WIDTH {
            if final_screen[column][row] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn create_rectangle(screen: &mut [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH], wide: u8, tall: u8) {
    (0..wide).for_each(|x| {
        (0..tall).for_each(|y| {
            screen[x as usize][y as usize] = true;
        });
    });
}

fn rotate_row(screen: &mut [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH], row: u8, nb_shift: u8) {
    let mut row_to_rotate: [bool; SCREEN_WIDTH] = [false; SCREEN_WIDTH];
    (0..screen.len()).for_each(|x| {
        row_to_rotate[x as usize] = screen[x as usize][row as usize];
    });

    row_to_rotate.rotate_right(nb_shift as usize);

    (0..screen.len()).for_each(|x| {
        screen[x as usize][row as usize] = row_to_rotate[x as usize];
    });
}

fn rotate_column(screen: &mut [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH], column: u8, nb_shift: u8) {
    screen[column as usize].rotate_right(nb_shift as usize);
}
