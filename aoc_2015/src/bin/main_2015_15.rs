use std::fs::File;
use std::io::prelude::*;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let mut file = File::open("inputs/input_2015_15.txt").expect("Error: File not found");
    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let all_ingredients = data
        .lines()
        .map(|line| {
            let split_data: Vec<&str> = line.split(' ').collect();
            Ingredient {
                capacity: split_data[2]
                    .split_at(split_data[2].len() - 1)
                    .0
                    .parse::<i32>()
                    .unwrap(),
                durability: split_data[4]
                    .split_at(split_data[4].len() - 1)
                    .0
                    .parse::<i32>()
                    .unwrap(),
                flavour: split_data[6]
                    .split_at(split_data[6].len() - 1)
                    .0
                    .parse::<i32>()
                    .unwrap(),
                texture: split_data[8]
                    .split_at(split_data[8].len() - 1)
                    .0
                    .parse::<i32>()
                    .unwrap(),
                calories: split_data[10].parse::<i32>().unwrap(),
            }
        })
        .collect();

    dispatch_teaspoons(all_ingredients, 100);
}

fn dispatch_teaspoons(all_ingredients: Vec<Ingredient>, nb_teaspoons: i32) {
    let mut max_score: u64 = 0;
    let mut max_score_cal: u64 = 0;
    for sprinkles_ts in 0..=nb_teaspoons {
        for peanut_butter_ts in 0..=nb_teaspoons - sprinkles_ts {
            for frosting_ts in 0..=nb_teaspoons - sprinkles_ts - peanut_butter_ts {
                let sugar_ts = nb_teaspoons - sprinkles_ts - peanut_butter_ts - frosting_ts;
                let all_teaspoons: [i32; 4] =
                    [sprinkles_ts, peanut_butter_ts, frosting_ts, sugar_ts];

                let mut total_capacity = 0;
                let mut total_durability = 0;
                let mut total_flavour = 0;
                let mut total_texture = 0;
                let mut total_calories = 0;
                for index in 0..all_ingredients.len() {
                    total_capacity += all_ingredients[index].capacity * all_teaspoons[index];
                    total_durability += all_ingredients[index].durability * all_teaspoons[index];
                    total_flavour += all_ingredients[index].flavour * all_teaspoons[index];
                    total_texture += all_ingredients[index].texture * all_teaspoons[index];
                    total_calories += all_ingredients[index].calories * all_teaspoons[index];
                }

                if total_capacity.is_negative() {
                    total_capacity = 0;
                }
                if total_durability.is_negative() {
                    total_durability = 0;
                }
                if total_flavour.is_negative() {
                    total_flavour = 0;
                }
                if total_texture.is_negative() {
                    total_texture = 0;
                }

                let total_score = total_capacity * total_durability * total_flavour * total_texture;
                if total_calories == 500 && total_score as u64 > max_score_cal {
                    max_score_cal = total_score as u64;
                }
                if total_score as u64 > max_score {
                    max_score = total_score as u64;
                }
            }
        }
    }

    // first part
    println!("the total score of the highest-scoring cookie is: {max_score}");

    // second part
    println!("the total score of the highest-scoring cookie is with 500 calories: {max_score_cal}");
}
