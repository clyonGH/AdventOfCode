use std::fs::File;
use std::io::prelude::*;

const ROCK_1: [[char; 4]; 4] = [
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    ['@', '@', '@', '@'],
];

const ROCK_2: [[char; 4]; 4] = [
    [' ', ' ', ' ', ' '],
    [' ', '@', ' ', ' '],
    ['@', '@', '@', ' '],
    [' ', '@', ' ', ' '],
];

const ROCK_3: [[char; 4]; 4] = [
    [' ', ' ', ' ', ' '],
    [' ', ' ', '@', ' '],
    [' ', ' ', '@', ' '],
    ['@', '@', '@', ' '],
];

const ROCK_4: [[char; 4]; 4] = [
    ['@', ' ', ' ', ' '],
    ['@', ' ', ' ', ' '],
    ['@', ' ', ' ', ' '],
    ['@', ' ', ' ', ' '],
];

const ROCK_5: [[char; 4]; 4] = [
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    ['@', '@', ' ', ' '],
    ['@', '@', ' ', ' '],
];

#[derive(Debug, Clone, Copy)]
struct RockShape {
    content: [[char; 4]; 4],
    height: usize,
    width: usize,
}

#[derive(Debug, Clone, Copy)]
// (x,y) corresponds to the position of the bottom left corner of the rock
struct Rock {
    x: usize,
    y: usize,
    shape: RockShape,
}

#[derive(Debug, Clone)]
struct Chamber {
    content: Vec<[char; 7]>,
    tower_size: usize,
    rock: Rock,
    i_rock: usize,
    nb_rocks: u128,
}

impl Chamber {
    fn move_rock_down(&mut self, all_rocks: [RockShape; 5]) {
        if self.rock.y != 0 {
            let mut can_be_moved: bool = true;
            for n in 0..self.rock.shape.width {
                for m in 0..self.rock.shape.height {
                    let row_below = self.content[self.rock.y - 1 + m];

                    match (
                        self.rock.shape.content[3 - m][n],
                        row_below[n + self.rock.x],
                    ) {
                        ('@', ' ') | (' ', _) => (), // can be moved down, and will be moved
                        _ => can_be_moved = false,   // cannot be moved down, so will go to rest
                    }
                }
            }

            // move or let to rest
            if can_be_moved {
                self.rock.y -= 1;
            } else {
                self.let_rock_go_to_rest(all_rocks);
            }
        } else {
            self.let_rock_go_to_rest(all_rocks);
        }
    }

    fn move_rock_left(&mut self) {
        if self.rock.x != 0 {
            let mut can_be_moved: bool = true;
            for n in 0..4 {
                for m in 0..self.rock.shape.width {
                    if self.rock.x - 1 + m < 7 {
                        let column_beside: [char; 4] = [
                            self.content[self.rock.y + 3][self.rock.x - 1 + m],
                            self.content[self.rock.y + 2][self.rock.x - 1 + m],
                            self.content[self.rock.y + 1][self.rock.x - 1 + m],
                            self.content[self.rock.y][self.rock.x - 1 + m],
                        ];

                        match (self.rock.shape.content[n][m], column_beside[n]) {
                            ('@', ' ') | (' ', _) => (), // can be moved left, and will be moved
                            _ => can_be_moved = false,   // cannot be moved left, so will go to rest
                        }
                    } else {
                        can_be_moved = false;
                    }
                }
            }

            // move or do nothing
            if can_be_moved {
                self.rock.x -= 1;
            }
        }
    }

    fn move_rock_right(&mut self) {
        let mut can_be_moved: bool = true;
        for n in 0..4 {
            for m in 0..self.rock.shape.width {
                if self.rock.x + self.rock.shape.width < 7 {
                    let column_beside: [char; 4] = [
                        self.content[self.rock.y + 3][self.rock.x + self.rock.shape.width - m],
                        self.content[self.rock.y + 2][self.rock.x + self.rock.shape.width - m],
                        self.content[self.rock.y + 1][self.rock.x + self.rock.shape.width - m],
                        self.content[self.rock.y][self.rock.x + self.rock.shape.width - m],
                    ];

                    match (
                        self.rock.shape.content[n][self.rock.shape.width - 1 - m],
                        column_beside[n],
                    ) {
                        ('@', ' ') | (' ', _) => (), // can be moved right, and will be moved
                        _ => can_be_moved = false,   // cannot be moved right, so will go to rest
                    }
                } else {
                    can_be_moved = false;
                }
            }
        }

        // move or do nothing
        if can_be_moved {
            self.rock.x += 1;
        }
    }

    fn let_rock_go_to_rest(&mut self, all_rocks: [RockShape; 5]) {
        // go to rest
        for n in 0..self.rock.shape.width {
            for m in 0..self.rock.shape.height {
                if self.rock.shape.content[3 - m][n] != ' ' {
                    self.content[self.rock.y + m][n + self.rock.x] =
                        self.rock.shape.content[3 - m][n];
                }
            }
        }

        // set new tower size
        self.tower_size = self.get_tower_size();

        // go to next rock shape
        self.i_rock += 1;
        if self.i_rock == all_rocks.len() {
            self.i_rock = 0;
        }

        // change current rock
        self.rock = Rock {
            x: 2,
            y: self.tower_size + 3,
            shape: all_rocks[self.i_rock],
        };

        // add rock count
        self.nb_rocks += 1;

        // add empty new rows for the new rock
        (self.content.len() - self.tower_size..=7)
            .for_each(|_| self.content.push([' ', ' ', ' ', ' ', ' ', ' ', ' ']));
    }

