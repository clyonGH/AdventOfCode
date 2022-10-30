use std::fs::File;
use std::io::prelude::*;

struct Item {
    cost: i16,
    damage: i16,
    armor: i16,
}

fn main() {
    let mut file = File::open("inputs/input_2015_21.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // setting all the data
    let boss_data: Vec<i16> = data
        .lines()
        .map(|line| {
            let stat: Vec<&str> = line.split(": ").collect();
            stat[1].parse::<i16>().unwrap()
        })
        .collect();
    let boss: (i16, i16, i16) = (boss_data[0], boss_data[1], boss_data[2]);
    let all_weapons: [Item; 5] = set_weapons();
    let all_armor: [Item; 6] = set_armor();
    let all_rings: [Item; 7] = set_rings();

    // testing all the options
    let mut lowest_cost: i16 = 1000;
    let mut highest_cost: i16 = 0;
    let mut winning_player_stats: (i16, i16) = (0, 0);
    let mut losing_player_stats: (i16, i16) = (0, 0);
    all_weapons.iter().for_each(|weapon| {
        all_armor.iter().for_each(|armor| {
            let mut available_rings = [2, 1, 1, 1, 1, 1, 1];
            all_rings
                .iter()
                .enumerate()
                .for_each(|(first_index, first_ring)| {
                    available_rings[first_index] = available_rings[first_index] - 1;

                    all_rings
                        .iter()
                        .enumerate()
                        .for_each(|(second_index, second_ring)| {
                            if available_rings[second_index] > 0 {
                                let total_damage =
                                    weapon.damage + first_ring.damage + second_ring.damage;
                                let total_armor =
                                    armor.armor + first_ring.armor + second_ring.armor;
                                let total_cost =
                                    weapon.cost + armor.cost + first_ring.cost + second_ring.cost;

                                if fight_boss((100, total_damage, total_armor), boss) {
                                    if total_cost < lowest_cost {
                                        lowest_cost = total_cost;
                                        winning_player_stats = (total_damage, total_armor);
                                    }
                                } else {
                                    if total_cost > highest_cost {
                                        highest_cost = total_cost;
                                        losing_player_stats = (total_damage, total_armor);
                                    }
                                }
                            }
                        });
                });
        });
    });

    // first part
    println!("the least amount of gold you can spend and still win the fight is {lowest_cost} with {} damage and {} armor", winning_player_stats.0, winning_player_stats.1);

    // second part
    println!("the most amount of gold you can spend and still lose the fight is {highest_cost} with {} damage and {} armor", losing_player_stats.0, losing_player_stats.1);
}

fn fight_boss(player: (i16, i16, i16), boss: (i16, i16, i16)) -> bool {
    let mut player_hp = player.0;
    let mut boss_hp = boss.0;
    loop {
        if player.1 > boss.2 {
            boss_hp = boss_hp + boss.2 - player.1;
        } else {
            boss_hp = boss_hp - 1;
        }

        if boss_hp <= 0 {
            return true;
        }

        if boss.1 > player.2 {
            player_hp = player_hp + player.2 - boss.1;
        } else {
            player_hp = player_hp - 1;
        }

        if player_hp <= 0 {
            return false;
        }
    }
}

fn set_weapons() -> [Item; 5] {
    [
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ]
}

fn set_armor() -> [Item; 6] {
    [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ]
}

fn set_rings() -> [Item; 7] {
    [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ]
}