    fn get_tower_size(&self) -> usize {
        let mut added_units = 0;
        (self.tower_size..self.content.len()).for_each(|i| {
            if self.content[i].iter().any(|&c| c == '@') {
                added_units += 1;
            }
        });

        self.tower_size + added_units
    }
}

fn main() {
    let mut file = File::open("inputs/input_2022_17.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let jet_patterns: Vec<char> = data.chars().map(|c| c).collect();
    let mut i: usize = 0;
    let all_rocks: [RockShape; 5] = [
        RockShape {
            content: ROCK_1,
            height: 1,
            width: 4,
        },
        RockShape {
            content: ROCK_2,
            height: 3,
            width: 3,
        },
        RockShape {
            content: ROCK_3,
            height: 3,
            width: 3,
        },
        RockShape {
            content: ROCK_4,
            height: 4,
            width: 1,
        },
        RockShape {
            content: ROCK_5,
            height: 2,
            width: 2,
        },
    ];
    let first_rock: Rock = Rock {
        x: 2,
        y: 3,
        shape: all_rocks[0],
    };
    let mut chamber: Chamber = Chamber {
        content: vec![
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ],
        tower_size: 0,
        rock: first_rock,
        i_rock: 0,
        nb_rocks: 0,
    };

    // first part
    let mut first_part: bool = false;
    loop {
        if chamber.nb_rocks == 2022 && !first_part {
            first_part = true;
            println!(
                "after 2022 rocks have stopped falling the tower of rocks will be {:?} units tall",
                chamber.tower_size
            );
        }

        // continue for second part
        if chamber.nb_rocks == 10000 {
            break;
        }

        match jet_patterns[i] {
            '<' => chamber.move_rock_left(),
            '>' => chamber.move_rock_right(),
            _ => (),
        }
        i += 1;
        if i == jet_patterns.len() {
            i = 0;
        }

        chamber.move_rock_down(all_rocks);
    }

    // second part
    // find pattern
    let pattern_size = find_pattern(&chamber);
    let pattern_nb_rocks = get_nb_rocks(&chamber, pattern_size, &jet_patterns, i, all_rocks);

    // nb of time the pattern can be iterated - 5, which gives a margin
    let iter_pattern = (1000000000000 / pattern_nb_rocks) - 5;
    let mut new_chamber: Chamber = Chamber {
        content: vec![
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ],
        tower_size: 0,
        rock: first_rock,
        i_rock: 0,
        nb_rocks: 0,
    };

    let mut j = 0;
    loop {
        if new_chamber.nb_rocks == 1000000000000 - iter_pattern * pattern_nb_rocks {
            println!(
                "after 1,000,000,000,000 rocks have stopped falling the tower of rocks will be {:?} units tall",
                new_chamber.tower_size + iter_pattern as usize * pattern_size
            );
            break;
        }

        match jet_patterns[j] {
            '<' => new_chamber.move_rock_left(),
            '>' => new_chamber.move_rock_right(),
            _ => (),
        }
        j += 1;
        if j == jet_patterns.len() {
            j = 0;
        }

        new_chamber.move_rock_down(all_rocks);
    }
}

fn find_pattern(chamber: &Chamber) -> usize {
    let mut test_chamber = chamber.content.clone();
    // get 10 rows of the chamber and see if the pattern can be found
    let mut pattern: Vec<[char; 7]> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;

    test_chamber.reverse();
    (0..10).for_each(|_| {
        pattern.push(test_chamber.pop().unwrap());
    });
    test_chamber.reverse();
    loop {
        let i_find = test_chamber.windows(10).enumerate().any(|(k, ten_rows)| {
            i = k;
            ten_rows[0] == pattern[0]
                && ten_rows[1] == pattern[1]
                && ten_rows[2] == pattern[2]
                && ten_rows[3] == pattern[3]
                && ten_rows[4] == pattern[4]
                && ten_rows[5] == pattern[5]
                && ten_rows[6] == pattern[6]
                && ten_rows[7] == pattern[7]
                && ten_rows[8] == pattern[8]
                && ten_rows[9] == pattern[9]
        });

        if !i_find {
            test_chamber.reverse();
            pattern.remove(0);
            pattern.push(test_chamber.pop().unwrap());
            test_chamber.reverse();
        } else {
            let new_test_chamber = &test_chamber[i + 10..];

            let j_find = new_test_chamber
                .windows(10)
                .enumerate()
                .any(|(k, ten_rows)| {
                    j = k;
                    ten_rows[0] == pattern[0]
                        && ten_rows[1] == pattern[1]
                        && ten_rows[2] == pattern[2]
                        && ten_rows[3] == pattern[3]
                        && ten_rows[4] == pattern[4]
                        && ten_rows[5] == pattern[5]
                        && ten_rows[6] == pattern[6]
                        && ten_rows[7] == pattern[7]
                        && ten_rows[8] == pattern[8]
                        && ten_rows[9] == pattern[9]
                });

            if j_find {
                break;
            }
        }
    }

    j + 10
}

fn get_nb_rocks(
    chamber: &Chamber,
    pattern_size: usize,
    jet_patterns: &Vec<char>,
    i: usize,
    all_rocks: [RockShape; 5],
) -> u128 {
    let first_part_tower_size = chamber.tower_size;
    let mut new_chamber = chamber.clone();
    let mut new_i = i;
    loop {
        if new_chamber.tower_size >= first_part_tower_size + pattern_size {
            break;
        }

        match jet_patterns[new_i] {
            '<' => new_chamber.move_rock_left(),
            '>' => new_chamber.move_rock_right(),
            _ => (),
        }
        new_i += 1;
        if new_i == jet_patterns.len() {
            new_i = 0;
        }

        new_chamber.move_rock_down(all_rocks);
    }
    // nb of rocks per pattern
    new_chamber.nb_rocks - 10000
}
